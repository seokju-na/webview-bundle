use std::fmt::{Display, Formatter};

pub(crate) const VERSION_LEN: usize = 1;

/// Version fo Webview Bundle.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum Version {
  /// Version 1
  #[default]
  V1,
}

impl Version {
  pub const fn bytes(&self) -> [u8; VERSION_LEN] {
    match self {
      Version::V1 => [0x01],
    }
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::V1 => "v1",
    };
    f.write_str(s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default() {
    assert_eq!(Version::default(), Version::V1);
  }

  #[test]
  fn bytes() {
    assert_eq!(Version::V1.bytes(), [0x01]);
  }

  #[test]
  fn display() {
    assert_eq!(format!("{}", Version::V1), "v1");
  }
}
