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

extern crate libc;
extern crate errno;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate ioctl_gen;

use std::path::Path;

mod device;
mod frontend;
mod demux;

pub use device::{ReadWriteMode,BlockMode,DeviceError,DeviceResult};
pub use frontend::*;
pub use demux::*;

pub fn open_frontend(adapter: u32, frontend: u32, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Frontend> {
    let path_string = format!("/dev/dvb/adapter{}/frontend{}", adapter, frontend);
    let path = Path::new(&path_string);
    Frontend::open(path, rw_mode, block_mode)
}

pub fn open_demux(adapter: u32, demux: u32, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Demux> {
    let path_string = format!("/dev/dvb/adapter{}/demux{}", adapter, demux);
    let path = Path::new(&path_string);
    Demux::open(path, rw_mode, block_mode)
}

pub fn open_dvr(adapter: u32, dvr: u32, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<Dvr> {
    let path_string = format!("/dev/dvb/adapter{}/dvr{}", adapter, dvr);
    let path = Path::new(&path_string);
    Dvr::open(path, rw_mode, block_mode)
}
