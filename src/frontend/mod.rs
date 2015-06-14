
use super::device::{DeviceFileDescriptor,BlockMode,ReadWriteMode,DeviceResult};

#[allow(dead_code,non_camel_case_types,non_snake_case)]
mod ffi;

#[allow(dead_code,non_camel_case_types,non_snake_case)]
pub mod properties {
    use super::ffi as ffi;

    include!(concat!(env!("OUT_DIR"), "/frontend-properties.rs"));
}

pub struct Frontend {
    device: DeviceFileDescriptor
}

impl Frontend {
    pub fn open(filename: &str, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Frontend> {
        let device = try!(DeviceFileDescriptor::open(filename, rw_mode, block_mode));
        Ok(Frontend { device: device })
    }
}