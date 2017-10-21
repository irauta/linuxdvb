// Copyright 2017 Ilkka Rauta
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

#![allow(non_snake_case)]

// This file defines the ioctl command numbers for demux commands. They are
// defined as fns to keep the code similar with frontend command handling.

use libc::c_ulong;
use super::ffi::*;

pub fn DMX_START() -> c_ulong {
    io!('o', 41) as c_ulong
}

pub fn DMX_STOP() -> c_ulong {
    io!('o', 42) as c_ulong
}

pub fn DMX_SET_FILTER() -> c_ulong {
    iow!('o', 43, ::std::mem::size_of::<dmx_sct_filter_params>()) as c_ulong
}

pub fn DMX_SET_PES_FILTER() -> c_ulong {
    iow!('o', 44, ::std::mem::size_of::<dmx_pes_filter_params>()) as c_ulong
}

pub fn DMX_SET_BUFFER_SIZE() -> c_ulong {
    io!('o', 45) as c_ulong
}

pub fn DMX_GET_PES_PIDS() -> c_ulong {
    ior!('o', 47, ::std::mem::size_of::<[__u16; 5]>()) as c_ulong
}

// See parent mod for why these are commented out
//pub fn DMX_GET_CAPS() -> c_ulong {
//    ior!('o', 48, ::std::mem::size_of::<dmx_caps_t>) as c_ulong
//}
//
//pub fn DMX_SET_SOURCE() -> c_ulong {
//    iow!('o', 49, ::std::mem::size_of::<dmx_source_t>) as c_ulong
//}

pub fn DMX_GET_STC() -> c_ulong {
    iowr!('o', 50, ::std::mem::size_of::<dmx_stc>()) as c_ulong
}

pub fn DMX_ADD_PID() -> c_ulong {
    iow!('o', 51, ::std::mem::size_of::<__u16>()) as c_ulong
}

pub fn DMX_REMOVE_PID() -> c_ulong {
    iow!('o', 52, ::std::mem::size_of::<__u16>()) as c_ulong
}
