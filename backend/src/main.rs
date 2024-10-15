use actix_web::{web, App, HttpServer, middleware::Logger};
use std::{env, sync::Arc};
use tokio::time::{self, Duration};
use actix_cors::Cors;
use tokio::sync::Mutex;
use rocksdb::{DB, Options};

async fn init_rocksdb(path: &str) -> Arc<Mutex<DB>> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    let db = DB::open(&opts, path).expect("Failed to open RocksDB");
    
    Arc::new(Mutex::new(db))
}

mod handlers;
mod routes;
mod state;
mod models;
mod functions;

use functions::scheduler_task;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set environment variables for logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Fetch environment variables
    let rocksdb_path = env::var("ROCKSDB_PATH").unwrap_or_else(|_| "rocksdb".to_string());
    let aas_id_short = env::var("AAS_IDSHORT").expect("AAS_IDSHORT must be set");
    let aas_identifier = env::var("AAS_IDENTIFIER").expect("AAS_IDENTIFIER must be set");
    let aasx_server = env::var("AASX_SERVER").expect("AASX_SERVER must be set");
    let device_name = env::var("DEVICE_NAME").expect("DEVICE_NAME must be set");
    let offboarding_time = env::var("OFFBOARDING_TIME").expect("OFFBOARDING_TIME must be set").parse::<i64>().expect("OFFBOARDING_TIME must be an integer");

    // Initialize RocksDB
    let rocksdb = init_rocksdb(&rocksdb_path).await;

    // Initialize AppState
    let app_state = web::Data::new(state::AppState {
        health_check_response: Mutex::new(String::from("I'm OK!")),
        rocksdb: rocksdb.clone(),
        aas_identifier,
        aas_id_short,
        aasx_server,
        device_name,
        offboarding_time,
    });

    // Onboard the device
    loop {
        let result = functions::onboarding::edge_device_onboarding(
            &app_state.aasx_server,
            &app_state.aas_identifier,
            &app_state.aas_id_short,
            rocksdb.clone(),
        ).await;

        match result {
            Ok(_) => {
                println!("Device onboarded successfully!");
                break; // Exit loop on success
            },
            Err(err) => {
                eprintln!("Failed to onboard device: {}. \nRetrying in 10 seconds...", err);
                time::sleep(Duration::from_secs(10)).await; // Wait for 10 seconds before retrying
            }
        }
    }

    // only after is onboarded

    scheduler_task::submodels_scheduler(app_state.clone(), rocksdb.clone()).await;

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .app_data(web::Data::new(rocksdb.clone()))
            .configure(routes::config) // Setup routes
    })
    .bind("0.0.0.0:18000")?
    .run()
    .await
}