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

// Relevant file in Linux kernel code: drivers/media/dvb-core/dmxdev.c
// For some starting points, see functions dvb_demux_do_ioctl, dvb_dvr_do_ioctl,
// and structs dvb_demux_fops, dvb_dvr_fops.

use std::error::Error;
use std::fmt::{self,Display,Formatter};
use std::ffi::CString;
use std::borrow::Borrow;
use std::path::Path;

use libc::{c_ulong,c_int,size_t,c_void};
use libc::{open,close,read,write};
use libc::{O_RDONLY,O_RDWR,O_NONBLOCK};

use errno::errno;

extern {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[derive(Clone,Debug)]
pub struct DeviceError {
    pub errno: c_int,
    message: String
}

impl DeviceError {
    fn new() -> DeviceError {
        let error = errno();
        let message = error.to_string();
        DeviceError { errno: error.0, message: message }
    }
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.description(), self.errno)
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        self.message.borrow()
    }
}

pub type DeviceResult<T> = Result<T, DeviceError>;

#[derive(Copy,Clone,Debug)]
pub enum ReadWriteMode {
    ReadOnly,
    ReadWrite
}

#[derive(Copy,Clone,Debug)]
pub enum BlockMode {
    Blocking,
    NonBlocking
}

#[derive(Debug)]
pub struct DeviceFileDescriptor {
    pub fd: c_int
}

impl DeviceFileDescriptor {
    pub fn open(file: &Path, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<DeviceFileDescriptor> {
        let mode = match rw_mode {
            ReadWriteMode::ReadOnly => O_RDONLY,
            ReadWriteMode::ReadWrite => O_RDWR
        };
        let blocking = match block_mode {
            BlockMode::Blocking => 0,
            BlockMode::NonBlocking => O_NONBLOCK
        };
        let file_string = file.to_string_lossy().into_owned();
        let c_filename = CString::new(file_string).unwrap();
        let fd = unsafe {
            open(c_filename.as_ptr(), mode | blocking, 0)
        };
        if fd < 0 {
            Err(DeviceError::new())
        }
        else {
            Ok(DeviceFileDescriptor { fd: fd })
        }
    }

    pub fn ioctl_argumentless(&self, request: c_ulong) -> DeviceResult<()> {
        let result = unsafe {
            ioctl(self.fd, request)
        };
        make_result(result)
    }

    pub fn ioctl_pointer<T>(&self, request: c_ulong, argument: *mut T) -> DeviceResult<()> {
        let result = unsafe {
            ioctl(self.fd, request, argument)
        };
        //println!("[DEBUG] ioctl(fd: {}, request: {}, argument: [size {} bytes]) -> {}", self.fd, request, size_of::<T>(), result);
        make_result(result)
    }

    pub fn ioctl_value<T: Copy>(&self, request: c_ulong, argument: T) -> DeviceResult<()> {
        let result = unsafe {
            ioctl(self.fd, request, argument)
        };
        //println!("[DEBUG] ioctl(fd: {}, request: {}, argument: [size {} bytes]) -> {}", fd, request, size_of::<T>(), result);
        make_result(result)
    }

    pub fn read(&self, buffer: &mut [u8]) -> DeviceResult<isize> {
        let result = unsafe {
            let ptr = buffer.as_mut_ptr() as *mut c_void;
            read(self.fd, ptr, buffer.len() as size_t)
        };
        if result == -1 {
            Err(DeviceError::new())
        } else {
            Ok(result as isize)
        }
    }

    pub fn write(&self, buffer: &mut [u8]) -> DeviceResult<isize> {
        let result = unsafe {
            let ptr = buffer.as_mut_ptr() as *const c_void;
            write(self.fd, ptr, buffer.len() as size_t)
        };
        if result == -1 {
            Err(DeviceError::new())
        } else {
            Ok(result as isize)
        }
    }
}

impl Drop for DeviceFileDescriptor {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

fn make_result(ioctl_result: c_int) -> DeviceResult<()> {
    if ioctl_result == 0 {
        Ok(())
    }
    else {
        Err(DeviceError::new())
    }
}
