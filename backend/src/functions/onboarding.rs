use serde_json;
use reqwest;
use base64;
use anyhow;
use anyhow::Context;
use tokio;
use futures;
use chrono::Utc;
use actix_web::Error;
use rocksdb::DB;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::aas_interfaces;

async fn fetch_single_submodel(
    aasx_server_url: &str,
    aas_id_short: &str,
    submodel_uid: &str,
    rocksdb: Arc<Mutex<DB>>,
    submodels_dictionary: Arc<Mutex<serde_json::Map<String, Value>>>,
    _onboarding: bool,
) -> Result<(), actix_web::Error> {
    let submodel_id_short_url = format!(
        "{}submodels/{}",
        aasx_server_url,
        base64::encode_config(submodel_uid, base64::URL_SAFE_NO_PAD)
    );

    let submodel_value_url = format!(
        "{}submodels/{}/$value",
        aasx_server_url,
        base64::encode_config(submodel_uid, base64::URL_SAFE_NO_PAD)
    );

    let client = reqwest::Client::new();

    let response_value = client
        .get(&submodel_value_url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to fetch submodel value from URL: {}", submodel_value_url))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let response_id_short = client
        .get(&submodel_id_short_url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to fetch submodel id short from URL: {}", submodel_id_short_url))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if response_id_short.status().is_success() && response_value.status().is_success() {
        let body_id_short: Value = response_id_short
            .json()
            .await
            .with_context(|| "Failed to parse response body as JSON")
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let body_value: Value = response_value
            .json()
            .await
            .with_context(|| "Failed to parse response body as JSON")
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let submodel_id_short = body_id_short["idShort"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to extract idShort from response body"))
            .map_err(actix_web::error::ErrorInternalServerError)?;

        // Serialize the submodel value to store in RocksDB
        let serialized_value = serde_json::to_vec(&body_value)
            .with_context(|| "Failed to serialize submodel value")
            .map_err(actix_web::error::ErrorInternalServerError)?;

        // Store the submodel in RocksDB
        {
            let mut db = rocksdb.lock().await;
            db.put(format!("{}:{}", aas_id_short, submodel_id_short), serialized_value)
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }

        println!("Successfully replaced submodel: {}", submodel_id_short);

        // Update the submodels dictionary
        {
            let mut dictionary = submodels_dictionary.lock().await;
            dictionary.insert(
                submodel_id_short.to_string(),
                Value::String(submodel_uid.to_string()),
            );
        }
    } else {
        // Handle unsuccessful responses
        if !response_id_short.status().is_success() {
            let status_code = response_id_short.status();
            let response_body = response_id_short.text().await.unwrap_or_default();

            println!(
                "Failed to fetch URL {}. Status code: {}. Response body: {}",
                submodel_id_short_url, status_code, response_body
            );
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Failed to fetch URL {}. Status code: {}. Response body: {}",
                submodel_id_short_url, status_code, response_body
            )));
        } else {
            let status_code = response_value.status();
            let response_body = response_value.text().await.unwrap_or_default();

            println!(
                "Failed to fetch URL {}. Status code: {}. Response body: {}",
                submodel_value_url, status_code, response_body
            );
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Failed to fetch URL {}. Status code: {}. Response body: {}",
                submodel_value_url, status_code, response_body
            )));
        }
    }

    Ok(())
}


async fn fetch_all_submodels(
    aasx_server_url: &str,
    aas_id_short: &str,
    submodel_uids: Vec<String>,
    rocksdb: Arc<Mutex<DB>>,
    onboarding: bool,
) -> Result<(), actix_web::Error> {
    let submodels_dictionary = Arc::new(Mutex::new(serde_json::Map::new()));

    let fetch_tasks: Vec<_> = submodel_uids
        .into_iter()
        .map(|submodel_uid| {
            let rocksdb_clone = Arc::clone(&rocksdb);
            let submodels_dictionary_clone = Arc::clone(&submodels_dictionary);
            let aasx_server_url_clone = aasx_server_url.to_string();
            let aas_id_short_clone = aas_id_short.to_string();

            tokio::spawn(async move {
                if let Err(e) = fetch_single_submodel(
                    &aasx_server_url_clone,
                    &aas_id_short_clone,
                    &submodel_uid,
                    rocksdb_clone,
                    submodels_dictionary_clone,
                    onboarding,
                )
                .await
                {
                    eprintln!("Failed to fetch submodel: {}", e);
                }
            })
        })
        .collect();

    let _results = futures::future::join_all(fetch_tasks).await;

    // Store the submodels dictionary in RocksDB
    {
        let dictionary = submodels_dictionary.lock().await;
        let serialized_dictionary = serde_json::to_vec(&*dictionary)
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let mut db = rocksdb.lock().await;
        db.put(
            format!("{}:submodels_dictionary", aas_id_short),
            serialized_dictionary,
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    Ok(())
}

pub async fn edge_device_onboarding(
    aasx_server: &str,
    aas_uid: &str,
    aas_id_short: &str,
    rocksdb: Arc<Mutex<DB>>,
) -> Result<(), actix_web::Error> {
    let url: String = format!(
        "{}shells/{}",
        aasx_server,
        base64::encode_config(aas_uid, base64::URL_SAFE_NO_PAD)
    );

    println!("Fetching URL: {}", url);

    // Request Shell information from the Server
    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response = client
        .get(&url)
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if response.status().is_success() {
        let insert_data: Value = response
            .json()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        // Extract submodels ID
        let submodels_id: Vec<String> = extract_submodels_id(&insert_data)?;

        // Store the shell data in RocksDB
        {
            let serialized_data = serde_json::to_vec(&insert_data)
                .map_err(actix_web::error::ErrorInternalServerError)?;

            let mut db = rocksdb.lock().await;
            db.put(aas_id_short, serialized_data)
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }

        fetch_all_submodels(
            aasx_server,
            aas_id_short,
            submodels_id,
            rocksdb.clone(),
            true,
        )
        .await?;
    } else {
        println!("Failed to fetch URL. Status code: {}", response.status());
        return Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to fetch URL. Status code: {}",
            response.status()
        )));
    }

    collecting_thumbnail_image(aas_id_short, aasx_server, aas_uid).await?;

    onboarding_managed_device(aas_id_short, aasx_server, rocksdb.clone()).await?;

    Ok(())
}

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn collecting_thumbnail_image(
    aas_id_short: &str,   // A short ID for the AAS (Asset Administration Shell)
    aasx_server_url: &str,   // Base URL of the AASX server
    aas_uid: &str,           // UID of the AAS
) -> Result<(), Error> {
    // Construct the URL for the thumbnail image
    let url = format!(
        "{}shells/{}/asset-information/thumbnail",
        aasx_server_url,
        base64::encode_config(aas_uid, base64::URL_SAFE_NO_PAD) // Encode the AAS UID in a URL-safe manner
    );

    // Create a new HTTP client
    let client: reqwest::Client = reqwest::Client::new();

    // Send a GET request to the constructed URL
    let mut response = client
        .get(&url)
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?; // Map any request errors to Actix Web internal server errors

    // Ensure the request was successful and returned a status code of 200 OK
    if response.status().is_success() {
        // Ensure the directory for storing images exists
        tokio::fs::create_dir_all("./static/asset_images/")
            .await
            .map_err(actix_web::Error::from)?; // Convert any errors to Actix Web errors

        // Create a file to store the downloaded image
        let image_name = format!("./static/asset_images/{}.png", aas_id_short);
        let mut file = File::create(image_name)
            .await
            .map_err(actix_web::Error::from)?; // Convert any errors to Actix Web errors

        // Stream the bytes directly into the file
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?
        {
            file.write_all(&chunk)
                .await
                .map_err(actix_web::Error::from)?; // Write each chunk to the file, converting errors
        }

        println!("Successfully retrieved image");
        Ok(())
    } else {
        // If the response status is not 200 OK, return a message
        eprintln!("Failed to retrieve image");
        Ok(())
    }
}

async fn onboarding_managed_device(
    aas_id_short: &str,
    aasx_server_url: &str,
    rocksdb: Arc<Mutex<DB>>,
) -> Result<(), actix_web::Error> {
    match aas_interfaces::read_managed_device(rocksdb.clone(), aas_id_short).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to read managed device: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };

    let time_now = Utc::now();
    let submodel_id_short = "ManagedDevice";
    let json = serde_json::json!({
        "BoardingStatus": "ONBOARDED",
        "LastUpdate": time_now.to_rfc3339()
    });

    match aas_interfaces::patch_submodel_database(
        rocksdb.clone(),
        aas_id_short,
        submodel_id_short,
        &json,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "Failed Onboarding because of error by patch submodel to database: {}",
                e
            );
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };

    match aas_interfaces::patch_submodel_server(
        rocksdb.clone(),
        aas_id_short,
        submodel_id_short,
        aasx_server_url,
        &json,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "Failed Onboarding because of error by patch submodel to AAS Server: {}",
                e
            );
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };

    Ok(())
}

fn extract_submodels_id(data: &Value) -> Result<Vec<String>, actix_web::Error> {
    let mut filtered_values = Vec::new();

    // Access "submodels" field as an array
    if let Some(submodels) = data.get("submodels").and_then(|v| v.as_array()) {
        for submodel in submodels.iter() {
            // Access "keys" array within each submodel
            if let Some(keys) = submodel.get("keys").and_then(|v| v.as_array()) {
                if let Some(first_key) = keys.first() {
                    if let Some(value_str) = first_key.get("value").and_then(|v| v.as_str()) {
                        filtered_values.push(value_str.to_string());
                    }
                }
            }
        }
    }

    Ok(filtered_values)
}

// fn extract_submodels_id(data: &mongodb::bson::Document) -> Result<Vec<String>, actix_web::Error> {
//     let mut filtered_values = Vec::new();

//     // Access "submodels" field, defaulting to an empty array if not found
//     let submodels = data.get_array("submodels").unwrap_or(&vec![]);

//     for submodel in submodels {
//         // Attempt to cast each submodel as a Document to access its fields
//         if let Some(submodel_doc) = submodel.as_document() {
//             // Check if the submodel's type is "ModelReference"
//             if submodel_doc.get_str("type").unwrap_or_default() == "ModelReference" {
//                 // Extract the value from the first element in "keys", if any
//                 if let Ok(keys) = submodel_doc.get_array("keys") {
//                     if let Some(first_key) = keys.first() {
//                         if let Some(mongodb::bson::Bson::Document(key_doc)) = first_key.as_document() {
//                             if let Ok(value_str) = key_doc.get_str("value") {
//                                 filtered_values.push(value_str.to_string());
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(filtered_values)
// }

// fn convert_json_to_document(value: &serde_json::Value) -> Result<mongodb::bson::Document, actix_web::Error> {
//     // First, convert serde_json::Value to mongodb::bson::Bson
//     let bson_value = mongodb::bson::to_bson(value)
//         .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Error converting JSON to BSON: {}", e)))?;

//     // Then, try to convert mongodb::bson::Bson to mongodb::bson::Document
//     let document = mongodb::bson::from_bson::<mongodb::bson::Document>(bson_value)
//         .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Error converting BSON to Document: {}", e)))?;

//     Ok(document)
// }