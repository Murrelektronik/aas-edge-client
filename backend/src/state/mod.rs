// Authors: Manh-Linh Phan (manh.linh.phan@yacoub.de)


use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: Mutex<String>,
    // Adding new fields to the AppState
    pub mongo_uri: String,
    pub aas_identifier: String,
    pub aas_id_short: String,
    pub aasx_server: String,
    pub device_name: String,
    pub offboarding_time: i64,
}
