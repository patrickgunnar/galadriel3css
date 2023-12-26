use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  pub static ref SCREEN_CORE: HashMap<String, String> = {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("min4KScreen".to_string(), "#min-width: 2560px".to_string());
    map.insert(
      "minExtraLargeDesktops".to_string(),
      "#min-width: 1440px".to_string(),
    );
    map.insert(
      "minLargeDesktops".to_string(),
      "#min-width: 1200px".to_string(),
    );
    map.insert(
      "minStandardDesktops".to_string(),
      "#min-width: 992px".to_string(),
    );
    map.insert(
      "minPortraitTablets".to_string(),
      "#min-width: 768px".to_string(),
    );
    map.insert(
      "minLargeSmartphones".to_string(),
      "#min-width: 426px".to_string(),
    );
    map.insert(
      "minStandardSmartphones".to_string(),
      "#min-width: 320px".to_string(),
    );
    map.insert("max4KScreen".to_string(), "#max-width: 2559px".to_string());
    map.insert(
      "maxExtraLargeDesktops".to_string(),
      "#max-width: 1439px".to_string(),
    );
    map.insert(
      "maxLargeDesktops".to_string(),
      "#max-width: 1199px".to_string(),
    );
    map.insert(
      "maxStandardDesktops".to_string(),
      "#max-width: 991px".to_string(),
    );
    map.insert(
      "maxPortraitTablets".to_string(),
      "#max-width: 767px".to_string(),
    );
    map.insert(
      "maxLargeSmartphones".to_string(),
      "#max-width: 480px".to_string(),
    );
    map.insert(
      "maxStandardSmartphones".to_string(),
      "#max-width: 425px".to_string(),
    );

    map
  };
}
