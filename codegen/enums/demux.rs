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

use std::env;
use std::fs::File;
use std::path::Path;

use super::common::*;

static FFI_MOD: &'static str = "ffi";

fn make_enum(f: &mut File, enum_name: &str, variants: &[&str], ffi_mod: &str) {
    let variant_info = make_variant_info(variants, None);
    make_simple_enum(f, enum_name, &variant_info);
    make_simple_into(f, enum_name, &variant_info, ffi_mod);
}

pub fn generate() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path =  Path::new(&out_dir).join("demux-enums.rs");
    let mut f = File::create(&dest_path).unwrap();

    make_enum(&mut f, "Input", &[
        "DMX_IN_FRONTEND",
        "DMX_IN_DVR"
    ], FFI_MOD);

    make_enum(&mut f, "Output", &[
        "DMX_OUT_DECODER",
        "DMX_OUT_TAP",
        "DMX_OUT_TS_TAP",
        "DMX_OUT_TSDEMUX_TAP"
    ], FFI_MOD);

    make_enum(&mut f, "DemuxSource", &[
        "DMX_SOURCE_FRONT0",
        "DMX_SOURCE_FRONT1",
        "DMX_SOURCE_FRONT2",
        "DMX_SOURCE_FRONT3",
        "DMX_SOURCE_DVR0",
        "DMX_SOURCE_DVR1",
        "DMX_SOURCE_DVR2",
        "DMX_SOURCE_DVR3"
    ], FFI_MOD);

    make_enum(&mut f, "PesType", &[
        "DMX_PES_AUDIO0",
        "DMX_PES_VIDEO0",
        "DMX_PES_TELETEXT0",
        "DMX_PES_SUBTITLE0",
        "DMX_PES_PCR0",
        "DMX_PES_AUDIO1",
        "DMX_PES_VIDEO1",
        "DMX_PES_TELETEXT1",
        "DMX_PES_SUBTITLE1",
        "DMX_PES_PCR1",
        "DMX_PES_AUDIO2",
        "DMX_PES_VIDEO2",
        "DMX_PES_TELETEXT2",
        "DMX_PES_SUBTITLE2",
        "DMX_PES_PCR2",
        "DMX_PES_AUDIO3",
        "DMX_PES_VIDEO3",
        "DMX_PES_TELETEXT3",
        "DMX_PES_SUBTITLE3",
        "DMX_PES_PCR3",
        "DMX_PES_OTHER"
    ], FFI_MOD);
}
