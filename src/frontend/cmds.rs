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

#![allow(non_snake_case,dead_code)]

// This file defines the ioctl command numbers for frontend commands. They are
// defined as fns, because stable Rust doesn't support const fns at the moment.
// They can't be produced in the codegen phase, because the numbers vary depending
// on the platform (due to 32/64-bit pointers).

use libc::c_ulong;
use super::ffi::*;

pub fn FE_SET_PROPERTY() -> c_ulong {
   iow!('o', 82, ::std::mem::size_of::<dtv_properties>()) as c_ulong
}

pub fn FE_GET_PROPERTY() -> c_ulong {
   ior!('o', 83, ::std::mem::size_of::<dtv_properties>()) as c_ulong
}

pub fn FE_GET_INFO() -> c_ulong {
   ior!('o', 61, ::std::mem::size_of::<dvb_frontend_info>()) as c_ulong
}

pub fn FE_DISEQC_RESET_OVERLOAD() -> c_ulong {
   io!('o', 62) as c_ulong
}

pub fn FE_DISEQC_SEND_MASTER_CMD() -> c_ulong {
   iow!('o', 63, ::std::mem::size_of::<dvb_diseqc_master_cmd>()) as c_ulong
}

pub fn FE_DISEQC_RECV_SLAVE_REPLY() -> c_ulong {
   ior!('o', 64, ::std::mem::size_of::<dvb_diseqc_slave_reply>()) as c_ulong
}

pub fn FE_DISEQC_SEND_BURST() -> c_ulong {
   io!('o', 65)  /* fe_sec_mini_cmd_t */ as c_ulong
}

pub fn FE_SET_TONE() -> c_ulong {
   io!('o', 66)  /* fe_sec_tone_mode_t */ as c_ulong
}

pub fn FE_SET_VOLTAGE() -> c_ulong {
   io!('o', 67)  /* fe_sec_voltage_t */ as c_ulong
}

pub fn FE_ENABLE_HIGH_LNB_VOLTAGE() -> c_ulong {
   io!('o', 68)  /* int */ as c_ulong
}

pub fn FE_READ_STATUS() -> c_ulong {
   ior!('o', 69, ::std::mem::size_of::<fe_status_t>()) as c_ulong
}

pub fn FE_READ_BER() -> c_ulong {
   ior!('o', 70, ::std::mem::size_of::<__u32>()) as c_ulong
}

pub fn FE_READ_SIGNAL_STRENGTH() -> c_ulong {
   ior!('o', 71, ::std::mem::size_of::<__u16>()) as c_ulong
}

pub fn FE_READ_SNR() -> c_ulong {
   ior!('o', 72, ::std::mem::size_of::<__u16>()) as c_ulong
}

pub fn FE_READ_UNCORRECTED_BLOCKS() -> c_ulong {
   ior!('o', 73, ::std::mem::size_of::<__u32>()) as c_ulong
}

pub fn FE_SET_FRONTEND() -> c_ulong {
    iow!('o', 76, ::std::mem::size_of::<dvb_frontend_parameters>()) as c_ulong
}

pub fn FE_GET_FRONTEND() -> c_ulong {
    ior!('o', 77, ::std::mem::size_of::<dvb_frontend_parameters>()) as c_ulong
}

pub fn FE_SET_FRONTEND_TUNE_MODE() -> c_ulong {
    io!('o', 81) /* unsigned int */ as c_ulong
}

pub fn FE_GET_EVENT() -> c_ulong {
    ior!('o', 78, ::std::mem::size_of::<dvb_frontend_event>()) as c_ulong
}

pub fn FE_DISHNETWORK_SEND_LEGACY_CMD() -> c_ulong {
    io!('o', 80) /* unsigned int */ as c_ulong
}
