use std::fmt;

use crate::config::Config;
use crate::{enum_with_helpers, util};

enum_with_helpers! {
    pub enum Level {
        Error,
        Warn,
        Info,
        Verb,
        Debug,
        Trace,
    } default: Warn
}

impl TryFrom<usize> for Level {
  type Error = &'static str;

  fn try_from(value: usize) -> Result<Self, <Level as TryFrom<usize>>::Error> {
    if let Some(val) = Self::from_usize(value) {
      Ok(val)
    } else {
      Err("Unknown log level provided")
    }
  }
}

impl TryFrom<&str> for Level {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, <Level as TryFrom<&str>>::Error> {
    if let Some(val) = Self::from_str(value) {
      Ok(val)
    } else {
      Err("Unknown log level provided")
    }
  }
}

impl fmt::Display for Level {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let value = format!("{self:?}");
    util::format_with_options(&value, f)
  }
}

pub struct Log {
  config: Config,
  message: String,
  level: Level,
  time: String,
}

impl Log {
  pub fn new(config: Config, text: &str, level: Level) -> Log {
    Log {
      level,
      message: text.to_string(),
      config,
      time: util::new_time_string(),
    }
  }
}

#[cfg(feature = "color")]
impl fmt::Display for Log {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let level_text = if self.config.colored_output {
      util::log_level_to_string_colorized(self.level).text
    } else {
      self.level.to_string()
    };

    let log_message_prefix = crate::util::format_log_message_prefix(&self.time.clone(), &level_text, true);
    let log_message = format!("{log_message_prefix}{}", self.message);
    util::format_with_options(&log_message, f)
  }
}

#[cfg(not(feature = "color"))]
impl fmt::Display for Log {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // config is only used when color feature is enabled
    let _ = self.config;

    let log_message_prefix = util::format_log_message_prefix(
      &self.time.clone(),
      &self.level.to_string(),
      false,
    );
    let log_message = format!("{log_message_prefix}{}", self.message);
    util::format_with_options(&log_message, f)
  }
}

fn find_tty_and_update_from(config: Config) -> Config {
  let mut config = config;

  // A terminal is not attached, disable ANSI colored output
  if !util::enable_terminal_colors(config) {
    config.colored_output = false;
  }

  config
}

#[allow(dead_code)]
pub fn error(config: Config, text: &str) {
  let config = find_tty_and_update_from(config);
  let formatted_message = format!("{}", Log::new(config, text, Level::Error));
  eprintln!("{formatted_message}");
}

#[allow(dead_code)]
pub fn warn(config: Config, text: &str) {
  let config = find_tty_and_update_from(config);
  let formatted_message = format!("{}", Log::new(config, text, Level::Warn));
  eprintln!("{formatted_message}");
}

#[allow(dead_code)]
pub fn info(config: Config, text: &str) {
  let config = find_tty_and_update_from(config);
  let formatted_message = format!("{}", Log::new(config, text, Level::Info));
  println!("{formatted_message}");
}

#[allow(dead_code)]
pub fn verb(config: Config, text: &str) {
  let config = find_tty_and_update_from(config);
  let formatted_message = format!("{}", Log::new(config, text, Level::Verb));
  eprintln!("{formatted_message}");
}

#[allow(dead_code)]
pub fn debug(config: Config, text: &str) {
  let config = find_tty_and_update_from(config);
  let formatted_message = format!("{}", Log::new(config, text, Level::Debug));
  eprintln!("{formatted_message}");
}

#[allow(dead_code)]
pub fn trace(config: Config, text: &str) {
  let config = find_tty_and_update_from(config);
  let formatted_message = format!("{}", Log::new(config, text, Level::Trace));
  eprintln!("{formatted_message}");
}

#[macro_export]
macro_rules! init {
    ($($arg:tt)*) => {{
        #[cfg(feature = "log-trace")]
        {
            let text = &std::fmt::format(format_args!($($arg)*));
            let formatted_message_prefix = crate::util::format_log_message_prefix(&new_time_string(), "Init", false);
            let formatted_message = format!("{formatted_message_prefix}{text}");
            eprintln!("{formatted_message}");
        }
    }};
}

#[macro_export]
macro_rules! error {
    ($config:expr, $($arg:tt)*) => {{
        #[cfg(feature = "log-err")]
        {
            if $config.log_level >= Level::Error {
                error($config, &std::fmt::format(format_args!($($arg)*)));
            }
        }
    }};
}

#[macro_export]
macro_rules! warn {
    ($config:expr, $($arg:tt)*) => {{
        #[cfg(feature = "log-warn")]
        {
            if $config.log_level >= Level::Warn {
                warn($config, &std::fmt::format(format_args!($($arg)*)));
            }
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($config:expr, $($arg:tt)*) => {{
        #[cfg(feature = "log-info")]
        {
            if $config.log_level >= Level::Info {
                info($config, &std::fmt::format(format_args!($($arg)*)));
            }
        }
    }};
}

#[macro_export]
macro_rules! verb {
    ($config:expr, $($arg:tt)*) => {{
        #[cfg(feature = "log-verb")]
        {
            if $config.log_level >= Level::Verb {
                verb($config, &std::fmt::format(format_args!($($arg)*)));
            }
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($config:expr, $($arg:tt)*) => {{
        #[cfg(feature = "log-debug")]
        {
            if $config.log_level >= Level::Debug {
                debug($config, &std::fmt::format(format_args!($($arg)*)));
            }
        }
    }};
}

#[macro_export]
macro_rules! trace {
    ($config:expr, $($arg:tt)*) => {{
        #[cfg(feature = "log-trace")]
        {
            if $config.log_level >= Level::Trace {
                trace($config, &std::fmt::format(format_args!($($arg)*)));
            }
        }
    }};
}
