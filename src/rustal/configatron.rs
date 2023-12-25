use crate::rustal::blueprint::Blueprint;
use crate::rustal::file_reader::file_reader;
use crate::rustal::nucleus::NUCLEUS_CONFIG;

#[napi(js_name = "Configatron")]
pub struct Configatron {
  config: std::collections::HashMap<String, serde_json::Value>,
}

/*
  - Configatron is responsible for reading the configuration of the library
*/
#[napi]
impl Configatron {
  #[napi(constructor)]
  pub fn new() -> Self {// sets the config with the galadriel.json configuration on memory
    Configatron {
      config: NUCLEUS_CONFIG.lock().unwrap().clone(),
    }
  }

  fn collects_configurations(&self, keys: Vec<&str>) -> std::collections::HashMap<String, serde_json::Value> {
    // instantiate a map to store the config to be collected
    let mut collected_config: std::collections::HashMap<String, serde_json::Value> =
      std::collections::HashMap::new();

    // loops through the received keys to be collected
    for k in keys.iter() {
      // if the current key is valid and was collected
      if let Some(value) = self.config.get(&k.to_string()) {
        // sets the keys values in the map
        collected_config.insert(k.to_string(), value.clone());
      } else { // if the current key is not valid, sets it to null in the map
        collected_config.insert(k.to_string(), serde_json::Value::Null);
      }
    }

    collected_config
  }

  // collects the configuration from rust code
  pub fn collects_from_rust(&self, keys: Vec<&str>) -> std::collections::HashMap<String, serde_json::Value> {
    // returns the collected configurations
    self.collects_configurations(keys)
  }

  #[napi] // collects the configuration from javascript code
  pub fn collects_from_js(&self, keys: Vec<&str>) -> napi::Result<String> {
    // collects the configurations
    let collected_config = self.collects_configurations(keys);
    // transforms the collected config into json representation
    let return_value = serde_json::to_string(&collected_config).expect("Failed to convert config");

    // returns the collected configurations as json
    Ok(return_value)
  }
}

/*
  - Configatron init is responsible for collecting
  - collecting the configuration from the galadriel.json file
*/
// the configuration's init to starts configuration collection
pub fn configatron_init() -> std::collections::HashMap<String, serde_json::Value> {
  let blueprint = Blueprint::new();
  // initiates a map to hold the configs from the json file 
  let mut map = std::collections::HashMap::new();
  // gets the current dir of the application
  let current_dir = std::env::current_dir().expect("Failed to get current directory");
  // generates a path to the galadriel.json file
  let path = current_dir.join("galadriel.json");
  // reads the file content
  let file_content = file_reader(&path.to_string_lossy());

  match file_content {
    // if the configuration was successfully collected
    // receives the string content - json alike
    Ok(content) => {
      // parsers the json string
      let json_data: Result<serde_json::Value, _> = serde_json::from_str(&content);

      match json_data {
        Ok(objects) => { // if thee json was successfully parsed
          // extracts the parsed objects
          if let Some(obj) = objects.as_object() {
            // loops over the objects, extracting the keys and values
            for (key, value) in obj.iter() {
              // inserts the key - value pairs into the map
              map.insert(key.to_string(), value.clone());
            }
          }
        }
        Err(_) => { // prints to the console some error messages
          blueprint.error("the data in 'galadriel.json' cannot be transformed".to_string());
          blueprint.info("verify the data in 'galadriel.json' and try again!".to_string());
        }
      }
    }
    Err(_) => { // prints to the console some error messages
      blueprint.error("the data from 'galadriel.json' was not read".to_string());
      blueprint.info("verify if the 'galadriel.json' is in root directory".to_string());
    }
  }

  // returns the map containing the 
  // configurations from the galadriel.json file
  map
}
