use actix_web::web;
use chrono::{DateTime, Utc};
use clokwerk::{Scheduler, TimeUnits};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::{self, sync::Mutex};
use rocksdb::DB;

use crate::functions::{aas_interfaces, bash_command};
use crate::functions::transform_value_submodel::submodel_to_submodel_value;
use crate::state::AppState;

/// Parses a date-time string into a `DateTime<Utc>` object.
fn parse_date_time_from_string(date_time_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let date_time = DateTime::parse_from_rfc3339(date_time_str)?;
    Ok(date_time.with_timezone(&Utc))
}

/// Runs a bash script and returns the output as a JSON `Value`.
async fn run_script() -> Result<Value, anyhow::Error> {
    let script_output = bash_command::run_bash_script("./scripts/aas_client/sysInfo.sh").await?;
    let json: Value = serde_json::from_str(&script_output)?;
    Ok(json)
}

async fn update_submodel_database(
    rocksdb: Arc<Mutex<DB>>,
    aas_id_short: &str,
    submodel_id_short: &str,
    json: &Value,
) -> Result<(), anyhow::Error> {
    aas_interfaces::patch_submodel_database(rocksdb, aas_id_short, submodel_id_short, json)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to patch submodel to database: {}", e))?;
    Ok(())
}

async fn update_submodel_server(
    rocksdb: Arc<Mutex<DB>>,
    app_data: &AppState,
    submodel_id_short: &str,
    json: &Value,
) -> Result<(), anyhow::Error> {
    aas_interfaces::patch_submodel_server(
        rocksdb,
        &app_data.aas_id_short,
        submodel_id_short,
        &app_data.aasx_server,
        json,
    )
    .await
    .map_err(|e| anyhow::anyhow!("Failed to patch submodel to AAS server: {}", e))?;
    Ok(())
}

async fn handle_offboarding(
    rocksdb: Arc<Mutex<DB>>,
    app_data: &AppState,
    time_now: DateTime<Utc>,
) -> Result<(), anyhow::Error> {
    let submodel_id_short = "ManagedDevice";
    let json = json!({
        "BoardingStatus": "OFFBOARDED",
        "LastUpdate": time_now.to_rfc3339()
    });

    update_submodel_database(rocksdb.clone(), &app_data.aas_id_short, submodel_id_short, &json)
        .await?;
    update_submodel_server(rocksdb, app_data, submodel_id_short, &json).await?;

    Ok(())
}

async fn handle_onboarding(
    rocksdb: Arc<Mutex<DB>>,
    app_data: &AppState,
    time_now: DateTime<Utc>,
) -> Result<(), anyhow::Error> {
    let managed_device_json = json!({
        "BoardingStatus": "ONBOARDED",
        "LastUpdate": time_now.to_rfc3339()
    });

    update_submodel_database(
        rocksdb.clone(),
        &app_data.aas_id_short,
        "ManagedDevice",
        &managed_device_json,
    )
    .await?;
    update_submodel_server(
        rocksdb.clone(),
        app_data,
        "ManagedDevice",
        &managed_device_json,
    )
    .await?;

    let json = run_script().await?;
    update_submodel_server(rocksdb, app_data, "SystemInformation", &json).await?;

    Ok(())
}

/// Handles the ONBOARDED status by updating system information.
async fn handle_onboarded_status(
    rocksdb: Arc<Mutex<DB>>,
    app_data: &AppState,
    json: &Value,
) -> Result<(), anyhow::Error> {
    update_submodel_server(rocksdb, app_data, "SystemInformation", json).await?;
    Ok(())
}

async fn server_pushing(app_data: web::Data<AppState>, rocksdb: Arc<Mutex<DB>>) {
    let offboarding_time = app_data.offboarding_time;

    let json = match run_script().await {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to execute script: {}", e);
            return;
        }
    };

    if let Err(e) = update_submodel_database(
        rocksdb.clone(),
        &app_data.aas_id_short,
        "SystemInformation",
        &json,
    )
    .await
    {
        eprintln!("Failed to patch submodel to database: {}", e);
        return;
    }

    let managed_device_submodel = match aas_interfaces::read_managed_device(rocksdb.clone(), &app_data.aas_id_short).await {
        Ok(managed_device) => managed_device,
        Err(e) => {
            eprintln!("Failed to read managed device: {}", e);
            return;
        }
    };

    let managed_device_submodel_value = submodel_to_submodel_value(managed_device_submodel);
    let boarding_status = managed_device_submodel_value
        .get("BoardingStatus")
        .and_then(|status| status.as_str())
        .unwrap_or("UNKNOWN");
    let last_update_str = managed_device_submodel_value
        .get("LastUpdate")
        .and_then(|update| update.as_str())
        .unwrap_or("UNKNOWN");

    let last_update = match parse_date_time_from_string(last_update_str) {
        Ok(last_update) => last_update,
        Err(e) => {
            eprintln!("Failed to parse last update: {}", e);
            return;
        }
    };

    let time_now = Utc::now();

    match boarding_status {
        "OFFBOARDING_REQUESTED" => {
            if let Err(e) = handle_offboarding(rocksdb.clone(), &app_data, time_now).await {
                eprintln!("Failed to offboard: {}", e);
            }
        }
        "OFFBOARDED" if (time_now - last_update).num_seconds() < offboarding_time => (),
        "OFFBOARDED" if (time_now - last_update).num_seconds() >= offboarding_time => {
            if let Err(e) = handle_onboarding(rocksdb.clone(), &app_data, time_now).await {
                eprintln!("Failed to onboard: {}", e);
            }
        }
        "ONBOARDED" => {
            if let Err(e) = handle_onboarded_status(rocksdb.clone(), &app_data, &json).await {
                eprintln!("Failed to push system information to server: {}", e);
            }
        }
        _ => eprintln!("Invalid boarding status"),
    }
}

async fn server_polling(app_data: web::Data<AppState>, rocksdb: Arc<Mutex<DB>>) {
    let submodel_ids = vec!["ManagedDevice"]; // Add more submodel IDs if necessary

    for submodel_id_short in submodel_ids {
        match aas_interfaces::fetch_single_submodel_from_server(
            &app_data.aasx_server,
            &app_data.aas_id_short,
            &submodel_id_short,
            rocksdb.clone(),
        )
        .await
        {
            Ok(_) => println!(
                "Successfully fetched submodel {} from server",
                submodel_id_short
            ),
            Err(e) => eprintln!(
                "Failed to fetch submodel {} from server: {}",
                submodel_id_short, e
            ),
        }
    }
}

pub async fn submodels_scheduler(app_state: web::Data<AppState>, rocksdb: Arc<Mutex<DB>>) {
    let mut scheduler = Scheduler::with_tz(chrono::Utc);

    let app_state_clone_push = app_state.clone();
    let rocksdb_clone_push = rocksdb.clone();

    scheduler.every(5.seconds()).run(move || {
        let task = server_pushing(app_state_clone_push.clone(), rocksdb_clone_push.clone());
        tokio::spawn(task);
    });

    let app_state_clone_poll = app_state.clone();
    let rocksdb_clone_poll = rocksdb.clone();

    scheduler.every(10.seconds()).run(move || {
        let task = server_polling(app_state_clone_poll.clone(), rocksdb_clone_poll.clone());
        tokio::spawn(task);
    });

    tokio::spawn(async move {
        loop {
            scheduler.run_pending();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}

// async fn server_polling(
//     app_data: web::Data<AppState>,
//     submodels_collection_arc: Arc<Mutex<Collection<Document>>>,
// ) {
//     let submodels_collection = submodels_collection_arc.clone();
//     let submodel_id_short = "ManagedDevice";

//     match aas_interfaces::fetch_single_submodel_from_server(
//         &app_data.aasx_server,
//         &app_data.aas_id_short,
//         &app_data.aas_identifier,
//         &submodel_id_short,
//         submodels_collection,
//     )
//     .await
//     {
//         Ok(_) => println!("Successfully fetched submodel {} from server", submodel_id_short),
//         Err(e) => eprintln!("Failed to fetch submodel {} from server: {}", submodel_id_short, e),
//     }
// }