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

use libc::{c_ulong};

pub struct Demux {
    device: DeviceFileDescriptor
}

use super::device::{DeviceFileDescriptor,BlockMode,ReadWriteMode,DeviceResult};

impl Demux {
    pub fn open(file: &Path, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Demux> {
        let device = try!(DeviceFileDescriptor::open(file, rw_mode, block_mode));
        Ok(Demux { device: device })
    }

    pub fn read(&self) -> DeviceResult<()> {
        unimplemented!();
    }

    // Should this even exist here?
    pub fn write(&self) -> DeviceResult<()> {
        unimplemented!();
    }

    pub fn start(&self) -> DeviceResult<()> {
        self.device.ioctl_argumentless(ffi::DMX_START as c_ulong)
    }

    pub fn stop(&self) -> DeviceResult<()> {
        self.device.ioctl_argumentless(ffi::DMX_STOP as c_ulong)
    }

    pub fn set_filter(&self) -> DeviceResult<()> {
        unimplemented!();
    }

    pub fn set_pes_filter(&self) -> DeviceResult<()> {
        unimplemented!();
    }

    pub fn set_buffer_size(&self, buffer_size: u32) -> DeviceResult<()> {
        let mut ffi_size = buffer_size as c_ulong;
        self.device.ioctl_pointer(ffi::DMX_SET_BUFFER_SIZE as c_ulong, &mut ffi_size)
    }

    pub fn get_system_time_counter(&self) -> DeviceResult<SystemTimeCounter> {
        unimplemented!();
    }

    pub fn get_pes_pids(&self) -> DeviceResult<[u16; 5]> {
        unimplemented!();
    }

    /// This function is undocumented in the official, and doesn't seem to be actually in use.
    /// It is still nevertheless included here, if it is used by out-of-tree DVB drivers.
    pub fn get_caps(&self) -> DeviceResult<DemuxCaps> {
        let mut ffi_caps = ffi::Struct_dmx_caps { caps: 0, num_decoders: 0 };
        try!(self.device.ioctl_pointer(ffi::DMX_GET_CAPS as c_ulong, &mut ffi_caps));
        Ok(DemuxCaps { caps: ffi_caps.caps, num_decoders: ffi_caps.num_decoders as i32 })
    }

    pub fn set_source(&self) -> DeviceResult<()> {
        unimplemented!();
    }

    pub fn add_pid(&self, pid: u16) -> DeviceResult<()> {
        unimplemented!();
    }

    pub fn remove_pid(&self, pid: u16) -> DeviceResult<()> {
        unimplemented!();
    }
}

pub struct DemuxCaps {
    pub caps: u32,
    pub num_decoders: i32
}

pub struct SystemTimeCounter;
