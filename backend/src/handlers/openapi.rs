// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)


use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_yaml::Value as YamlValue;
use std::fs::File;
use std::io::Read;

// Define a struct to deserialize the query parameters
#[derive(Deserialize)]
struct QueryParams {
    data_format: Option<String>,
}

pub async fn openapi_endpoint(req: HttpRequest) -> impl Responder {
    // Deserialize the query parameters
    let query_params = web::Query::<QueryParams>::from_query(req.query_string());
    
    if let Ok(params) = query_params {
        if params.data_format.as_deref() == Some("json") {
            // Path to your YAML file
            let path = "./static/openapi.yaml";
            let mut file = match File::open(path) {
                Ok(file) => file,
                Err(_) => return HttpResponse::InternalServerError().body("Cannot open openapi.yaml file"),
            };

            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_err() {
                return HttpResponse::InternalServerError().body("Failed to read openapi.yaml file");
            }

            // Convert YAML to JSON
            let yaml_data: YamlValue = match serde_yaml::from_str(&contents) {
                Ok(data) => data,
                Err(_) => return HttpResponse::InternalServerError().body("Failed to parse YAML"),
            };

            let json_data = match serde_json::to_string_pretty(&yaml_data) {
                Ok(json) => json,
                Err(_) => return HttpResponse::InternalServerError().body("Failed to convert YAML to JSON"),
            };

            return HttpResponse::Ok().content_type("application/json").body(json_data);
        }
    }
    
    // Serve the YAML file directly if `dataFormat` is not `json`
    let file = match File::open("./static/openapi.yaml") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_err() {
                return HttpResponse::InternalServerError().body("Failed to read openapi.yaml file");
            }
            contents
        },
        Err(_) => return HttpResponse::InternalServerError().body("Cannot open openapi.yaml file"),
    };

    HttpResponse::Ok().content_type("text/yaml").body(file)
}
