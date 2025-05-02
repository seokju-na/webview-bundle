use bpaf::Bpaf;
use std::str::FromStr;

#[derive(Debug, Clone, Bpaf)]
pub struct CliOptions {
  /// Set the formatting mode for markup: "off" prints everything as plain text, "force" forces the formatting of markup using ANSI even if the console output is determined to be incompatible
  #[bpaf(long("colors"), argument("off|force"))]
  pub colors: Option<ColorsArg>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
