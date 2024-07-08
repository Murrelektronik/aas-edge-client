// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)


use actix_web::{web::Data, HttpResponse, Error};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::state::AppState;

pub async fn get_picture(
    app_data: Data<AppState>
) -> Result<HttpResponse, Error> {
    // Get the reference to the inner data
    let aas_id_short = &app_data.aas_id_short;
    let base_path = "./static/asset_images/";
    
    let thumbnail_png = format!("{}.png", aas_id_short);
    // List of potential files in order of priority
    let file_names = vec!["product.svg", "product.png", &thumbnail_png];
    
    let mut contents = Vec::new();
    let mut content_type = "image/png"; // Default content type
    
    for file_name in file_names {
        let file_path = format!("{}{}", base_path, file_name);
        
        if let Ok(mut file) = File::open(&file_path).await {
            // Read file content
            if file.read_to_end(&mut contents).await.is_ok() {
                // Check the file extension to set the correct content type
                if file_name.ends_with(".svg") {
                    content_type = "image/svg+xml";
                }
                return Ok(HttpResponse::Ok().content_type(content_type).body(contents));
            }
        }
    }
    
    // If no file was found or read successfully
    Ok(HttpResponse::NotFound().body("Image not found"))
}
