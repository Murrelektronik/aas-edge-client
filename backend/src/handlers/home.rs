use actix_web::web;
use serde_json::{json, Value};
// use tokio::sync::Mutex;
// use std::sync::Arc;
// use rocksdb::DB;

use crate::state::AppState;
use crate::functions::aas_interfaces;

pub async fn index(
    req: actix_web::HttpRequest,
    app_state: web::Data<AppState>,
) -> impl actix_web::Responder {
    // Extract connection info
    let conn_info = req.connection_info();
    let ip = conn_info.host();
    let scheme_str = conn_info.scheme();
    let protocol = if scheme_str.is_empty() {
        "http".to_string()
    } else {
        scheme_str.to_string()
    };

    let url = format!("{}://{}/", protocol, ip);

    // Read and parse asset info JSON
    let asset_info_path = "./static/asset_info.json";
    let asset_info = match std::fs::read_to_string(asset_info_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read asset info: {}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    };
    let asset_info_json: Value = match serde_json::from_str(&asset_info) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to parse asset info JSON: {}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    };

    // Fetch the submodels dictionary from RocksDB
    let _id_submodels_dictionary = format!("{}:submodels_dictionary", &app_state.aas_id_short);

    let submodels_dictionary_result =
        aas_interfaces::aas_find_one(_id_submodels_dictionary, app_state.rocksdb.clone()).await;
    let submodels_dictionary = match submodels_dictionary_result {
        Ok(dictionary) => dictionary,
        Err(e) => {
            let error_message = format!("Failed to get submodel dictionary: {}", e);
            return actix_web::HttpResponse::InternalServerError().body(error_message);
        }
    };

    // Build the links array from the submodels dictionary
    let mut links = Vec::new();
    if let Some(submodels_dict_map) = submodels_dictionary.as_object() {
        for (key, _) in submodels_dict_map.iter() {
            let link_object = json!({
                "href": format!("/submodels/{}", key),
                "rel": key,
                "method": "GET"
            });
            links.push(link_object);
        }
    }

    // Construct the JSON response
    let json_data = json!({
        "@context": "https://www.w3.org/2022/wot/td/v1.1",
        "id": url,
        "title": "LNI Edge Device", // TODO: update this if necessary
        "version": asset_info_json,
        "security": [
            "bearer_sc"
        ],
        "base": format!("{}api/", url),
        "links": links
    });

    // Return the response
    actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(json_data.to_string())
}