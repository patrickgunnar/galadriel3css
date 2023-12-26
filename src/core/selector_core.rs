use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  pub static ref SELECTOR_CORE: HashMap<String, String> = {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("hover".to_string(), ":hover".to_string());
    map.insert("active".to_string(), ":active".to_string());
    map.insert("focus".to_string(), ":focus".to_string());
    map.insert("firstChild".to_string(), ":first-child".to_string());
    map.insert("lastChild".to_string(), ":last-child".to_string());
    map.insert("firstOfType".to_string(), ":first-of-type".to_string());
    map.insert("lastOfType".to_string(), ":last-of-type".to_string());
    map.insert("onlyChild".to_string(), ":only-child".to_string());
    map.insert("onlyOfType".to_string(), ":only-of-type".to_string());
    map.insert("targetPseudoClass".to_string(), ":target".to_string());
    map.insert("visited".to_string(), ":visited".to_string());
    map.insert("checked".to_string(), ":checked".to_string());
    map.insert("disabled".to_string(), ":disabled".to_string());
    map.insert("enabled".to_string(), ":enabled".to_string());
    map.insert("readOnly".to_string(), ":read-only".to_string());
    map.insert("readWrite".to_string(), ":read-write".to_string());
    map.insert(
      "placeholderShown".to_string(),
      ":placeholder-shown".to_string(),
    );
    map.insert("valid".to_string(), ":valid".to_string());
    map.insert("invalid".to_string(), ":invalid".to_string());
    map.insert("required".to_string(), ":required".to_string());
    map.insert("optional".to_string(), ":optional".to_string());
    map.insert("fullscreen".to_string(), ":fullscreen".to_string());
    map.insert("focusWithin".to_string(), ":focus-within".to_string());
    map.insert("firstLine".to_string(), "::first-line".to_string());
    map.insert("firstLetter".to_string(), "::first-letter".to_string());
    map.insert("before".to_string(), "::before".to_string());
    map.insert("after".to_string(), "::after".to_string());
    map.insert("outOfRange".to_string(), ":out-of-range".to_string());
    map.insert("root".to_string(), ":root".to_string());
    map.insert("firstPage".to_string(), ":first-page".to_string());
    map.insert("leftPage".to_string(), ":left-page".to_string());
    map.insert("rightPage".to_string(), ":right-page".to_string());
    map.insert("empty".to_string(), ":empty".to_string());

    map
  };
}
