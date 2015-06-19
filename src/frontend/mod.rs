
use std::error::Error;
use std::fmt::{self,Display,Formatter};
use std::default::Default;

use libc::{c_int,c_ulong};

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
        let mut ffi_status: ffi::Enum_fe_status = 0;
        try!(self.device.ioctl_pointer(ffi::FE_READ_STATUS as c_ulong, &mut ffi_status));
        Ok(FrontendStatus {
            has_signal: ffi_status & ffi::FE_HAS_SIGNAL != 0,
            has_carrier: ffi_status & ffi::FE_HAS_CARRIER != 0,
            has_viterbi: ffi_status & ffi::FE_HAS_VITERBI != 0,
            has_sync: ffi_status & ffi::FE_HAS_SYNC != 0,
            has_lock: ffi_status & ffi::FE_HAS_LOCK != 0,
            timedout: ffi_status & ffi::FE_TIMEDOUT != 0,
            reinit: ffi_status & ffi::FE_REINIT != 0,
        })
    }
    pub fn set_properties(&self, properties: &[properties::SetPropertyValue]) -> DeviceResult<()> {
        unimplemented!();
    }
    pub fn get_properties(&self, properties: &[properties::GetProperty]) -> PropertyResult<Vec<properties::GetPropertyValue>> {
        unimplemented!();
    }

    pub fn diseqc_reset_overload(&self) -> DeviceResult<()> {
        self.device.ioctl_argumentless(ffi::FE_DISEQC_RESET_OVERLOAD as c_ulong)
    }

    pub fn diseqc_send_master_cmd(&self, command: DiseqcMasterCommand) -> DeviceResult<()> {
        let mut ffi_cmd = ffi::Struct_dvb_diseqc_master_cmd {
            msg: command.msg,
            msg_len: command.msg_len
        };
        self.device.ioctl_pointer(ffi::FE_DISEQC_SEND_MASTER_CMD as c_ulong, &mut ffi_cmd)
    }

    /// The timeout parameter is not used by most drivers. (2015-06-17)
    pub fn diseqc_recv_slave_reply(&self, timeout: u32) -> DeviceResult<DiseqcSlaveReply> {
        let mut ffi_reply: ffi::Struct_dvb_diseqc_slave_reply = Default::default();
        ffi_reply.timeout = timeout as c_int;
        try!(self.device.ioctl_pointer(ffi::FE_DISEQC_RECV_SLAVE_REPLY as c_ulong, &mut ffi_reply));
        Ok(DiseqcSlaveReply {
            msg: ffi_reply.msg,
            msg_len: ffi_reply.msg_len
        })
    }

    pub fn diseqc_send_burst(&self, command: SecMiniCmd) -> DeviceResult<()> {
        let mut command = match command {
            SecMiniCmd::A => ffi::SEC_MINI_A,
            SecMiniCmd::B => ffi::SEC_MINI_B
        };
        self.device.ioctl_pointer(ffi::FE_DISEQC_SEND_BURST as c_ulong, &mut command)
    }

    pub fn set_tone(&self, tone: SecToneMode) -> DeviceResult<()> {
        let mut tone = match tone {
            SecToneMode::On => ffi::SEC_TONE_ON,
            SecToneMode::Off => ffi::SEC_TONE_OFF
        };
        self.device.ioctl_pointer(ffi::FE_SET_TONE as c_ulong, &mut tone)
    }

    pub fn set_voltage(&self, voltage: properties::Voltage) -> DeviceResult<()> {
        let voltage = match voltage {
            properties::Voltage::SecVoltage13 => ffi::SEC_VOLTAGE_13,
            properties::Voltage::SecVoltage18 => ffi::SEC_VOLTAGE_18,
            properties::Voltage::SecVoltageOff => ffi::SEC_VOLTAGE_OFF,
        };
        self.device.ioctl_value(ffi::FE_SET_VOLTAGE as c_ulong, voltage)
    }

    pub fn enable_high_lnb_voltage(&self, high: u32) -> DeviceResult<()> {
        self.device.ioctl_value(ffi::FE_ENABLE_HIGH_LNB_VOLTAGE as c_ulong, high)
    }

    pub fn set_frontend_tune_mode(&self, tune_mode: TuneMode) -> DeviceResult<()> {
        let value = match tune_mode {
            TuneMode::Normal => 0,
            TuneMode::OneShot => ffi::FE_TUNE_MODE_ONESHOT
        };
        self.device.ioctl_value(ffi::FE_SET_FRONTEND_TUNE_MODE as c_ulong, value)
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
    pub msg_len: u8
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