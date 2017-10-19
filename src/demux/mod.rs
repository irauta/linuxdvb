// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[allow(dead_code,non_camel_case_types,non_snake_case)]
mod ffi;

use std::path::Path;
use libc::{c_uint,c_ulong};

pub const DEMUX_FILTER_SIZE: usize = ffi::DMX_FILTER_SIZE as usize;

#[allow(dead_code,non_camel_case_types,non_snake_case)]
mod enums {
    use super::ffi as ffi;

    include!(concat!(env!("OUT_DIR"), "/demux-enums.rs"));
}

pub use self::enums::{Input,Output,PesType};

pub struct Demux {
    device: DeviceFileDescriptor
}

use super::device::{DeviceFileDescriptor,BlockMode,ReadWriteMode,DeviceResult};

impl Demux {
    pub fn open(file: &Path, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Demux> {
        let device = try!(DeviceFileDescriptor::open(file, rw_mode, block_mode));
        Ok(Demux { device: device })
    }

    pub fn read(&self, buffer: &mut [u8]) -> DeviceResult<usize> {
        self.device.read(buffer)
    }

    pub fn start(&self) -> DeviceResult<()> {
        self.device.ioctl_argumentless(ffi::DMX_START as c_ulong)
    }

    pub fn stop(&self) -> DeviceResult<()> {
        self.device.ioctl_argumentless(ffi::DMX_STOP as c_ulong)
    }

    pub fn set_filter(&self, filter_params: &SectionFilterParams) -> DeviceResult<()> {
        let mut ffi_params = ffi::Struct_dmx_sct_filter_params {
            pid: filter_params.pid,
            filter: ffi::Struct_dmx_filter {
                filter: filter_params.filter.filter,
                mask: filter_params.filter.mask,
                mode: filter_params.filter.mode,
            },
            timeout: filter_params.timeout,
            flags: filter_params.flags.bits()
        };
        self.device.ioctl_pointer(ffi::DMX_SET_FILTER as c_ulong, &mut ffi_params)
    }

    pub fn set_pes_filter(&self, filter_params: &PesFilterParams) -> DeviceResult<()> {
        let mut ffi_params = ffi::Struct_dmx_pes_filter_params {
            pid: filter_params.pid,
            input: filter_params.input.into(),
            output: filter_params.output.into(),
            pes_type: filter_params.pes_type.into(),
            flags: filter_params.flags.bits()
        };
        self.device.ioctl_pointer(ffi::DMX_SET_PES_FILTER as c_ulong, &mut ffi_params)
    }

    pub fn set_buffer_size(&self, buffer_size: u32) -> DeviceResult<()> {
        set_buffer_size(&self.device, buffer_size)
    }

    pub fn get_system_time_counter(&self, num: u32) -> DeviceResult<SystemTimeCounter> {
        let mut ffi_stc = ffi::Struct_dmx_stc { num: num as c_uint, base: 0, stc: 0 };
        try!(self.device.ioctl_pointer(ffi::DMX_GET_STC as c_ulong, &mut ffi_stc));
        Ok(SystemTimeCounter { base: ffi_stc.base, counter: ffi_stc.stc })
    }

    pub fn get_pes_pids(&self) -> DeviceResult<[u16; 5]> {
        let mut ffi_pids = [0u16; 5];
        try!(self.device.ioctl_pointer(ffi::DMX_GET_PES_PIDS as c_ulong, ffi_pids.as_mut_ptr()));
        Ok(ffi_pids)
    }

    /// This function is undocumented in the official documentation, and doesn't seem to be
    /// actually in use. It is still nevertheless included here, if it is used by out-of-tree DVB
    /// drivers.
    pub fn get_caps(&self) -> DeviceResult<DemuxCaps> {
        let mut ffi_caps = ffi::Struct_dmx_caps { caps: 0, num_decoders: 0 };
        try!(self.device.ioctl_pointer(ffi::DMX_GET_CAPS as c_ulong, &mut ffi_caps));
        Ok(DemuxCaps { caps: ffi_caps.caps, num_decoders: ffi_caps.num_decoders as i32 })
    }

    /// Like get_caps(), this doesn't seem to be actively in use in Linux, but is still implemented.
    pub fn set_source(&self, source: enums::DemuxSource) -> DeviceResult<()> {
        let mut ffi_source: ffi::dmx_source_t =  source.into();
        self.device.ioctl_pointer(ffi::DMX_SET_SOURCE as c_ulong, &mut ffi_source)
    }

    pub fn add_pid(&self, pid: u16) -> DeviceResult<()> {
        let mut ffi_pid = pid;
        self.device.ioctl_pointer(ffi::DMX_ADD_PID as c_ulong, &mut ffi_pid)
    }

    pub fn remove_pid(&self, pid: u16) -> DeviceResult<()> {
        let mut ffi_pid = pid;
        self.device.ioctl_pointer(ffi::DMX_REMOVE_PID as c_ulong, &mut ffi_pid)
    }
}

pub struct DemuxCaps {
    pub caps: u32,
    pub num_decoders: i32
}

pub struct SystemTimeCounter {
    pub base: u32,
    pub counter: u64
}

pub struct DemuxFilter {
    pub filter: [u8; DEMUX_FILTER_SIZE],
    pub mask: [u8; DEMUX_FILTER_SIZE],
    pub mode: [u8; DEMUX_FILTER_SIZE],
}

impl Default for DemuxFilter {
    fn default() -> DemuxFilter {
        DemuxFilter {
            filter: [0u8; DEMUX_FILTER_SIZE],
            mask: [0u8; DEMUX_FILTER_SIZE],
            mode: [0u8; DEMUX_FILTER_SIZE],
        }
    }
}

pub struct SectionFilterParams {
    pub pid: u16,
    pub filter: DemuxFilter,
    pub timeout: u32,
    pub flags: flags::SectionFilterFlags
}

pub struct PesFilterParams {
    pub pid: u16,
    pub input: Input,
    pub output: Output,
    pub pes_type: PesType,
    pub flags: flags::PesFilterFlags
}

#[allow(dead_code,non_upper_case_globals)]
pub mod flags {
    use super::ffi as ffi;
    bitflags!{
        pub struct SectionFilterFlags: u32 {
            const SecCheckCrC = ffi::DMX_CHECK_CRC;
            const SecOneShot = ffi::DMX_ONESHOT;
            const SecImmediateStart = ffi::DMX_IMMEDIATE_START;
        }
    }

    bitflags!{
        pub struct PesFilterFlags: u32 {
            const PesImmediateStart = ffi::DMX_IMMEDIATE_START;
        }
    }
}

pub struct Dvr {
    device: DeviceFileDescriptor
}

impl Dvr {
    pub fn open(file: &Path, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Dvr> {
        let device = try!(DeviceFileDescriptor::open(file, rw_mode, block_mode));
        Ok(Dvr { device: device })
    }

    pub fn write(&self, buffer: &mut [u8]) -> DeviceResult<usize> {
        self.device.write(buffer)
    }

    pub fn set_buffer_size(&self, buffer_size: u32) -> DeviceResult<()> {
        set_buffer_size(&self.device, buffer_size)
    }
}

fn set_buffer_size(device: &DeviceFileDescriptor, buffer_size: u32) -> DeviceResult<()> {
    device.ioctl_value(ffi::DMX_SET_BUFFER_SIZE as c_ulong, buffer_size as c_ulong)
}
