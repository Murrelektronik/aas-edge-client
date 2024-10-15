use std::sync::Arc;
use tokio::sync::Mutex;
use rocksdb::DB;

pub struct AppState {
    pub health_check_response: Mutex<String>,
    // Replacing the MongoDB URI with RocksDB
    pub rocksdb: Arc<Mutex<DB>>, // RocksDB instance wrapped in Arc<Mutex<>> for shared state across threads
    pub aas_identifier: String,
    pub aas_id_short: String,
    pub aasx_server: String,
    pub device_name: String,
    pub offboarding_time: i64,
}