use twox_hash::XxHash32;

pub(crate) const CHECKSUM_LEN: usize = size_of::<u32>();

pub(crate) fn make_checksum(seed: u32, data: &[u8]) -> u32 {
  XxHash32::oneshot(seed, data)
}

pub(crate) fn write_checksum(checksum: u32) -> Vec<u8> {
  checksum.to_be_bytes().to_vec()
}

pub(crate) fn parse_checksum(data: &[u8]) -> u32 {
  u32::from_be_bytes(AsRef::<[u8]>::as_ref(&data).try_into().unwrap())
}
