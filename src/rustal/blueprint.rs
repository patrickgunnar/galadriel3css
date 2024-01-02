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

/*
  - Blueprint is responsible for printing the Galadriel3CSS statements to the console
*/
#[napi]
impl Blueprint {
  #[napi(constructor)] // initializes the class' properties
  pub fn new() -> Self {
    Blueprint {
      bold_log_color: String::from("\x1b[1;38;5;10m"),
      bold_error_color: String::from("\x1b[1;38;5;9m"),
      bold_warning_color: String::from("\x1b[1;38;5;11m"),
      bold_info_color: String::from("\x1b[1;38;5;207m"),
      bold_title_color: String::from("\x1b[1;38;5;82m"),
      bold_time_color: String::from("\x1b[1;38;5;33m"),
      log_color: String::from("\x1b[0;32m"),
      error_color: String::from("\x1b[0;31m"),
      warning_color: String::from("\x1b[0;33m"),
      info_color: String::from("\x1b[0;35m"),
      bold: String::from("\x1b[1m"),
      closing_style: String::from("\x1b[0m"),
    }
  }

  #[napi] // get current local time
  pub fn use_current_time(&self) -> String {
    // initializes the Local library
    let time = Local::now();

    // gets the hour, minutes and seconds
    let hour = time.hour();
    let minute = time.minute();
    let second = time.second();

    // returns the formatted local time
    format!(
      "{}[{:02}:{:02}:{:02}]{}",
      self.bold_time_color, hour, minute, second, self.closing_style
    )
  }

  #[napi] // prints the received message as a title formatting
  pub fn title(&self, msg: String) {
    println!("{}{}{}", self.bold_title_color, msg, self.closing_style);
    print!("\n");
  }

  #[napi] // prints the received message as a log formatting
  pub fn log(&self, msg: String) {
    println!(
      " {}•{} {} {}logging{} {}",
      self.bold_log_color,
      self.closing_style,
      self.use_current_time(),
      self.log_color,
      self.closing_style,
      msg
    );
  }

  #[napi] // prints the received message as an error formatting
  pub fn error(&self, msg: String) {
    println!(
      " {}•{} {} {}failure{} {}",
      self.bold_error_color,
      self.closing_style,
      self.use_current_time(),
      self.error_color,
      self.closing_style,
      msg
    );
  }

  #[napi] // prints the received message as warning formatting
  pub fn warn(&self, msg: String) {
    println!(
      " {}•{} {} {}warning{} {}",
      self.bold_warning_color,
      self.closing_style,
      self.use_current_time(),
      self.warning_color,
      self.closing_style,
      msg
    );
  }

  #[napi] // prints the received message as an info formatting
  pub fn info(&self, msg: String) {
    println!(
      " {}•{} {} {}reports{} {}",
      self.bold_info_color,
      self.closing_style,
      self.use_current_time(),
      self.info_color,
      self.closing_style,
      msg
    );
  }

  #[napi] // formats the received message as bold
  pub fn bold(&self, msg: String) -> String {
    format!("{}{}{}", self.bold, msg, self.closing_style)
  }
}
