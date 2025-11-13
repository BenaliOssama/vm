use crate::config::MEM_SIZE;

pub fn bytes_to_i16(bytes: &[u8]) -> i16 {
    let mut arr = [0u8; 2]; // 2 bytes for i16
    let len = bytes.len();
    arr[2 - len..].copy_from_slice(bytes);
    i16::from_be_bytes(arr)
}
pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
    let mut arr = [0u8; 4]; // 4 bytes for i32
    let len = bytes.len();
    // copy bytes to the end of the array (big-endian)
    arr[4 - len..].copy_from_slice(bytes);
    i32::from_be_bytes(arr)
}

pub fn wrap_address(pc: usize, offset: i16) -> usize {
    let mut addr = (pc as isize + offset as isize) % MEM_SIZE as isize;
    if addr < 0 {
        addr += MEM_SIZE as isize;
    }
    addr as usize
}
