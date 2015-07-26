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

pub struct VariantInfo<'a> {
    pub ffi_name: &'a str,
    pub formatted: String
}

pub type StringFormatter = Fn(String) -> String;

fn format_variant(snake_text: &str, variant_formatter: Option<&StringFormatter>) -> String {
    match variant_formatter {
        Some(formatter) => formatter(snake_text.into()).to_ascii_lowercase().to_camel(),
        None => snake_text.to_ascii_lowercase().to_camel()
    }
}

pub fn make_variant_info<'a>(variants: &[&'a str], variant_formatter: Option<&StringFormatter>) -> Vec<VariantInfo<'a>> {
    variants.iter().map(|variant| VariantInfo {
        ffi_name: variant,
        formatted: format_variant(variant, variant_formatter)
    }).collect()
}

pub fn make_simple_into(f: &mut File, enum_name: &str, variants: &Vec<VariantInfo>, ffi_mod: &str) {
    writeln!(f, "impl Into<u32> for {} {{", enum_name).unwrap();
    writeln!(f, "    fn into(self) -> u32 {{").unwrap();
    writeln!(f, "        match self {{").unwrap();
    for variant in variants {
        writeln!(f, "            {}::{} => {}::{},", enum_name, variant.formatted, ffi_mod, variant.ffi_name).unwrap();
    }
    writeln!(f, "        }}").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

pub fn make_simple_enum(f: &mut File, enum_name: &str, variants: &Vec<VariantInfo>) {
    writeln!(f, "#[derive(Copy,Clone,Debug)]").unwrap();
    writeln!(f, "pub enum {} {{", enum_name).unwrap();
    for variant in variants {
        writeln!(f, "    {},", variant.formatted).unwrap();
    }
    writeln!(f, "}}").unwrap();
}
