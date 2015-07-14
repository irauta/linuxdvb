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

extern crate case;

use std::fs::File;
use std::io::Write;

use self::case::CaseExt;
use std::ascii::AsciiExt;

pub fn pascal_case(snake_text: &str) -> String {
    snake_text.to_ascii_lowercase().to_camel()
}

pub fn make_simple_into(f: &mut File, enum_name: &str, variants: &[&str], ffi_mod: &str) {
    writeln!(f, "impl Into<u32> for {} {{", enum_name).unwrap();
    writeln!(f, "    fn into(self) -> u32 {{").unwrap();
    writeln!(f, "        match self {{").unwrap();
    for ffi_name in variants {
        let name = pascal_case(ffi_name);
        writeln!(f, "            {}::{} => {}::{},", enum_name, name, ffi_mod, ffi_name).unwrap();
    }
    writeln!(f, "        }}").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

pub fn make_simple_enum(f: &mut File, enum_name: &str, variants: &[&str]) {
    writeln!(f, "#[derive(Copy,Clone,Debug)]").unwrap();
    writeln!(f, "pub enum {} {{", enum_name).unwrap();
    for ffi_name in variants {
        let name = pascal_case(ffi_name);
        writeln!(f, "    {},", name).unwrap();
    }
    writeln!(f, "}}").unwrap();
}
