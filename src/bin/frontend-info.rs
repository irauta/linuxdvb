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

extern crate linuxdvb;

use std::fs;
use std::error::Error;
use std::path::Path;

use linuxdvb::{Frontend,ReadWriteMode,BlockMode};
use linuxdvb::properties::GetProperty as GP;
use linuxdvb::properties::GetPropertyValue as GPV;

type SimpleResult = Result<(), Box<Error>>;

fn show_frontend_info(path: &Path) -> SimpleResult {
    let frontend = Frontend::open("/dev/dvb/adapter0/frontend0", ReadWriteMode::ReadOnly, BlockMode::NonBlocking).unwrap();
    println!("Device: {}", path.to_string_lossy());
    let info = try!(frontend.get_info());
    println!("Name: {}", info.name);

    let properties = try!(frontend.get_properties(&[
        GP::DtvEnumDelsys
        ]));
    if let GPV::DtvEnumDelsys(delsys) = properties[0] {
        println!("Supported delivery systems: {:?}", delsys.delivery_systems());
    }

    println!("Minimal frequency: {}", info.frequency_min);
    println!("Maximal frequency: {}", info.frequency_max);
    println!("Frequency step: {}", info.frequency_stepsize);
    println!("Frequency tolerance: {}", info.frequency_tolerance);
    println!("Minimal symbol rate: {}", info.symbol_rate_min);
    println!("Maximal symbol rate: {}", info.symbol_rate_max);
    println!("Maximal symbol rate tolerance: {}", info.symbol_rate_tolerance);

    let caps: Vec<linuxdvb::caps::FrontendCapsEnum> = info.caps.into();
    println!("Frontend capabilities: {:?}", caps);

    Ok(())
}

fn iterate_devices(adapter_dir: &Path) -> SimpleResult {
    for device in try!(fs::read_dir(adapter_dir)) {
        let device = try!(device);
        let is_frontend = device.path().file_name()
            .map(|f| f.to_str())
            .unwrap_or(None)
            .map_or(false, |f| f.starts_with("frontend"));
        if is_frontend {
            try!(show_frontend_info(&device.path()));
        }
    }
    Ok(())
}

fn iterate_adapters() -> SimpleResult {
    for adapter_dir in try!(fs::read_dir("/dev/dvb")) {
        let adapter_dir = try!(adapter_dir);
        try!(iterate_devices(&adapter_dir.path()));
    }
    Ok(())
}

fn main() {
    match iterate_adapters() {
        Ok(_) => (),
        Err(error) => println!("Error: {}", error.description())
    }
}