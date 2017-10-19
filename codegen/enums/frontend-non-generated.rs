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

use std::error::Error;
use std::fmt::{self,Display,Formatter};
use std::num::ParseIntError;


#[derive(Copy,Clone,Debug)]
pub enum PropertyMappingError {
    UnrecognizedValue(GetProperty, u32),
    UnrecognizedProperty(u32),
    StatError(GetProperty),
    BufferError(GetProperty)
}

impl Error for PropertyMappingError {
    fn description(&self) -> &str {
        match *self {
            PropertyMappingError::UnrecognizedValue(_, _) => "Unrecognized property value",
            PropertyMappingError::UnrecognizedProperty(_) => "Unrecognized property",
            PropertyMappingError::StatError(_) => "Error interpreting statistical property",
            PropertyMappingError::BufferError(_) => "Error interpreting buffer property"
        }
    }
}

impl Display for PropertyMappingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub type PropertyMappingResult<T> = Result<T, PropertyMappingError>;


pub trait FromProperty : Sized {
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self>;
}

impl FromProperty for u32 {
    #[allow(unused_variables)]
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {
        let mut property = property;
        let value: u32 = unsafe { *(property.u.data()) };
        Ok(value)
    }
}

impl FromProperty for i32 {
    #[allow(unused_variables)]
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {
        let mut property = property;
        let uvalue: u32 = unsafe { *(property.u.data()) };
        Ok(uvalue as i32)
    }
}


#[derive(Clone,Debug)]
pub enum PropertyValueError {
    ParseIntError(ParseIntError),
    UnrecognizedValue,
    UnrecognizedProperty
}

impl Error for PropertyValueError {
    fn description(&self) -> &str {
        match *self {
            PropertyValueError::UnrecognizedProperty => "Unrecognized property",
            PropertyValueError::UnrecognizedValue => "Unrecognized property value",
            PropertyValueError::ParseIntError(ref error) => Error::description(error)
        }
    }
}

impl Display for PropertyValueError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ParseIntError> for PropertyValueError {
    fn from(err: ParseIntError) -> PropertyValueError {
        PropertyValueError::ParseIntError(err)
    }
}


pub type PropertyValueResult<T> = Result<T, PropertyValueError>;


pub trait IntoPropertyValue : Sized {
    fn into_property_value(value_str: &str) -> PropertyValueResult<Self>;
}

impl IntoPropertyValue for u32 {
    fn into_property_value(value_str: &str) -> PropertyValueResult<Self> {
        u32::from_str_radix(value_str, 10).map_err(PropertyValueError::ParseIntError)
    }
}

impl IntoPropertyValue for i32 {
    fn into_property_value(value_str: &str) -> PropertyValueResult<Self> {
        i32::from_str_radix(value_str, 10).map_err(PropertyValueError::ParseIntError)
    }
}


fn ffi_property_data(property: ffi::Struct_dtv_property) -> u32 {
    unsafe {
        let mut property = property;
        *property.u.data()
    }
}

fn make_ffi_property(cmd: u32, value: u32) -> ffi::Struct_dtv_property {
    let mut p = ffi::Struct_dtv_property { cmd: cmd, ..Default::default() };
    unsafe {
        *p.u.data() = value;
    }
    p
}


#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Lna {
    LnaOff,
    LnaOn,
    LnaAuto,
}

impl Into<u32> for Lna {
    fn into(self) -> u32 {
        match self {
            // On and off variants as defined by Linux DVB documentation
            Lna::LnaOff => 0,
            Lna::LnaOn => 1,
            Lna::LnaAuto => ffi::LNA_AUTO
        }
    }
}

impl FromProperty for Lna {
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {
        match ffi_property_data(property) {
            0 => Ok(Lna::LnaOff),
            1 => Ok(Lna::LnaOn),
            ffi::LNA_AUTO => Ok(Lna::LnaAuto),
            value => Err(PropertyMappingError::UnrecognizedValue(property_name, value))
        }
    }
}

impl IntoPropertyValue for Lna {
    fn into_property_value(value_str: &str) -> PropertyValueResult<Self> {
        match value_str {
            "LnaOff" => Ok(Lna::LnaOff),
            "LnaOn" => Ok(Lna::LnaOn),
            "LnaAuto" => Ok(Lna::LnaAuto),
            _ => Err(PropertyValueError::UnrecognizedValue)
        }
    }
}


#[derive(Copy,Clone,Debug,PartialEq)]
pub enum StatValue {
    ScaleNotAvailable,
    Counter(u64),
    Decibel(i64),
    Relative(u64),
}

const STAT_COUNT: usize = 4;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Stat {
    len: u8,
    stats: [StatValue; STAT_COUNT]
}

impl Stat {
    pub fn stats(&self) -> &[StatValue] {
        &self.stats[0..self.len as usize]
    }
}

impl FromProperty for Stat {
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {
        let (len, ffi_stats) = unsafe {
            let mut property = property;
            ((*property.u.st()).len, (*property.u.st()).stat)
        };
        if len as usize > STAT_COUNT {
            return Err(PropertyMappingError::StatError(property_name));
        }
        let mut stats = [StatValue::ScaleNotAvailable; STAT_COUNT];
        for i in 0..len as usize {
            let mut ffi_stat = ffi_stats[i];
            let uvalue: u64 = unsafe { *ffi_stat.uvalue() };
            let svalue: i64 = unsafe { *ffi_stat.svalue() };
            let scale = ffi_stat.scale as u32;
            stats[i] = match scale {
                ffi::FE_SCALE_NOT_AVAILABLE => StatValue::ScaleNotAvailable,
                ffi::FE_SCALE_COUNTER => StatValue::Counter(uvalue),
                ffi::FE_SCALE_DECIBEL => StatValue::Decibel(svalue),
                ffi::FE_SCALE_RELATIVE => StatValue::Relative(uvalue),
                _ => return Err(PropertyMappingError::StatError(property_name))
            };
        }
        Ok(Stat { len: len, stats: stats })
    }
}


const BUFFER_SIZE: usize = 32;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct SupportedDeliverySystems {
    len: u32,
    delsys: [DeliverySystem; BUFFER_SIZE]
}

impl SupportedDeliverySystems {
    pub fn delivery_systems(&self) -> &[DeliverySystem] {
        &self.delsys[0..self.len as usize]
    }
}

impl FromProperty for SupportedDeliverySystems {
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {
        let (len, buffer) = unsafe {
            let mut property = property;
            ((*property.u.buffer()).len, (*property.u.buffer()).data)
        };
        if len as usize > BUFFER_SIZE {
            return Err(PropertyMappingError::BufferError(property_name));
        }
        let mut delsys = [DeliverySystem::SysUndefined; BUFFER_SIZE];
        for i in 0..len as usize {
            let mut tmp: ffi::Struct_dtv_property = Default::default();
            unsafe { *tmp.u.data() = buffer[i] as u32; }
            delsys[i] = try!(FromProperty::from_property(property_name, tmp));
        }
        Ok(SupportedDeliverySystems { len: len, delsys: delsys })
    }
}


#[derive(Copy,Clone,Debug,PartialEq)]
pub struct ApiVersion {
    pub major: u8,
    pub minor: u8
}

impl FromProperty for ApiVersion {
    #[allow(unused_variables)]
    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {
        let data = ffi_property_data(property);
        let major = (data >> 8) & 0xff;
        let minor = data & 0xff;
        Ok(ApiVersion { major: major as u8, minor: minor as u8 })
    }
}


#[test]
fn ffi_property_generation() {
    let mut property = make_ffi_property(123, 456);
    assert_eq!(property.cmd, 123);
    let data = unsafe { *property.u.data() };
    assert_eq!(data, 456);
}

#[test]
fn str_to_property() {
    let property = make_set_property_value("DtvModulation", "Qam128").unwrap();
    assert_eq!(property, SetPropertyValue::DtvModulation(Modulation::Qam128));
}
