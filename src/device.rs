
use std::error::Error;
use std::fmt::{self,Display,Formatter};
use std::ffi::CString;
use std::borrow::Borrow;

use libc::{c_ulong,c_int};
use libc::{open,close};
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
    pub fn open(filename: &str, rw_mode: ReadWriteMode, block_mode: BlockMode) -> DeviceResult<DeviceFileDescriptor> {
        let mode = match rw_mode {
            ReadWriteMode::ReadOnly => O_RDONLY,
            ReadWriteMode::ReadWrite => O_RDWR
        };
        let blocking = match block_mode {
            BlockMode::Blocking => 0,
            BlockMode::NonBlocking => O_NONBLOCK
        };
        let c_filename = CString::new(filename).unwrap();
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

    pub fn ioctl_pointer<T>(&self, request: c_ulong, argument: &mut T) -> DeviceResult<()> {
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
