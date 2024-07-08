// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)

use anyhow::Context;
use mongodb::{
    bson::{ doc, Bson, Document},
    Collection,
    options::UpdateOptions,
};
use tokio::sync::Mutex;
use std::sync::Arc;
use serde_json::Value;
use reqwest;

// use serde::{Serialize, Deserialize};

// Adjusted to return a Result<Value, String> to better handle success and error states.
pub async fn aas_find_one(
    _id: String, // composite ID for the submodel using AAS ID and submodel ID
    submodels_collection_arc: Arc<Mutex<Collection<Document>>>
) 
    -> Result<Document, String> {
    let submodels_collection_lock = submodels_collection_arc.lock().await;
    // println!("Finding document with id: {}", _id);
    let filter = doc! { "_id": &_id };

    match submodels_collection_lock.find_one(filter, None).await {
        Ok(Some(document)) => {
            // Optionally remove the _id field from the document if not needed
            let mut document = document.clone(); // Clone if you need to modify the document.
            document.remove("_id");
            Ok(document)
        },
        Ok(None) => Err("Document not found".into()),
        Err(e) => Err(format!("Error finding document: {}", e)),
    }
}

pub async fn aas_update_one(
    _id: String, // composite ID for the submodel using AAS ID and submodel ID
    submodels_collection: Arc<Mutex<Collection<Document>>>, 
    new_document: Document, upsert: bool) // Document to be upserted
    -> Result<String, String> 
{
    
    let filter = doc! { "_id": _id };
    let options = UpdateOptions::builder().upsert(upsert).build();

    let submodels_collection_lock = submodels_collection.lock().await;
    // GUIDE: Use the '$set' operator for the update, which requires modifying the document structure
    let update = doc! { "$set": new_document };

    // Perform the update operation
    match submodels_collection_lock.update_one(filter, update, options).await {
        Ok(update_result) => {
            if let Some(upserted_id) = update_result.upserted_id {
                Ok(format!("Document upserted with id: {:?}", upserted_id))
            } else {
                Ok("Document updated successfully".into())
            }
        },
        Err(e) => Err(format!("Error upserting document: {}", e)),
    }
}

pub async fn get_submodel_database(
    submodels_collection_arc: std::sync::Arc<tokio::sync::Mutex<mongodb::Collection<mongodb::bson::Document>>>,
    aas_id_short: &str,
    submodel_id_short: &str
) -> Result<mongodb::bson::Document, String> {
    
    let _id_submodel = format!("{}:{}", aas_id_short, submodel_id_short);

    let aas_submodel_result = aas_find_one(_id_submodel, submodels_collection_arc.clone()).await;
    let aas_submodel = match aas_submodel_result {
        Ok(aas_submodel) => aas_submodel,
        Err(e) => return Err(format!("Error getting submodel: {}", e)),
    };

    Ok(aas_submodel)
}

pub async fn patch_submodel_database(
    submodels_collection_arc: Arc<Mutex<Collection<Document>>>, // Arc and Mutex for thread-safe shared access to the MongoDB collection
    aas_id_short: &str,    // Short ID for the AAS (Asset Administration Shell)
    submodel_id_short: &str,  // Short ID for the submodel
    json: &Value  // JSON data to be patched into the submodel
) -> Result<String, String> {
    
    // Clone the Arc to get a second reference to the collection
    let second_submodels_collection_arc = submodels_collection_arc.clone();
    // Create a composite ID for the submodel using AAS ID and submodel ID
    let _id_submodel = format!("{}:{}", aas_id_short, submodel_id_short);

    // Retrieve the existing submodel document from the database
    let aas_submodel_result = aas_find_one(_id_submodel.clone(), submodels_collection_arc.clone()).await;
    let aas_submodel = match aas_submodel_result {
        Ok(aas_submodel) => aas_submodel,
        Err(e) => return Err(format!("Error getting submodel: {}", e)),  // Return an error if submodel retrieval fails
    };

    // Parse the JSON request body into a BSON document
    let mut patch_document: Document = match mongodb::bson::to_document(&json) {
        Ok(document) => document,
        Err(e) => return Err(format!("Error parsing request body: {}", e)),  // Return an error if parsing fails
    };

    // Merge the existing submodel document with the patch document
    let merged_doc = merge_documents(&aas_submodel, &mut patch_document);

    // Update the submodel document in the database with the merged document
    let update_result = aas_update_one(_id_submodel, second_submodels_collection_arc.clone(), merged_doc, false).await;
    match update_result {
        Ok(message) => Ok(message),  // Return a success message if the update is successful
        Err(e) => Err(format!("Error patching submodel: {}", e)),  // Return an error if the update fails
    }
}

fn merge_documents(old_doc: &Document, new_doc: &Document) -> Document {
    let mut merged_doc = old_doc.clone(); // Clone old_doc to preserve its structure

    // Iterate over old_doc to preserve its keys and structure
    for (key, old_value) in old_doc.iter() {
        // Check if new_doc has a value for the current key
        if let Some(new_value) = new_doc.get(key) {
            // Deep merge if both values are documents
            if let (Bson::Document(old_subdoc), Bson::Document(new_subdoc)) = (old_value, new_value) {
                let merged_subdoc = merge_documents(old_subdoc, new_subdoc);
                merged_doc.insert(key.clone(), Bson::Document(merged_subdoc));
            } else {
                // Update with new value if not a sub-document
                merged_doc.insert(key.clone(), new_value.clone());
            }
        }
        // If new_doc does not have the current key, old_value remains unchanged
    }

    merged_doc
}


pub async fn patch_submodel_server(
    submodels_collection_arc: Arc<Mutex<Collection<Document>>>, // Arc and Mutex for thread-safe shared access to the MongoDB collection
    aas_id_short: &str,    // Short ID for the AAS (Asset Administration Shell)
    submodel_id_short: &str,  // Short ID for the submodel
    aasx_server_url: &str,    // Base URL of the AASX server
    aas_uid: &str,    // UID of the AAS
    json: &Value  // JSON data to be patched into the submodel
) -> Result<String, String> {
    // Create a composite ID for the submodel using AAS ID and submodel ID
    let _id_submodel = format!("{}:{}", aas_id_short, submodel_id_short);

    // Retrieve the existing submodel document from the database
    let submodels_collection = submodels_collection_arc.clone();
    let aas_submodel = aas_find_one(_id_submodel, submodels_collection_arc.clone()).await
        .map_err(|e| format!("Error getting submodel: {}", e))?;

    // Parse the JSON request body into a BSON document
    let mut patch_document: Document = mongodb::bson::to_document(&json)
        .map_err(|e| format!("Error parsing request body: {}", e))?;

    // Merge the existing submodel document with the patch document
    let merged_doc = merge_documents(&aas_submodel, &mut patch_document);
    
    // Retrieve the submodels dictionary from the database
    let submodels_dictionary = aas_find_one(format!("{}:submodels_dictionary", aas_id_short), submodels_collection.clone()).await
        .map_err(|e| format!("Error getting submodels dictionary: {}", e))?;

    // Extract the submodel UID from the dictionary
    let submodel_uid = submodels_dictionary.get_str(submodel_id_short)
        .map_err(|_| "Submodel not found in dictionary".to_string())?;

    // Create a new HTTP client
    let client = reqwest::Client::new();
    // Construct the URL for the submodel value endpoint
    let url = format!(
        "{}shells/{}/submodels/{}/$value",
        aasx_server_url,
        base64::encode_config(aas_uid, base64::URL_SAFE_NO_PAD),
        base64::encode_config(submodel_uid, base64::URL_SAFE_NO_PAD),
    );

    // Send a PATCH request to the submodel value endpoint
    let response = client.patch(&url)
        .json(&merged_doc)
        .send()
        .await
        .map_err(|e| format!("Error sending patch request: {}", e))?;

    // Check the response status code and return appropriate message
    match response.status() {
        reqwest::StatusCode::NO_CONTENT => Ok("Submodel patched successfully".into()),  // Return success message if status is 204 No Content
        _ => Err(response.text().await.unwrap_or_else(|_| "Unknown error".into())),  // Return error message if status is not 204
    }
}


pub async fn fetch_single_submodel_from_server(
    aasx_server_url: &str,
    aas_id_short: &str,
    aas_uid: &str,
    submodel_id_short: &str,
    submodels_collection_arc: Arc<Mutex<Collection<Document>>>
) -> Result<(), String> {
    // let submodels_collection_lock = submodels_collection_arc.lock().await;
    let submodels_dictionary = aas_find_one(format!("{}:submodels_dictionary", aas_id_short), 
                                                        submodels_collection_arc.clone()).await;
    
    let submodel_uid = match submodels_dictionary {
        Ok(submodels_dictionary) => {
            match submodels_dictionary.get(submodel_id_short) {
                Some(Bson::String(submodel_uid_str)) => submodel_uid_str.to_owned(), // Convert Bson::String to Rust String
                _ => return Err("Submodel not found in dictionary".into()), // Handle both None and non-string cases
            }
        },
        Err(e) => return Err(format!("Error getting submodels dictionary: {}", e)),
    };

    let client = reqwest::Client::new();
    let submodel_value_url = format!(
        "{}/shells/{}/submodels/{}/$value",
        aasx_server_url,
        base64::encode_config(aas_uid, base64::URL_SAFE_NO_PAD),
        base64::encode_config(submodel_uid, base64::URL_SAFE_NO_PAD),
    );

    let response = client.get(&submodel_value_url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to fetch submodel value from URL: {}", submodel_value_url))
        .map_err(|e| format!("Error sending GET request: {}", e))?;

    if response.status().is_success() {
        let body_value: Value = response.json().await
            .with_context(|| "Failed to parse JSON response")
            .map_err(|e| format!("Error parsing JSON response: {}", e))?;
        let bson_value = mongodb::bson::to_bson(&body_value)
            .with_context(|| "Failed to convert JSON to BSON")
            .map_err(|e| format!("Error converting JSON to BSON: {}", e))?;

        if let mongodb::bson::Bson::Document(document) = bson_value {
            let submodels_collection_lock = submodels_collection_arc.lock().await;
            submodels_collection_lock.replace_one(
                mongodb::bson::doc! { "_id": format!("{}:{}", aas_id_short, submodel_id_short) },
                document,
                mongodb::options::ReplaceOptions::builder().upsert(false).build(),
            ).await
            .with_context(|| "Failed to replace submodel in the database")
            .map_err(|e| format!("Error replacing submodel in the database: {}", e))?;

            println!("Successfully replaced submodel: {}", submodel_id_short)
        } else {
            return Err("Response body is not a BSON document".into())
        }


    } else {
        return Err(format!("Error fetching submodel: {:?}", response))
    }
    Ok(())
    
}

pub async fn read_managed_device(
    submodels_collection_arc: Arc<Mutex<Collection<Document>>>,
    aas_id_short: &str
) -> Result<Document, String> {
    let table_id = format!("{}:{}", aas_id_short, "ManagedDevice");
    
    let managed_device = aas_find_one(table_id, submodels_collection_arc.clone()).await;
    match managed_device {
        Ok(managed_device) => Ok(managed_device),
        Err(e) => Err(format!("Failed to find managed device: {}", e)),
    }
}
