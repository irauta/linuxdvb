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

type SimpleResult<T> = Result<T, Box<Error>>;

fn show_frontend_info(path: &Path, yet_another_one: bool) -> SimpleResult<()> {
    let frontend = try!(Frontend::open(path, ReadWriteMode::ReadOnly, BlockMode::NonBlocking));
    if yet_another_one {
        println!("--");
    }
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

    let status = try!(frontend.read_status());
    println!("Status:");
    println!("    Has a signal: {}", status.has_signal);
    println!("    Has DVB signal: {}", status.has_carrier);
    println!("    Inner coding is stable: {}", status.has_viterbi);
    println!("    Synchronization bytes found: {}", status.has_sync);
    println!("    Has DVB lock: {}", status.has_lock);
    println!("    Timed out: {}", status.timedout);
    println!("    Frontend was reinitialized: {}", status.reinit);

    Ok(())
}

fn iterate_devices(adapter_dir: &Path, yet_another_one: bool) -> SimpleResult<bool> {
    let mut has_frontend = false;
    for device in try!(fs::read_dir(adapter_dir)) {
        let device = try!(device);
        let is_frontend = device.path().file_name()
            .map(|f| f.to_str())
            .unwrap_or(None)
            .map_or(false, |f| f.starts_with("frontend"));
        if is_frontend {
            try!(show_frontend_info(&device.path(), yet_another_one));
            has_frontend = true;
        }
    }
    Ok(has_frontend)
}

fn iterate_adapters() -> SimpleResult<bool> {
    let mut has_frontend = false;
    for adapter_dir in try!(fs::read_dir("/dev/dvb")) {
        let adapter_dir = try!(adapter_dir);
        let is_frontend = try!(iterate_devices(&adapter_dir.path(), has_frontend));
        has_frontend = is_frontend || has_frontend;
    }
    Ok(has_frontend)
}

fn main() {
    let has_frontend = match iterate_adapters() {
        Ok(has_frontend) => has_frontend,
        Err(error) => {
            println!("Error: {}", error.description());
            return;
        }
    };
    if !has_frontend {
        println!("No DVB frontend devices were found");
    }
}