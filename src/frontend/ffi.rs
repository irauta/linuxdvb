/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_winsize {
    pub ws_row: ::libc::c_ushort,
    pub ws_col: ::libc::c_ushort,
    pub ws_xpixel: ::libc::c_ushort,
    pub ws_ypixel: ::libc::c_ushort,
}
impl ::std::clone::Clone for Struct_winsize {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_winsize {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_termio {
    pub c_iflag: ::libc::c_ushort,
    pub c_oflag: ::libc::c_ushort,
    pub c_cflag: ::libc::c_ushort,
    pub c_lflag: ::libc::c_ushort,
    pub c_line: ::libc::c_uchar,
    pub c_cc: [::libc::c_uchar; 8usize],
}
impl ::std::clone::Clone for Struct_termio {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_termio {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __s8 = ::libc::c_char;
pub type __u8 = ::libc::c_uchar;
pub type __s16 = ::libc::c_short;
pub type __u16 = ::libc::c_ushort;
pub type __s32 = ::libc::c_int;
pub type __u32 = ::libc::c_uint;
pub type __s64 = ::libc::c_longlong;
pub type __u64 = ::libc::c_ulonglong;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub fds_bits: [::libc::c_ulong; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __kernel_fd_set = Struct_Unnamed1;
pub type __kernel_sighandler_t =
    ::std::option::Option<extern "C" fn(arg1: ::libc::c_int) -> ()>;
pub type __kernel_key_t = ::libc::c_int;
pub type __kernel_mqd_t = ::libc::c_int;
pub type __kernel_old_uid_t = ::libc::c_ushort;
pub type __kernel_old_gid_t = ::libc::c_ushort;
pub type __kernel_old_dev_t = ::libc::c_ulong;
pub type __kernel_long_t = ::libc::c_long;
pub type __kernel_ulong_t = ::libc::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::libc::c_uint;
pub type __kernel_pid_t = ::libc::c_int;
pub type __kernel_ipc_pid_t = ::libc::c_int;
pub type __kernel_uid_t = ::libc::c_uint;
pub type __kernel_gid_t = ::libc::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::libc::c_int;
pub type __kernel_uid32_t = ::libc::c_uint;
pub type __kernel_gid32_t = ::libc::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub val: [::libc::c_int; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __kernel_fsid_t = Struct_Unnamed2;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::libc::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::libc::c_int;
pub type __kernel_clockid_t = ::libc::c_int;
pub type __kernel_caddr_t = *mut ::libc::c_char;
pub type __kernel_uid16_t = ::libc::c_ushort;
pub type __kernel_gid16_t = ::libc::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type Enum_fe_type = ::libc::c_uint;
pub const FE_QPSK: ::libc::c_uint = 0;
pub const FE_QAM: ::libc::c_uint = 1;
pub const FE_OFDM: ::libc::c_uint = 2;
pub const FE_ATSC: ::libc::c_uint = 3;
pub type fe_type_t = Enum_fe_type;
pub type Enum_fe_caps = ::libc::c_uint;
pub const FE_IS_STUPID: ::libc::c_uint = 0;
pub const FE_CAN_INVERSION_AUTO: ::libc::c_uint = 1;
pub const FE_CAN_FEC_1_2: ::libc::c_uint = 2;
pub const FE_CAN_FEC_2_3: ::libc::c_uint = 4;
pub const FE_CAN_FEC_3_4: ::libc::c_uint = 8;
pub const FE_CAN_FEC_4_5: ::libc::c_uint = 16;
pub const FE_CAN_FEC_5_6: ::libc::c_uint = 32;
pub const FE_CAN_FEC_6_7: ::libc::c_uint = 64;
pub const FE_CAN_FEC_7_8: ::libc::c_uint = 128;
pub const FE_CAN_FEC_8_9: ::libc::c_uint = 256;
pub const FE_CAN_FEC_AUTO: ::libc::c_uint = 512;
pub const FE_CAN_QPSK: ::libc::c_uint = 1024;
pub const FE_CAN_QAM_16: ::libc::c_uint = 2048;
pub const FE_CAN_QAM_32: ::libc::c_uint = 4096;
pub const FE_CAN_QAM_64: ::libc::c_uint = 8192;
pub const FE_CAN_QAM_128: ::libc::c_uint = 16384;
pub const FE_CAN_QAM_256: ::libc::c_uint = 32768;
pub const FE_CAN_QAM_AUTO: ::libc::c_uint = 65536;
pub const FE_CAN_TRANSMISSION_MODE_AUTO: ::libc::c_uint = 131072;
pub const FE_CAN_BANDWIDTH_AUTO: ::libc::c_uint = 262144;
pub const FE_CAN_GUARD_INTERVAL_AUTO: ::libc::c_uint = 524288;
pub const FE_CAN_HIERARCHY_AUTO: ::libc::c_uint = 1048576;
pub const FE_CAN_8VSB: ::libc::c_uint = 2097152;
pub const FE_CAN_16VSB: ::libc::c_uint = 4194304;
pub const FE_HAS_EXTENDED_CAPS: ::libc::c_uint = 8388608;
pub const FE_CAN_MULTISTREAM: ::libc::c_uint = 67108864;
pub const FE_CAN_TURBO_FEC: ::libc::c_uint = 134217728;
pub const FE_CAN_2G_MODULATION: ::libc::c_uint = 268435456;
pub const FE_NEEDS_BENDING: ::libc::c_uint = 536870912;
pub const FE_CAN_RECOVER: ::libc::c_uint = 1073741824;
pub const FE_CAN_MUTE_TS: ::libc::c_uint = -2147483648i32 as ::libc::c_uint;
pub type fe_caps_t = Enum_fe_caps;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_frontend_info {
    pub name: [::libc::c_char; 128usize],
    pub _type: fe_type_t,
    pub frequency_min: __u32,
    pub frequency_max: __u32,
    pub frequency_stepsize: __u32,
    pub frequency_tolerance: __u32,
    pub symbol_rate_min: __u32,
    pub symbol_rate_max: __u32,
    pub symbol_rate_tolerance: __u32,
    pub notifier_delay: __u32,
    pub caps: fe_caps_t,
}
impl ::std::clone::Clone for Struct_dvb_frontend_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_frontend_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_diseqc_master_cmd {
    pub msg: [__u8; 6usize],
    pub msg_len: __u8,
}
impl ::std::clone::Clone for Struct_dvb_diseqc_master_cmd {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_diseqc_master_cmd {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_diseqc_slave_reply {
    pub msg: [__u8; 4usize],
    pub msg_len: __u8,
    pub timeout: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_dvb_diseqc_slave_reply {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_diseqc_slave_reply {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_fe_sec_voltage = ::libc::c_uint;
pub const SEC_VOLTAGE_13: ::libc::c_uint = 0;
pub const SEC_VOLTAGE_18: ::libc::c_uint = 1;
pub const SEC_VOLTAGE_OFF: ::libc::c_uint = 2;
pub type fe_sec_voltage_t = Enum_fe_sec_voltage;
pub type Enum_fe_sec_tone_mode = ::libc::c_uint;
pub const SEC_TONE_ON: ::libc::c_uint = 0;
pub const SEC_TONE_OFF: ::libc::c_uint = 1;
pub type fe_sec_tone_mode_t = Enum_fe_sec_tone_mode;
pub type Enum_fe_sec_mini_cmd = ::libc::c_uint;
pub const SEC_MINI_A: ::libc::c_uint = 0;
pub const SEC_MINI_B: ::libc::c_uint = 1;
pub type fe_sec_mini_cmd_t = Enum_fe_sec_mini_cmd;
pub type Enum_fe_status = ::libc::c_uint;
pub const FE_HAS_SIGNAL: ::libc::c_uint = 1;
pub const FE_HAS_CARRIER: ::libc::c_uint = 2;
pub const FE_HAS_VITERBI: ::libc::c_uint = 4;
pub const FE_HAS_SYNC: ::libc::c_uint = 8;
pub const FE_HAS_LOCK: ::libc::c_uint = 16;
pub const FE_TIMEDOUT: ::libc::c_uint = 32;
pub const FE_REINIT: ::libc::c_uint = 64;
pub type fe_status_t = Enum_fe_status;
pub type Enum_fe_spectral_inversion = ::libc::c_uint;
pub const INVERSION_OFF: ::libc::c_uint = 0;
pub const INVERSION_ON: ::libc::c_uint = 1;
pub const INVERSION_AUTO: ::libc::c_uint = 2;
pub type fe_spectral_inversion_t = Enum_fe_spectral_inversion;
pub type Enum_fe_code_rate = ::libc::c_uint;
pub const FEC_NONE: ::libc::c_uint = 0;
pub const FEC_1_2: ::libc::c_uint = 1;
pub const FEC_2_3: ::libc::c_uint = 2;
pub const FEC_3_4: ::libc::c_uint = 3;
pub const FEC_4_5: ::libc::c_uint = 4;
pub const FEC_5_6: ::libc::c_uint = 5;
pub const FEC_6_7: ::libc::c_uint = 6;
pub const FEC_7_8: ::libc::c_uint = 7;
pub const FEC_8_9: ::libc::c_uint = 8;
pub const FEC_AUTO: ::libc::c_uint = 9;
pub const FEC_3_5: ::libc::c_uint = 10;
pub const FEC_9_10: ::libc::c_uint = 11;
pub const FEC_2_5: ::libc::c_uint = 12;
pub type fe_code_rate_t = Enum_fe_code_rate;
pub type Enum_fe_modulation = ::libc::c_uint;
pub const QPSK: ::libc::c_uint = 0;
pub const QAM_16: ::libc::c_uint = 1;
pub const QAM_32: ::libc::c_uint = 2;
pub const QAM_64: ::libc::c_uint = 3;
pub const QAM_128: ::libc::c_uint = 4;
pub const QAM_256: ::libc::c_uint = 5;
pub const QAM_AUTO: ::libc::c_uint = 6;
pub const VSB_8: ::libc::c_uint = 7;
pub const VSB_16: ::libc::c_uint = 8;
pub const PSK_8: ::libc::c_uint = 9;
pub const APSK_16: ::libc::c_uint = 10;
pub const APSK_32: ::libc::c_uint = 11;
pub const DQPSK: ::libc::c_uint = 12;
pub const QAM_4_NR: ::libc::c_uint = 13;
pub type fe_modulation_t = Enum_fe_modulation;
pub type Enum_fe_transmit_mode = ::libc::c_uint;
pub const TRANSMISSION_MODE_2K: ::libc::c_uint = 0;
pub const TRANSMISSION_MODE_8K: ::libc::c_uint = 1;
pub const TRANSMISSION_MODE_AUTO: ::libc::c_uint = 2;
pub const TRANSMISSION_MODE_4K: ::libc::c_uint = 3;
pub const TRANSMISSION_MODE_1K: ::libc::c_uint = 4;
pub const TRANSMISSION_MODE_16K: ::libc::c_uint = 5;
pub const TRANSMISSION_MODE_32K: ::libc::c_uint = 6;
pub const TRANSMISSION_MODE_C1: ::libc::c_uint = 7;
pub const TRANSMISSION_MODE_C3780: ::libc::c_uint = 8;
pub type fe_transmit_mode_t = Enum_fe_transmit_mode;
pub type Enum_fe_bandwidth = ::libc::c_uint;
pub const BANDWIDTH_8_MHZ: ::libc::c_uint = 0;
pub const BANDWIDTH_7_MHZ: ::libc::c_uint = 1;
pub const BANDWIDTH_6_MHZ: ::libc::c_uint = 2;
pub const BANDWIDTH_AUTO: ::libc::c_uint = 3;
pub const BANDWIDTH_5_MHZ: ::libc::c_uint = 4;
pub const BANDWIDTH_10_MHZ: ::libc::c_uint = 5;
pub const BANDWIDTH_1_712_MHZ: ::libc::c_uint = 6;
pub type fe_bandwidth_t = Enum_fe_bandwidth;
pub type Enum_fe_guard_interval = ::libc::c_uint;
pub const GUARD_INTERVAL_1_32: ::libc::c_uint = 0;
pub const GUARD_INTERVAL_1_16: ::libc::c_uint = 1;
pub const GUARD_INTERVAL_1_8: ::libc::c_uint = 2;
pub const GUARD_INTERVAL_1_4: ::libc::c_uint = 3;
pub const GUARD_INTERVAL_AUTO: ::libc::c_uint = 4;
pub const GUARD_INTERVAL_1_128: ::libc::c_uint = 5;
pub const GUARD_INTERVAL_19_128: ::libc::c_uint = 6;
pub const GUARD_INTERVAL_19_256: ::libc::c_uint = 7;
pub const GUARD_INTERVAL_PN420: ::libc::c_uint = 8;
pub const GUARD_INTERVAL_PN595: ::libc::c_uint = 9;
pub const GUARD_INTERVAL_PN945: ::libc::c_uint = 10;
pub type fe_guard_interval_t = Enum_fe_guard_interval;
pub type Enum_fe_hierarchy = ::libc::c_uint;
pub const HIERARCHY_NONE: ::libc::c_uint = 0;
pub const HIERARCHY_1: ::libc::c_uint = 1;
pub const HIERARCHY_2: ::libc::c_uint = 2;
pub const HIERARCHY_4: ::libc::c_uint = 3;
pub const HIERARCHY_AUTO: ::libc::c_uint = 4;
pub type fe_hierarchy_t = Enum_fe_hierarchy;
pub type Enum_fe_interleaving = ::libc::c_uint;
pub const INTERLEAVING_NONE: ::libc::c_uint = 0;
pub const INTERLEAVING_AUTO: ::libc::c_uint = 1;
pub const INTERLEAVING_240: ::libc::c_uint = 2;
pub const INTERLEAVING_720: ::libc::c_uint = 3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_qpsk_parameters {
    pub symbol_rate: __u32,
    pub fec_inner: fe_code_rate_t,
}
impl ::std::clone::Clone for Struct_dvb_qpsk_parameters {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_qpsk_parameters {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_qam_parameters {
    pub symbol_rate: __u32,
    pub fec_inner: fe_code_rate_t,
    pub modulation: fe_modulation_t,
}
impl ::std::clone::Clone for Struct_dvb_qam_parameters {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_qam_parameters {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_vsb_parameters {
    pub modulation: fe_modulation_t,
}
impl ::std::clone::Clone for Struct_dvb_vsb_parameters {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_vsb_parameters {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_ofdm_parameters {
    pub bandwidth: fe_bandwidth_t,
    pub code_rate_HP: fe_code_rate_t,
    pub code_rate_LP: fe_code_rate_t,
    pub constellation: fe_modulation_t,
    pub transmission_mode: fe_transmit_mode_t,
    pub guard_interval: fe_guard_interval_t,
    pub hierarchy_information: fe_hierarchy_t,
}
impl ::std::clone::Clone for Struct_dvb_ofdm_parameters {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_ofdm_parameters {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_frontend_parameters {
    pub frequency: __u32,
    pub inversion: fe_spectral_inversion_t,
    pub u: Union_Unnamed3,
}
impl ::std::clone::Clone for Struct_dvb_frontend_parameters {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_frontend_parameters {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed3 {
    pub _bindgen_data_: [u32; 7usize],
}
impl Union_Unnamed3 {
    pub unsafe fn qpsk(&mut self) -> *mut Struct_dvb_qpsk_parameters {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn qam(&mut self) -> *mut Struct_dvb_qam_parameters {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ofdm(&mut self) -> *mut Struct_dvb_ofdm_parameters {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn vsb(&mut self) -> *mut Struct_dvb_vsb_parameters {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dvb_frontend_event {
    pub status: fe_status_t,
    pub parameters: Struct_dvb_frontend_parameters,
}
impl ::std::clone::Clone for Struct_dvb_frontend_event {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dvb_frontend_event {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_fe_pilot = ::libc::c_uint;
pub const PILOT_ON: ::libc::c_uint = 0;
pub const PILOT_OFF: ::libc::c_uint = 1;
pub const PILOT_AUTO: ::libc::c_uint = 2;
pub type fe_pilot_t = Enum_fe_pilot;
pub type Enum_fe_rolloff = ::libc::c_uint;
pub const ROLLOFF_35: ::libc::c_uint = 0;
pub const ROLLOFF_20: ::libc::c_uint = 1;
pub const ROLLOFF_25: ::libc::c_uint = 2;
pub const ROLLOFF_AUTO: ::libc::c_uint = 3;
pub type fe_rolloff_t = Enum_fe_rolloff;
pub type Enum_fe_delivery_system = ::libc::c_uint;
pub const SYS_UNDEFINED: ::libc::c_uint = 0;
pub const SYS_DVBC_ANNEX_A: ::libc::c_uint = 1;
pub const SYS_DVBC_ANNEX_B: ::libc::c_uint = 2;
pub const SYS_DVBT: ::libc::c_uint = 3;
pub const SYS_DSS: ::libc::c_uint = 4;
pub const SYS_DVBS: ::libc::c_uint = 5;
pub const SYS_DVBS2: ::libc::c_uint = 6;
pub const SYS_DVBH: ::libc::c_uint = 7;
pub const SYS_ISDBT: ::libc::c_uint = 8;
pub const SYS_ISDBS: ::libc::c_uint = 9;
pub const SYS_ISDBC: ::libc::c_uint = 10;
pub const SYS_ATSC: ::libc::c_uint = 11;
pub const SYS_ATSCMH: ::libc::c_uint = 12;
pub const SYS_DTMB: ::libc::c_uint = 13;
pub const SYS_CMMB: ::libc::c_uint = 14;
pub const SYS_DAB: ::libc::c_uint = 15;
pub const SYS_DVBT2: ::libc::c_uint = 16;
pub const SYS_TURBO: ::libc::c_uint = 17;
pub const SYS_DVBC_ANNEX_C: ::libc::c_uint = 18;
pub type fe_delivery_system_t = Enum_fe_delivery_system;
pub type Enum_atscmh_sccc_block_mode = ::libc::c_uint;
pub const ATSCMH_SCCC_BLK_SEP: ::libc::c_uint = 0;
pub const ATSCMH_SCCC_BLK_COMB: ::libc::c_uint = 1;
pub const ATSCMH_SCCC_BLK_RES: ::libc::c_uint = 2;
pub type Enum_atscmh_sccc_code_mode = ::libc::c_uint;
pub const ATSCMH_SCCC_CODE_HLF: ::libc::c_uint = 0;
pub const ATSCMH_SCCC_CODE_QTR: ::libc::c_uint = 1;
pub const ATSCMH_SCCC_CODE_RES: ::libc::c_uint = 2;
pub type Enum_atscmh_rs_frame_ensemble = ::libc::c_uint;
pub const ATSCMH_RSFRAME_ENS_PRI: ::libc::c_uint = 0;
pub const ATSCMH_RSFRAME_ENS_SEC: ::libc::c_uint = 1;
pub type Enum_atscmh_rs_frame_mode = ::libc::c_uint;
pub const ATSCMH_RSFRAME_PRI_ONLY: ::libc::c_uint = 0;
pub const ATSCMH_RSFRAME_PRI_SEC: ::libc::c_uint = 1;
pub const ATSCMH_RSFRAME_RES: ::libc::c_uint = 2;
pub type Enum_atscmh_rs_code_mode = ::libc::c_uint;
pub const ATSCMH_RSCODE_211_187: ::libc::c_uint = 0;
pub const ATSCMH_RSCODE_223_187: ::libc::c_uint = 1;
pub const ATSCMH_RSCODE_235_187: ::libc::c_uint = 2;
pub const ATSCMH_RSCODE_RES: ::libc::c_uint = 3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dtv_cmds_h {
    pub name: *mut ::libc::c_char,
    pub cmd: __u32,
    pub _bindgen_bitfield_1_: __u32,
    pub _bindgen_bitfield_2_: __u32,
    pub _bindgen_bitfield_3_: __u32,
}
impl ::std::clone::Clone for Struct_dtv_cmds_h {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dtv_cmds_h {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_fecap_scale_params = ::libc::c_uint;
pub const FE_SCALE_NOT_AVAILABLE: ::libc::c_uint = 0;
pub const FE_SCALE_DECIBEL: ::libc::c_uint = 1;
pub const FE_SCALE_RELATIVE: ::libc::c_uint = 2;
pub const FE_SCALE_COUNTER: ::libc::c_uint = 3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dtv_stats {
    pub scale: __u8,
    pub _bindgen_data_1_: [u64; 1usize],
}
impl Struct_dtv_stats {
    pub unsafe fn uvalue(&mut self) -> *mut __u64 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn svalue(&mut self) -> *mut __s64 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Struct_dtv_stats {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dtv_stats {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dtv_fe_stats {
    pub len: __u8,
    pub stat: [Struct_dtv_stats; 4usize],
}
impl ::std::clone::Clone for Struct_dtv_fe_stats {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dtv_fe_stats {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dtv_property {
    pub cmd: __u32,
    pub reserved: [__u32; 3usize],
    pub u: Union_Unnamed4,
    pub result: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_dtv_property {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dtv_property {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed4 {
    pub _bindgen_data_: [u64; 7usize],
}
impl Union_Unnamed4 {
    pub unsafe fn data(&mut self) -> *mut __u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn st(&mut self) -> *mut Struct_dtv_fe_stats {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn buffer(&mut self) -> *mut Struct_Unnamed5 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub data: [__u8; 32usize],
    pub len: __u32,
    pub reserved1: [__u32; 3usize],
    pub reserved2: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_dtv_properties {
    pub num: __u32,
    pub props: *mut Struct_dtv_property,
}
impl ::std::clone::Clone for Struct_dtv_properties {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_dtv_properties {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_fe_rust_bind_defines = ::libc::c_uint;
pub const DTV_UNDEFINED: ::libc::c_uint = 0;
pub const DTV_TUNE: ::libc::c_uint = 1;
pub const DTV_CLEAR: ::libc::c_uint = 2;
pub const DTV_FREQUENCY: ::libc::c_uint = 3;
pub const DTV_MODULATION: ::libc::c_uint = 4;
pub const DTV_BANDWIDTH_HZ: ::libc::c_uint = 5;
pub const DTV_INVERSION: ::libc::c_uint = 6;
pub const DTV_DISEQC_MASTER: ::libc::c_uint = 7;
pub const DTV_SYMBOL_RATE: ::libc::c_uint = 8;
pub const DTV_INNER_FEC: ::libc::c_uint = 9;
pub const DTV_VOLTAGE: ::libc::c_uint = 10;
pub const DTV_TONE: ::libc::c_uint = 11;
pub const DTV_PILOT: ::libc::c_uint = 12;
pub const DTV_ROLLOFF: ::libc::c_uint = 13;
pub const DTV_DISEQC_SLAVE_REPLY: ::libc::c_uint = 14;
pub const DTV_FE_CAPABILITY_COUNT: ::libc::c_uint = 15;
pub const DTV_FE_CAPABILITY: ::libc::c_uint = 16;
pub const DTV_DELIVERY_SYSTEM: ::libc::c_uint = 17;
pub const DTV_ISDBT_PARTIAL_RECEPTION: ::libc::c_uint = 18;
pub const DTV_ISDBT_SOUND_BROADCASTING: ::libc::c_uint = 19;
pub const DTV_ISDBT_SB_SUBCHANNEL_ID: ::libc::c_uint = 20;
pub const DTV_ISDBT_SB_SEGMENT_IDX: ::libc::c_uint = 21;
pub const DTV_ISDBT_SB_SEGMENT_COUNT: ::libc::c_uint = 22;
pub const DTV_ISDBT_LAYERA_FEC: ::libc::c_uint = 23;
pub const DTV_ISDBT_LAYERA_MODULATION: ::libc::c_uint = 24;
pub const DTV_ISDBT_LAYERA_SEGMENT_COUNT: ::libc::c_uint = 25;
pub const DTV_ISDBT_LAYERA_TIME_INTERLEAVING: ::libc::c_uint = 26;
pub const DTV_ISDBT_LAYERB_FEC: ::libc::c_uint = 27;
pub const DTV_ISDBT_LAYERB_MODULATION: ::libc::c_uint = 28;
pub const DTV_ISDBT_LAYERB_SEGMENT_COUNT: ::libc::c_uint = 29;
pub const DTV_ISDBT_LAYERB_TIME_INTERLEAVING: ::libc::c_uint = 30;
pub const DTV_ISDBT_LAYERC_FEC: ::libc::c_uint = 31;
pub const DTV_ISDBT_LAYERC_MODULATION: ::libc::c_uint = 32;
pub const DTV_ISDBT_LAYERC_SEGMENT_COUNT: ::libc::c_uint = 33;
pub const DTV_ISDBT_LAYERC_TIME_INTERLEAVING: ::libc::c_uint = 34;
pub const DTV_API_VERSION: ::libc::c_uint = 35;
pub const DTV_CODE_RATE_HP: ::libc::c_uint = 36;
pub const DTV_CODE_RATE_LP: ::libc::c_uint = 37;
pub const DTV_GUARD_INTERVAL: ::libc::c_uint = 38;
pub const DTV_TRANSMISSION_MODE: ::libc::c_uint = 39;
pub const DTV_HIERARCHY: ::libc::c_uint = 40;
pub const DTV_ISDBT_LAYER_ENABLED: ::libc::c_uint = 41;
pub const DTV_STREAM_ID: ::libc::c_uint = 42;
pub const DTV_ISDBS_TS_ID_LEGACY: ::libc::c_uint = 42;
pub const DTV_DVBT2_PLP_ID_LEGACY: ::libc::c_uint = 43;
pub const DTV_ENUM_DELSYS: ::libc::c_uint = 44;
pub const DTV_ATSCMH_FIC_VER: ::libc::c_uint = 45;
pub const DTV_ATSCMH_PARADE_ID: ::libc::c_uint = 46;
pub const DTV_ATSCMH_NOG: ::libc::c_uint = 47;
pub const DTV_ATSCMH_TNOG: ::libc::c_uint = 48;
pub const DTV_ATSCMH_SGN: ::libc::c_uint = 49;
pub const DTV_ATSCMH_PRC: ::libc::c_uint = 50;
pub const DTV_ATSCMH_RS_FRAME_MODE: ::libc::c_uint = 51;
pub const DTV_ATSCMH_RS_FRAME_ENSEMBLE: ::libc::c_uint = 52;
pub const DTV_ATSCMH_RS_CODE_MODE_PRI: ::libc::c_uint = 53;
pub const DTV_ATSCMH_RS_CODE_MODE_SEC: ::libc::c_uint = 54;
pub const DTV_ATSCMH_SCCC_BLOCK_MODE: ::libc::c_uint = 55;
pub const DTV_ATSCMH_SCCC_CODE_MODE_A: ::libc::c_uint = 56;
pub const DTV_ATSCMH_SCCC_CODE_MODE_B: ::libc::c_uint = 57;
pub const DTV_ATSCMH_SCCC_CODE_MODE_C: ::libc::c_uint = 58;
pub const DTV_ATSCMH_SCCC_CODE_MODE_D: ::libc::c_uint = 59;
pub const DTV_INTERLEAVING: ::libc::c_uint = 60;
pub const DTV_LNA: ::libc::c_uint = 61;
pub const DTV_STAT_SIGNAL_STRENGTH: ::libc::c_uint = 62;
pub const DTV_STAT_CNR: ::libc::c_uint = 63;
pub const DTV_STAT_PRE_ERROR_BIT_COUNT: ::libc::c_uint = 64;
pub const DTV_STAT_PRE_TOTAL_BIT_COUNT: ::libc::c_uint = 65;
pub const DTV_STAT_POST_ERROR_BIT_COUNT: ::libc::c_uint = 66;
pub const DTV_STAT_POST_TOTAL_BIT_COUNT: ::libc::c_uint = 67;
pub const DTV_STAT_ERROR_BLOCK_COUNT: ::libc::c_uint = 68;
pub const DTV_STAT_TOTAL_BLOCK_COUNT: ::libc::c_uint = 69;
pub const DTV_MAX_COMMAND: ::libc::c_uint = 69;
pub const SYS_DVBC_ANNEX_AC: ::libc::c_uint = 1;
pub const SYS_DMBTH: ::libc::c_uint = 13;
pub const NO_STREAM_ID_FILTER: ::libc::c_uint = -1i32 as ::libc::c_uint;
pub const LNA_AUTO: ::libc::c_uint = -1i32 as ::libc::c_uint;
pub const MAX_DTV_STATS: ::libc::c_uint = 4;
pub const DTV_IOCTL_MAX_MSGS: ::libc::c_uint = 64;
pub const FE_SET_PROPERTY: ::libc::c_uint = 1074818898;
pub const FE_GET_PROPERTY: ::libc::c_uint = -2146406573i32 as ::libc::c_uint;
pub const FE_TUNE_MODE_ONESHOT: ::libc::c_uint = 1;
pub const FE_GET_INFO: ::libc::c_uint = -2136445123i32 as ::libc::c_uint;
pub const FE_DISEQC_RESET_OVERLOAD: ::libc::c_uint = 28478;
pub const FE_DISEQC_SEND_MASTER_CMD: ::libc::c_uint = 1074229055;
pub const FE_DISEQC_RECV_SLAVE_REPLY: ::libc::c_uint = -2146668736i32 as ::libc::c_uint;
pub const FE_DISEQC_SEND_BURST: ::libc::c_uint = 28481;
pub const FE_SET_TONE: ::libc::c_uint = 28482;
pub const FE_SET_VOLTAGE: ::libc::c_uint = 28483;
pub const FE_ENABLE_HIGH_LNB_VOLTAGE: ::libc::c_uint = 28484;
pub const FE_READ_STATUS: ::libc::c_uint = -2147193019i32 as ::libc::c_uint;
pub const FE_READ_BER: ::libc::c_uint = -2147193018i32 as ::libc::c_uint;
pub const FE_READ_SIGNAL_STRENGTH: ::libc::c_uint = -2147324089i32 as ::libc::c_uint;
pub const FE_READ_SNR: ::libc::c_uint = -2147324088i32 as ::libc::c_uint;
pub const FE_READ_UNCORRECTED_BLOCKS: ::libc::c_uint = -2147193015i32 as ::libc::c_uint;
pub const FE_SET_FRONTEND: ::libc::c_uint = 1076129612;
pub const FE_GET_FRONTEND: ::libc::c_uint = -2145095859i32 as ::libc::c_uint;
pub const FE_SET_FRONTEND_TUNE_MODE: ::libc::c_uint = 28497;
pub const FE_GET_EVENT: ::libc::c_uint = -2144833714i32 as ::libc::c_uint;
pub const FE_DISHNETWORK_SEND_LEGACY_CMD: ::libc::c_uint = 28496;
pub type fe_rust_bind_defines_t = Enum_fe_rust_bind_defines;
extern "C" {
    pub fn ioctl(__fd: ::libc::c_int, __request: ::libc::c_ulong, ...)
     -> ::libc::c_int;
}
