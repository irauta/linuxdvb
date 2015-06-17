
use std::error::Error;
use std::fmt::{self,Display,Formatter};

use super::device::{DeviceFileDescriptor,BlockMode,ReadWriteMode,DeviceResult,DeviceError};

#[allow(dead_code,non_camel_case_types,non_snake_case)]
mod ffi;

pub mod caps;

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

    pub fn get_info(&self) -> DeviceResult<FrontendInfo> {
        unimplemented!();
    }
    pub fn read_status(&self) -> DeviceResult<FrontendStatus> {
        unimplemented!();
    }
    pub fn set_properties(&self, properties: &[properties::SetPropertyValue]) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn get_properties(&self, properties: &[properties::GetProperty]) -> PropertyResult<Vec<properties::GetPropertyValue>> {
        unimplemented!();
    }
    pub fn diseqc_reset_overload(&self) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn diseqc_send_master_cmd(&self, command: DiseqcMasterCommand) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn diseqc_recv_slave_reply(&self) -> DeviceResult<DiseqcSlaveReply> {
        unimplemented!();
    }
    pub fn diseqc_send_burst(&self, command: SecMiniCmd) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn set_tone(&self, tone: SecToneMode) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn set_voltage(&self, voltage: properties::Voltage) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn enable_high_lnb_voltage(&self, high: u32) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn set_frontend_tune_mode(&self, tune_mode: TuneMode) -> DeviceResult<()> {
        unimplemented!();
    }
}

#[derive(Clone,Debug)]
pub struct FrontendInfo {
    pub name: String,
    pub frequency_min: u32,
    pub frequency_max: u32,
    pub frequency_stepsize: u32,
    pub frequency_tolerance: u32,
    pub symbol_rate_min: u32,
    pub symbol_rate_max: u32,
    pub symbol_rate_tolerance: u32,
    pub caps: caps::FrontendCaps,
}

pub struct FrontendStatus {
    pub has_signal: bool,
    pub has_carrier: bool,
    pub has_viterbi: bool,
    pub has_sync: bool,
    pub has_lock: bool,
    pub timedout: bool,
    pub reinit: bool
}

pub struct DiseqcSlaveReply {
    pub msg: [u8; 4],
    pub msg_len: u8,
    pub timeout: i32
}

pub struct DiseqcMasterCommand {
    pub msg: [u8; 6],
    pub msg_len: u8
}

pub enum SecMiniCmd {
    A,
    B
}

pub enum SecToneMode {
    On,
    Off
}

pub enum TuneMode {
    Normal,
    OneShot
}

pub type PropertyResult<T> = Result<T, PropertyError>;

#[derive(Clone,Debug)]
pub enum PropertyError {
    DeviceError(DeviceError),
    PropertyMappingError(properties::PropertyMappingError)
}

impl Error for PropertyError {
    fn description(&self) -> &str {
        match *self {
            PropertyError::DeviceError(ref err) => err.description(),
            PropertyError::PropertyMappingError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PropertyError::DeviceError(ref err) => Some(err),
            PropertyError::PropertyMappingError(ref err) => Some(err)
        }
    }
}

impl Display for PropertyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "PropertyError: {}", self.description())
    }
}