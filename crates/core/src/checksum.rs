use twox_hash::XxHash32;

pub(crate) const CHECKSUM_BYTES_LEN: usize = size_of::<u32>();

pub fn make_checksum(seed: u32, data: &[u8]) -> u32 {
  XxHash32::oneshot(seed, data)
}

pub fn get_checksum(data: &[u8]) -> u32 {
  u32::from_be_bytes(AsRef::<[u8]>::as_ref(&data).try_into().unwrap())
}
