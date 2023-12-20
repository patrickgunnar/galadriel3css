use chrono::{Local, Timelike};

#[napi(js_name = "Blueprint")]
pub struct Blueprint {
  bold_log_color: String,
  bold_error_color: String,
  bold_warning_color: String,
  bold_info_color: String,
  bold_title_color: String,
  bold_time_color: String,
  log_color: String,
  error_color: String,
  warning_color: String,
  info_color: String,
  bold: String,
  closing_style: String,
}

#[napi]
impl Blueprint {
  #[napi(constructor)]
  pub fn new() -> Self {
    Blueprint {
      bold_log_color: String::from("\x1b[1;38;5;10m"),
      bold_error_color: String::from("\x1b[1;38;5;9m"),
      bold_warning_color: String::from("\x1b[1;38;5;11m"),
      bold_info_color: String::from("\x1b[1;38;5;12m"),
      bold_title_color: String::from("\x1b[1;38;5;82m"),
      bold_time_color: String::from("\x1b[1;38;5;33m"),
      log_color: String::from("\x1b[0;32m"),
      error_color: String::from("\x1b[0;31m"),
      warning_color: String::from("\x1b[0;33m"),
      info_color: String::from("\x1b[0;34m"),
      bold: String::from("\x1b[1m"),
      closing_style: String::from("\x1b[0m"),
    }
  }

  #[napi]
  pub fn use_current_time(&self) -> String {
    let time = Local::now();

    let hour = time.hour();
    let minute = time.minute();
    let second = time.second();

    format!(
      "{}[{:02}:{:02}:{:02}]{}",
      self.bold_time_color, hour, minute, second, self.closing_style
    )
  }

  #[napi]
  pub fn title(&self, msg: String) {
    println!("{}{}{}", self.bold_title_color, msg, self.closing_style);
    print!("\n");
  }

  #[napi]
  pub fn log(&self, msg: String) {
    println!(
      " {}•{} {} {}log____{} {}",
      self.bold_log_color,
      self.closing_style,
      self.use_current_time(),
      self.log_color,
      self.closing_style,
      msg
    );
  }

  #[napi]
  pub fn error(&self, msg: String) {
    println!(
      " {}•{} {} {}error__{} {}",
      self.bold_error_color,
      self.closing_style,
      self.use_current_time(),
      self.error_color,
      self.closing_style,
      msg
    );
  }

  #[napi]
  pub fn warn(&self, msg: String) {
    println!(
      " {}•{} {} {}warn___{} {}",
      self.bold_warning_color,
      self.closing_style,
      self.use_current_time(),
      self.warning_color,
      self.closing_style,
      msg
    );
  }

  #[napi]
  pub fn info(&self, msg: String) {
    println!(
      " {}•{} {} {}info___{} {}",
      self.bold_info_color,
      self.closing_style,
      self.use_current_time(),
      self.info_color,
      self.closing_style,
      msg
    );
  }

  #[napi]
  pub fn bold(&self, msg: String) -> String {
    format!("{}{}{}", self.bold, msg, self.closing_style)
  }
}
