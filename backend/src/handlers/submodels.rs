// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)


use actix_web::{web::{self, Data, Path}, HttpResponse, Responder};
use mongodb::{bson::Document, Collection};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::{json, Value};
use chrono::Utc;
use futures::future::try_join_all;
use std::collections::HashMap;


use crate::functions::aas_interfaces;
use crate::state::AppState;

pub async fn get_submodels(
    submodels_collection_arc: Data<Arc<Mutex<Collection<Document>>>>,
    app_data : Data<AppState>
) -> impl Responder {
    // using get_ref() to get the reference to the inner data
    let submodels_collection = submodels_collection_arc.get_ref().clone();
    
    let submodels_dictionary_id = format!("{}:submodels_dictionary", &app_data.aas_id_short);
    let submodels_dictionary = match aas_interfaces::aas_find_one(submodels_dictionary_id, 
                                        submodels_collection.clone()).await{
        Ok(submodels_dictionary) => submodels_dictionary,
        Err(e) => return actix_web::HttpResponse::InternalServerError().body(format!("Error getting submodels dictionary: {}", e)),
    };

    let keys: Vec<String> = submodels_dictionary.keys().cloned().collect();

    let fetch_tasks = keys.into_iter().map(|key| {
        let collection_clone = submodels_collection.clone();
        let aas_id_short_clone = app_data.aas_id_short.clone();
        async move {
            aas_interfaces::get_submodel_database(
                collection_clone,
                &aas_id_short_clone,
                &key,
            )
            .await
            .map(|submodel| (key, submodel))
        }
    });

    match try_join_all(fetch_tasks).await {
        Ok(results) => {
            let submodels_map: HashMap<String, Document> = results.into_iter().collect();
            HttpResponse::Ok().json(submodels_map) // send the map as a JSON response
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error getting submodels: {}", e)),
    }
}

pub async fn get_submodel(
    submodels_collection_arc: Data<Arc<Mutex<Collection<Document>>>>,
    path: Path<String>,
    app_data : Data<AppState>
) -> impl Responder {
    let submodel_id = path.into_inner();
    
    // using get_ref() to get the reference to the inner data
    let submodels_collection = submodels_collection_arc.get_ref().clone();
    let aas_submodel = match aas_interfaces::get_submodel_database(submodels_collection, 
                                        &app_data.aas_id_short, 
                                        &submodel_id).await{
        Ok(aas_submodel) => aas_submodel,
        Err(e) => return actix_web::HttpResponse::InternalServerError().body(format!("Error getting submodel: {}", e)),
    };
    HttpResponse::Ok().json(aas_submodel)
}

pub async fn patch_submodel(
    submodels_collection_arc: Data<Arc<Mutex<Collection<Document>>>>,
    path: Path<String>,
    app_data : Data<AppState>,
    json: web::Json<Value>
) -> impl Responder {
    let submodel_id_short = path.into_inner();
    // Handle LastUpdate only for SystemInformation and NetworkConfiguration
    // To modify the `json` value, work with its inner `Value` directly
    let mut json = json.into_inner();

    // Check if key "LastUpdate" exists in the JSON object. If it does, update it with the current time
    if let Some(last_update) = json.get_mut("LastUpdate") {
        *last_update = json!(Utc::now().to_rfc3339());
    }
    
    // using get_ref() to get the reference to the inner data
    let submodels_collection = submodels_collection_arc.get_ref().clone();

    match aas_interfaces::patch_submodel_database(
        submodels_collection, 
        &app_data.aas_id_short, 
        &submodel_id_short, 
        &json).await{
            Ok(_) => (),
            Err(e) => return actix_web::HttpResponse::InternalServerError().body(format!("Error patching submodel in database: {}", e)),
    };

    let submodels_collection_2nd = submodels_collection_arc.get_ref().clone();

    match aas_interfaces::read_managed_device(
        submodels_collection_2nd.clone(), 
        &app_data.aas_id_short
    ).await{
        Ok(managed_device) => managed_device,
        Err(e) => return actix_web::HttpResponse::InternalServerError().body(format!("Error patching submodel to server: {}", e)),
    };
    
    match aas_interfaces::patch_submodel_server(
        submodels_collection_2nd.clone(), 
        &app_data.aas_id_short, 
        &submodel_id_short, 
        &app_data.aasx_server, 
        &app_data.aas_identifier, 
        &json
    ).await{
        Ok(_) => HttpResponse::Ok().body("Submodel patched successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error patching submodel to server: {}", e)),
    };

    HttpResponse::Ok().body("Submodel patched successfully")
}
