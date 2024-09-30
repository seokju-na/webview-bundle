use crate::logging::{LoggingKind, LoggingLevel};
use bpaf::Bpaf;
use std::str::FromStr;

#[derive(Debug, Clone, Bpaf)]
pub struct CliOptions {
  /// Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
  #[bpaf(long("colors"), argument("off|force"))]
  pub colors: Option<ColorsArg>,

  /// The level of logging. In order, from the most verbose to the least verbose: debug, info, warn, error.
  ///
  /// The value `none` won't show any logging.
  #[bpaf(
    long("log-level"),
    argument("none|debug|info|warn|error"),
    fallback(LoggingLevel::default()),
    display_fallback
  )]
  pub log_level: LoggingLevel,

  /// How the log should look like.
  #[bpaf(
    long("log-kind"),
    argument("pretty|compact|json"),
    fallback(LoggingKind::default()),
    display_fallback
  )]
  pub log_kind: LoggingKind,
}

#[derive(Debug, Clone)]
pub enum ColorsArg {
  Off,
  Force,
}

impl FromStr for ColorsArg {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "off" => Ok(Self::Off),
      "force" => Ok(Self::Force),
      _ => Err(format!(
        "value {s:?} is not valid for the --colors argument"
      )),
    }
  }
}
