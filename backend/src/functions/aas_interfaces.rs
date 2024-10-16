use anyhow::Context;
use rocksdb::DB;
use serde_json::Value;
use tokio::sync::Mutex;
use std::sync::Arc;
use reqwest;

// use serde::{Serialize, Deserialize};

use crate::functions::transform_value_submodel::merge_submodel_value_to_submodel;

// Find one document in RocksDB based on the _id
pub async fn aas_find_one(
    _id: String, // composite ID for the submodel using AAS ID and submodel ID
    rocksdb: Arc<Mutex<DB>>, // RocksDB database instance
) -> Result<Value, String> {
    let db = rocksdb.lock().await;

    // Fetch the value from RocksDB based on the key (_id)
    match db.get(_id.clone()) {
        Ok(Some(data)) => {
            // Deserialize the stored data into a `serde_json::Value` object
            let value: Value = serde_json::from_slice(&data)
                .map_err(|e| format!("Error deserializing document: {}", e))?;
            Ok(value)
        }
        Ok(None) => Err(format!("Document with id '{}' not found", _id)),
        Err(e) => Err(format!("Error finding document: {}", e)),
    }
}

// Update or upsert one document in RocksDB
pub async fn aas_update_one(
    _id: String, // composite ID for the submodel using AAS ID and submodel ID
    rocksdb: Arc<Mutex<DB>>, // RocksDB database instance
    new_document: Value, // Document to be upserted
    _upsert: bool, // Ignored since RocksDB overwrites by default
) -> Result<String, String> {
    let db = rocksdb.lock().await;

    // Serialize the JSON document into bytes
    let serialized_doc = serde_json::to_vec(&new_document)
        .map_err(|e| format!("Error serializing document: {}", e))?;

    // Insert or update the document in RocksDB
    db.put(_id.clone(), serialized_doc)
        .map_err(|e| format!("Error upserting document: {}", e))?;

    Ok(format!("Document updated successfully with id: {}", _id))
}

pub async fn get_submodel_database(
    rocksdb: Arc<Mutex<DB>>, // Arc and Mutex for thread-safe shared access to the RocksDB instance
    aas_id_short: &str,    // Short ID for the AAS (Asset Administration Shell)
    submodel_id_short: &str,  // Short ID for the submodel
) -> Result<Value, String> {
    // Create a composite ID for the submodel using AAS ID and submodel ID
    let _id_submodel = format!("{}:{}", aas_id_short, submodel_id_short);

    // Retrieve the submodel from RocksDB
    let aas_submodel_result = aas_find_one(_id_submodel, rocksdb.clone()).await;
    
    // Handle success or error in fetching the submodel
    let aas_submodel = match aas_submodel_result {
        Ok(aas_submodel) => aas_submodel,
        Err(e) => return Err(format!("Error getting submodel: {}", e)),
    };

    Ok(aas_submodel)
}
// Patch and merge submodel in RocksDB
pub async fn patch_submodel_database(
    rocksdb: Arc<Mutex<DB>>, // Arc and Mutex for thread-safe shared access to RocksDB
    aas_id_short: &str,    // Short ID for the AAS (Asset Administration Shell)
    submodel_id_short: &str,  // Short ID for the submodel
    submodel_value: &Value  // JSON data to be patched into the submodel
) -> Result<String, String> {
    // Create a composite ID for the submodel using AAS ID and submodel ID
    let _id_submodel = format!("{}:{}", aas_id_short, submodel_id_short);

    // Retrieve the existing submodel document from the database
    let existing_submodel_result = aas_find_one(_id_submodel.clone(), rocksdb.clone()).await;
    let existing_submodel = match existing_submodel_result {
        Ok(submodel) => submodel,
        Err(_) => submodel_value.clone(),  // If no existing submodel, start with the new JSON
    };

    // Merge the existing submodel with the new patch document
    let merged_doc = merge_submodel_value_to_submodel(existing_submodel, submodel_value.clone());

    // Update the submodel in RocksDB
    let update_result = aas_update_one(_id_submodel, rocksdb.clone(), merged_doc, true).await;

    update_result
}


// Merge two documents recursively
fn merge_documents(old_doc: &Value, new_doc: &Value) -> Value {
    let mut merged_doc = old_doc.clone(); // Start with the old document

    // Iterate over the keys in the new document and update or insert into the old document
    if let Value::Object(ref old_map) = old_doc {
        if let Value::Object(ref new_map) = new_doc {
            for (key, new_value) in new_map {
                // If both old and new values are objects, recursively merge them
                if let Some(old_value) = old_map.get(key) {
                    if let (Value::Object(_), Value::Object(_)) = (old_value, new_value) {
                        let merged_subdoc = merge_documents(old_value, new_value);
                        merged_doc[key] = merged_subdoc;
                    } else {
                        merged_doc[key] = new_value.clone(); // Overwrite if not both objects
                    }
                } else {
                    merged_doc[key] = new_value.clone(); // Add new key-value pairs
                }
            }
        }
    }
    merged_doc
}


pub async fn patch_submodel_server(
    rocksdb: Arc<Mutex<DB>>, // RocksDB instance for thread-safe shared access
    aas_id_short: &str,    // Short ID for the AAS (Asset Administration Shell)
    submodel_id_short: &str,  // Short ID for the submodel
    aasx_server_url: &str,    // Base URL of the AASX server
    submodel_value: &Value  // JSON data to be patched into the submodel
) -> Result<String, String> {
    // Create a composite ID for the submodel using AAS ID and submodel ID
    let _id_submodel = format!("{}:{}", aas_id_short, submodel_id_short);

    // Retrieve the existing submodel document from the RocksDB database
    let existing_submodel = aas_find_one(_id_submodel.clone(), rocksdb.clone()).await
        .map_err(|e| format!("Error getting submodel: {}", e))?;

    // Merge the existing submodel document with the patch document
    let merged_doc = merge_submodel_value_to_submodel(existing_submodel, submodel_value.clone());

    // Retrieve the submodels dictionary from RocksDB
    let submodels_dictionary = aas_find_one(format!("{}:submodels_dictionary", aas_id_short), rocksdb.clone()).await
        .map_err(|e| format!("Error getting submodels dictionary: {}", e))?;

    // Extract the submodel UID from the dictionary
    let submodel_uid = submodels_dictionary.get(submodel_id_short)
        .and_then(|val| val.as_str())
        .ok_or_else(|| "Submodel not found in dictionary".to_string())?;

    // Create a new HTTP client
    let client = reqwest::Client::new();
    // Construct the URL for the submodel value endpoint
    let url = format!(
        "{}submodels/{}",
        aasx_server_url,
        base64::encode_config(submodel_uid, base64::URL_SAFE_NO_PAD),
    );

    // Send a PATCH request to the submodel value endpoint
    let response = client.put(&url)
        .json(&merged_doc)
        .send()
        .await
        .map_err(|e| format!("Error sending put request: {}", e))?;

    // Check the response status code and return appropriate message
    match response.status() {
        reqwest::StatusCode::NO_CONTENT => Ok("Submodel putted successfully".into()),  // Return success message if status is 204 No Content
        _ => Err(response.text().await.unwrap_or_else(|_| "Unknown error".into())),  // Return error message if status is not 204
    }
}


// Fetch a single submodel from the AASX server
pub async fn fetch_single_submodel_from_server(
    aasx_server_url: &str,
    aas_id_short: &str,
    submodel_id_short: &str,
    rocksdb: Arc<Mutex<DB>>,
) -> Result<(), String> {
    
    let submodels_dictionary = aas_find_one(format!("{}:submodels_dictionary", aas_id_short), rocksdb.clone()).await;
    
    let submodel_uid = match submodels_dictionary {
        Ok(submodels_dictionary) => {
            match submodels_dictionary.get(submodel_id_short) {
                Some(Value::String(submodel_uid_str)) => submodel_uid_str.to_owned(), // Convert to String
                _ => return Err("Submodel not found in dictionary".into()), // Handle error
            }
        },
        Err(e) => return Err(format!("Error getting submodels dictionary: {}", e)),
    };

    let client = reqwest::Client::new();
    let submodel_url: String = format!(
        "{}submodels/{}",
        aasx_server_url,
        base64::encode_config(submodel_uid, base64::URL_SAFE_NO_PAD),
    );

    let response = client.get(&submodel_url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to fetch submodel value from URL: {}", submodel_url))
        .map_err(|e| format!("Error sending GET request: {}", e))?;

    if response.status().is_success() {
        let body_value: Value = response.json().await
            .with_context(|| "Failed to parse JSON response")
            .map_err(|e| format!("Error parsing JSON response: {}", e))?;

        let bson_value = serde_json::to_value(&body_value)
            .with_context(|| "Failed to convert JSON to BSON")
            .map_err(|e| format!("Error converting JSON to BSON: {}", e))?;

        if let Value::Object(document) = bson_value {
            let db = rocksdb.lock().await;
            db.put(
                format!("{}:{}", aas_id_short, submodel_id_short),
                serde_json::to_vec(&document).expect("Failed to serialize JSON")
            ).expect("Failed to replace submodel in RocksDB");

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
    rocksdb: Arc<Mutex<DB>>,
    aas_id_short: &str,
) -> Result<Value, String> {
    let table_id = format!("{}:{}", aas_id_short, "ManagedDevice");
    
    let managed_device = aas_find_one(table_id, rocksdb.clone()).await;
    match managed_device {
        Ok(managed_device) => Ok(managed_device),
        Err(e) => Err(format!("Failed to find managed device: {}", e)),
    }
}