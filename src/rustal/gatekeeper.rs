pub struct Gatekeeper {
    modular: bool,
    path: String,
    code: String,
}

impl Gatekeeper {
  pub fn new(modular: bool, path: String, code: String) -> Self {
    Gatekeeper {
        modular, path, code
    }
  }
}
