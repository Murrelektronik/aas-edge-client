// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)


use actix_web::{web, App, HttpServer, middleware::Logger};
use mongodb;
use std::{env, sync::Arc};
use tokio::time::{self, Duration};
use actix_cors::Cors;
use tokio::sync::Mutex;

async fn init_mongodb(mongo_uri: String) -> (mongodb::Database, mongodb::Collection<mongodb::bson::Document>, mongodb::Collection<mongodb::bson::Document>) {
    // Parse the MongoDB connection string into client options, panicking on failure.
    let client_options = mongodb::options::ClientOptions::parse(&mongo_uri).await
        .expect("Failed to parse MongoDB URI into client options");

    // Attempt to create a MongoDB client with the specified options, panicking on failure.
    let client = mongodb::Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client with given options");

    // Perform a simple operation to ensure the MongoDB server is accessible.
    // This operation tries to list database names, forcing a connection to be established.
    // Panics if the MongoDB server is not accessible.
    client.list_database_names(None, None).await
        .expect("Failed to connect to MongoDB. Ensure MongoDB is running and accessible.");

    // Access the specific database.
    let db = client.database("aas_edge_database");

    // Access the specific collections.
    let shells_collection = db.collection::<mongodb::bson::Document>("shells");
    let submodels_collection = db.collection::<mongodb::bson::Document>("submodels");

    // Clean up old data in the collections.
    match shells_collection.delete_many(mongodb::bson::doc! {}, None).await {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to clean up shells collection: {}", e),
    }
    match submodels_collection.delete_many(mongodb::bson::doc! {}, None).await {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to clean up submodels collection: {}", e),
    }

    // Return the database and collections; if any of the above steps fail, the function will have already panicked.
    (db, shells_collection, submodels_collection)
}

mod handlers;
mod routes;
mod state;
mod models;
mod functions;

use functions::scheduler_task;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // GUIDE: set env var for logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    // GUIDE: add logger
    env_logger::init();

    // Fetch the environment variables
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let aas_id_short = env::var("AAS_IDSHORT").expect("AAS_IDSHORT must be set");
    let aas_identifier = env::var("AAS_IDENTIFIER").expect("AAS_IDENTIFIER must be set");
    let aasx_server = env::var("AASX_SERVER").expect("AASX_SERVER must be set");
    let device_name = env::var("DEVICE_NAME").expect("DEVICE_NAME must be set");
    let offboarding_time = env::var("OFFBOARDING_TIME").expect("OFFBOARDING_TIME must be set").parse::<i64>().expect("OFFBOARDING_TIME must be an integer");

    // Initialize MongoDB
    let (db, shells_collection, submodels_collection) = init_mongodb(mongo_uri.clone()).await;
        
    

    // Initialize AppState with all necessary data
    let app_state = web::Data::new(state::AppState {
        health_check_response: std::sync::Mutex::new(String::from("I'm OK!")),
        mongo_uri,
        aas_identifier,
        aas_id_short,
        aasx_server,
        device_name,
        offboarding_time,
    });

    // GUIDE: wrap the collections in an Arc<Mutex<>> to share them between threads
    let submodels_collection_arc = Arc::new(Mutex::new(submodels_collection));
    let shells_collection_arc = Arc::new(Mutex::new(shells_collection));
    
    // Onboard the device
    loop {
        let result = functions::onboarding::edge_device_onboarding(
            &app_state.aasx_server,
            &app_state.aas_identifier,
            &app_state.aas_id_short,
            shells_collection_arc.clone(),
            submodels_collection_arc.clone(),
        ).await;
    
        match result {
            Ok(_) => {
                println!("Device onboarded successfully!");
                break
            }, // Exit loop on success
            Err(_) => {
                eprintln!("Failed to onboard device. Retrying in 10 seconds...");
                time::sleep(Duration::from_secs(10)).await; // Wait for 10 seconds before retrying
            }
        }
    }

    scheduler_task::submodels_scheduler(app_state.clone(), submodels_collection_arc.clone()).await;
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // GUIDE: add logger middleware
            .wrap(Logger::default())
            .wrap(cors)
            // GUIDE: pass shared data to the app
            .app_data(app_state.clone())
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(shells_collection_arc.clone()))
            .app_data(web::Data::new(submodels_collection_arc.clone()))
            // GUIDE: Configure the routes
            .configure(routes::config) 
    })

    .bind("0.0.0.0:18000")?
    .run()
    .await
}

    // Run bash script
    // let script_output = run_bash_script("./scripts/aas_client/sysInfo.sh").await;
    // match script_output {
    //     Ok(output) => println!("Script output: {}", output),
    //     Err(e) => eprintln!("Script error: {}", e),
    // }
