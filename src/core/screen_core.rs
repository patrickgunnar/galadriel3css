use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  pub static ref SCREEN_CORE: HashMap<String, String> = {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("min4K".to_string(), "#min-width: 2560px".to_string());
    map.insert("minXLDesktop".to_string(), "#min-width: 1440px".to_string());
    map.insert(
      "minLargeDesktop".to_string(),
      "#min-width: 1200px".to_string(),
    );
    map.insert(
      "minStandardDesktop".to_string(),
      "#min-width: 992px".to_string(),
    );
    map.insert(
      "minSmallTabletPortrait".to_string(),
      "#min-width: 768px".to_string(),
    );
    map.insert(
      "minMediumPhone".to_string(),
      "#min-width: 426px".to_string(),
    );
    map.insert("minSmallPhone".to_string(), "#min-width: 320px".to_string());
    map.insert("max4K".to_string(), "#max-width: 2559px".to_string());
    map.insert("maxXLDesktop".to_string(), "#max-width: 1439px".to_string());
    map.insert(
      "maxLargeDesktop".to_string(),
      "#max-width: 1199px".to_string(),
    );
    map.insert(
      "maxStandardDesktop".to_string(),
      "#max-width: 991px".to_string(),
    );
    map.insert(
      "maxSmallTabletPortrait".to_string(),
      "#max-width: 767px".to_string(),
    );
    map.insert(
      "maxMediumPhone".to_string(),
      "#max-width: 480px".to_string(),
    );
    map.insert("maxSmallPhone".to_string(), "#max-width: 425px".to_string());

    map
  };
}
