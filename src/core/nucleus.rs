use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
  pub static ref NUCLEUS_CONFIG: Arc<Mutex<HashMap<String, serde_json::Value>>> =
    Arc::new(Mutex::new(HashMap::new()));
}
