use crate::actions::Actions;
use crate::version::Version;
use relative_path::RelativePathBuf;

pub trait PackageManager {
  fn name(&self) -> &str;
  fn path(&self) -> &RelativePathBuf;
  fn version(&self) -> &Version;
  fn can_publish(&self) -> bool;
  fn write(&self, next_version: &Version) -> Result<Vec<Actions>, crate::Error>;
  fn publish(&self, next_version: &Version) -> Result<Vec<Actions>, crate::Error>;
}
