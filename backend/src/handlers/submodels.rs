use actix_web::{web::{self, Data, Path}, HttpResponse, Responder};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use futures::future::try_join_all;
use rocksdb::DB;

use crate::functions::aas_interfaces;
use crate::state::AppState;

/// Handler to get all submodels.
pub async fn get_submodels(
    rocksdb: Data<Arc<Mutex<DB>>>,
    app_data: Data<AppState>,
) -> impl Responder {
    // Clone the RocksDB instance
    let rocksdb = rocksdb.get_ref().clone();

    // Construct the key for the submodels dictionary
    let submodels_dictionary_id = format!("{}:submodels_dictionary", &app_data.aas_id_short);

    // Fetch the submodels dictionary from RocksDB
    let submodels_dictionary = match aas_interfaces::aas_find_one(
        submodels_dictionary_id,
        rocksdb.clone(),
    )
    .await
    {
        Ok(submodels_dictionary) => submodels_dictionary,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error getting submodels dictionary: {}", e))
        }
    };

    // Extract the keys (submodel IDs) from the dictionary
    let keys: Vec<String> = submodels_dictionary
        .as_object()
        .map(|map| map.keys().cloned().collect())
        .unwrap_or_default();

    // Create asynchronous tasks to fetch each submodel
    let fetch_tasks = keys.into_iter().map(|key| {
        let rocksdb_clone = rocksdb.clone();
        let aas_id_short_clone = app_data.aas_id_short.clone();
        async move {
            aas_interfaces::get_submodel_database(
                rocksdb_clone,
                &aas_id_short_clone,
                &key,
            )
            .await
            .map(|submodel| (key, submodel))
        }
    });

    // Execute all fetch tasks concurrently
    match try_join_all(fetch_tasks).await {
        Ok(results) => {
            let submodels_map: HashMap<String, Value> = results.into_iter().collect();
            HttpResponse::Ok().json(submodels_map) // Send the map as a JSON response
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error getting submodels: {}", e)),
    }
}

/// Handler to get a single submodel.
pub async fn get_submodel(
    rocksdb: Data<Arc<Mutex<DB>>>,
    path: Path<String>,
    app_data: Data<AppState>,
) -> impl Responder {
    let submodel_id = path.into_inner();

    // Clone the RocksDB instance
    let rocksdb_clone = rocksdb.get_ref().clone();

    // Fetch the submodel from RocksDB
    let aas_submodel = match aas_interfaces::get_submodel_database(
        rocksdb_clone,
        &app_data.aas_id_short,
        &submodel_id,
    )
    .await
    {
        Ok(aas_submodel) => aas_submodel,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error getting submodel: {}", e))
        }
    };
    HttpResponse::Ok().json(aas_submodel)
}

/// Handler to patch (update) a submodel.
pub async fn patch_submodel(
    rocksdb: Data<Arc<Mutex<DB>>>,
    path: Path<String>,
    app_data: Data<AppState>,
    json: web::Json<Value>,
) -> impl Responder {
    let submodel_id_short = path.into_inner();
    // Convert the JSON payload into a mutable `Value`
    let mut json = json.into_inner();

    // Update the "LastUpdate" field if it exists
    if let Some(last_update) = json.get_mut("LastUpdate") {
        *last_update = json!(Utc::now().to_rfc3339());
    }

    // Clone the RocksDB instance
    let rocksdb_clone = rocksdb.get_ref().clone();

    // Patch the submodel in the local RocksDB database
    match aas_interfaces::patch_submodel_database(
        rocksdb_clone.clone(),
        &app_data.aas_id_short,
        &submodel_id_short,
        &json,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error patching submodel in database: {}", e))
        }
    };

    // Read the managed device information
    match aas_interfaces::read_managed_device(
        rocksdb_clone.clone(),
        &app_data.aas_id_short,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading managed device: {}", e))
        }
    };

    // Patch the submodel on the AAS server
    match aas_interfaces::patch_submodel_server(
        rocksdb_clone,
        &app_data.aas_id_short,
        &submodel_id_short,
        &app_data.aasx_server,
        &json,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Submodel patched successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error patching submodel to server: {}", e)),
    }
}
