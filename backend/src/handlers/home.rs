// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)

use actix_web::web;
use serde_json::Value;
use mongodb::{bson::{Bson, Document}, Collection};
use serde_json;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::state::AppState;
use crate::functions::aas_interfaces;

// GUIDE: parameter name of web::Data muss be the same as the one in main.rs (example app_state, submodels_collection_arc)
pub async fn index(
    req: actix_web::HttpRequest, 
    app_state: actix_web::web::Data<AppState>,
    submodels_collection_arc: web::Data<Arc<Mutex<Collection<Document>>>>
) -> impl actix_web::Responder {
    // Bind the connection info to a variable to extend its lifetime.
    let conn_info = req.connection_info();
    // Retrieve the client's IP address. If it's not available, use a default value.
    // let ip = conn_info.realip_remote_addr().unwrap_or("unknown");
    let ip = conn_info.host();

    // Retrieve the request scheme (protocol). If it's not available, default to "http".
    let scheme_str = conn_info.scheme(); // Assuming this is &str
    let protocol = if scheme_str.is_empty() { "http".to_string() } else { scheme_str.to_string() };
    
    let url = format!("{}://{}/", protocol, ip);

    let asset_info_path = format!("./static/asset_info.json");
    let asset_info = match std::fs::read_to_string(asset_info_path) {
        Ok(asset_info) => asset_info,
        Err(e) => {
            eprintln!("Failed to read asset info: {}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        },
    };
    let asset_info_json: Value = match serde_json::from_str(&asset_info) {
        Ok(asset_info_json) => asset_info_json,
        Err(e) => {
            eprintln!("Failed to parse asset info JSON: {}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        },
    };

    let version_bson = match mongodb::bson::to_bson(&asset_info_json) {
        Ok(bson_data) => bson_data,
        Err(e) => {
            eprintln!("Failed to convert asset info JSON to BSON: {}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        },
    };
    let _id_submodels_dictionary = format!("{}:submodels_dictionary", &app_state.aas_id_short);
    // println!("Submodels Dictionary ID: {}", _id_submodels_dictionary);
    
    let submodels_dictionary_result = aas_interfaces::aas_find_one(_id_submodels_dictionary, 
        submodels_collection_arc.get_ref().clone()).await;
    let submodels_dictionary = match submodels_dictionary_result {
        Ok(submodels_dictionary) => submodels_dictionary,
        Err(e) => {
            let error_message = format!("Failed to get submodel dictionary: {}", e);
            return actix_web::HttpResponse::InternalServerError().body(error_message);
        },
    };

    let mut links = Vec::new();
    for (key, _) in submodels_dictionary.iter() {
        let link_object = mongodb::bson::doc! {
            "href": format!("/submodels/{}", key),
            "rel": key,
            "method": "GET"
        };
        links.push(Bson::Document(link_object));
    }



    let bson_data = mongodb::bson::doc! {
        "@context": "https://www.w3.org/2022/wot/td/v1.1",
        "id": &url,
        "title": "LNI Edge Device", // TODO: change it later
        "version": version_bson,
        "security": [
            "bearer_sc"
        ],
        // to handle error "value borrowed here after move" use &url instead of url (or use url.clone()
        // In Rust, values can only have one owner at a time, and when you pass a value to another variable or function
        // without using a reference or cloning, the ownership is transferred (moved), and the original variable can no longer be used.
        "base": format!("{}api/", url.clone()),
        // "base": format!("{}", url.clone()),
        "links": links  
    };

    

    

    // let _id_submodels_dictionary = format!("{}:submodels_dictionary", &app_state.aas_id_short);
    // // println!("Submodels Dictionary ID: {}", _id_submodels_dictionary);
    
    // let submodels_dictionary_result = aas_interfaces::aas_find_one(_id_submodels_dictionary, 
    //     submodels_collection_arc.get_ref().clone()).await;
    // let submodels_dictionary = match submodels_dictionary_result {
    //     Ok(submodels_dictionary) => submodels_dictionary,
    //     Err(e) => {
    //         let error_message = format!("Failed to get submodel dictionary: {}", e);
    //         return actix_web::HttpResponse::InternalServerError().body(error_message);
    //     },
    // };

    // for (key, _) in submodels_dictionary.iter() {
    //     let link_object = mongodb::bson::doc! {
    //         "href": format!("/submodels/{}", key),
    //         "rel": key,
    //         "method": "GET"
    //     };
    //     bson_data.get_array_mut("links").unwrap().push(Bson::Document(link_object));
    // }

    // Convert BSON to a serde_json Value
    let json_data: serde_json::Value = match mongodb::bson::to_bson(&bson_data) {
        Ok(bson) => serde_json::to_value(bson).unwrap(),
        Err(_) => serde_json::json!({"error": "Failed to convert BSON to JSON"}),
    };

    actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(json_data.to_string()) // Convert Value to a String to set as the body
}
