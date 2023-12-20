use crate::rustal::blueprint::Blueprint;
use crate::rustal::file_reader::file_reader;
use crate::rustal::nucleus::NUCLEUS_CONFIG;

#[napi(js_name = "Configatron")]
pub struct Configatron {
  config: std::collections::HashMap<String, serde_json::Value>,
}

#[napi]
impl Configatron {
  #[napi(constructor)]
  pub fn new() -> Self {
    Configatron {
      config: NUCLEUS_CONFIG.lock().unwrap().clone(),
    }
  }

  pub fn collect_from_rust(&self, keys: Vec<&str>) -> std::collections::HashMap<String, serde_json::Value> {
    let mut collected_config: std::collections::HashMap<String, serde_json::Value> =
      std::collections::HashMap::new();

    for k in keys.iter() {
      if let Some(value) = self.config.get(&k.to_string()) {
        collected_config.insert(k.to_string(), value.clone());
      } else {
        collected_config.insert(k.to_string(), serde_json::Value::Null);
      }
    }

    collected_config
  }

  #[napi]
  pub fn collect_from_js(&self, keys: Vec<&str>) -> napi::Result<String> {
    let mut collected_config: std::collections::HashMap<String, serde_json::Value> =
      std::collections::HashMap::new();

    for k in keys.iter() {
      if let Some(value) = self.config.get(&k.to_string()) {
        collected_config.insert(k.to_string(), value.clone());
      } else {
        collected_config.insert(k.to_string(), serde_json::Value::Null);
      }
    }

    let return_value = serde_json::to_string(&collected_config).expect("Failed to convert config");

    Ok(return_value)
  }
}

pub fn configatron_init() -> std::collections::HashMap<String, serde_json::Value> {
  let blueprint = Blueprint::new();
  let mut map = std::collections::HashMap::new();
  let current_dir = std::env::current_dir().expect("Failed to get current directory");
  let path = current_dir.join("galadriel.json");
  let file_content = file_reader(&path.to_string_lossy());

  match file_content {
    Ok(content) => {
      let json_data: Result<serde_json::Value, _> = serde_json::from_str(&content);

      match json_data {
        Ok(objects) => {
          if let Some(obj) = objects.as_object() {
            for (key, value) in obj.iter() {
              map.insert(key.to_string(), value.clone());
            }
          }
        }
        Err(_) => {
          blueprint.error("the data in 'galadriel.json' cannot be transformed".to_string());
          blueprint.info("verify the data in 'galadriel.json' and try again!".to_string());
        }
      }
    }
    Err(_) => {
      blueprint.error("the data from 'galadriel.json' was not read".to_string());
      blueprint.info("verify if the 'galadriel.json' is in root directory".to_string());
    }
  }

  map
}
