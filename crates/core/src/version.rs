use std::fmt::{Display, Formatter};

pub(crate) const VERSION_BYTES_LEN: usize = 1;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Version {
  /// Version 1
  Version1,
}

impl Default for Version {
  fn default() -> Self {
    Self::Version1
  }
}

impl Version {
  pub const fn bytes(&self) -> [u8; VERSION_BYTES_LEN] {
    match self {
      Version::Version1 => [0x01],
    }
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Version1 => "v1",
    };
    f.write_str(s)
  }
}
