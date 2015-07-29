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

extern crate regex;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use self::regex::Regex;

use super::common::*;

static FFI_MOD: &'static str = "ffi";

// From the big getter switch in kernel code
// drivers/media/dvb-core/dvb_frontend.c: dtv_property_process_set
static GET_PROPERTIES: &'static [&'static str] = &[
    "DTV_API_VERSION",
    "DTV_ATSCMH_FIC_VER",
    "DTV_ATSCMH_NOG",
    "DTV_ATSCMH_PARADE_ID",
    "DTV_ATSCMH_PRC",
    "DTV_ATSCMH_RS_CODE_MODE_PRI",
    "DTV_ATSCMH_RS_CODE_MODE_SEC",
    "DTV_ATSCMH_RS_FRAME_ENSEMBLE",
    "DTV_ATSCMH_RS_FRAME_MODE",
    "DTV_ATSCMH_SCCC_BLOCK_MODE",
    "DTV_ATSCMH_SCCC_CODE_MODE_A",
    "DTV_ATSCMH_SCCC_CODE_MODE_B",
    "DTV_ATSCMH_SCCC_CODE_MODE_C",
    "DTV_ATSCMH_SCCC_CODE_MODE_D",
    "DTV_ATSCMH_SGN",
    "DTV_ATSCMH_TNOG",
    "DTV_BANDWIDTH_HZ",
    "DTV_CODE_RATE_HP",
    "DTV_CODE_RATE_LP",
    "DTV_DELIVERY_SYSTEM",
    "DTV_FREQUENCY",
    "DTV_GUARD_INTERVAL",
    "DTV_HIERARCHY",
    "DTV_INNER_FEC",
    "DTV_INTERLEAVING",
    "DTV_INVERSION",
    "DTV_ISDBT_LAYER_ENABLED",
    "DTV_ISDBT_LAYERA_FEC",
    "DTV_ISDBT_LAYERA_MODULATION",
    "DTV_ISDBT_LAYERA_SEGMENT_COUNT",
    "DTV_ISDBT_LAYERA_TIME_INTERLEAVING",
    "DTV_ISDBT_LAYERB_FEC",
    "DTV_ISDBT_LAYERB_MODULATION",
    "DTV_ISDBT_LAYERB_SEGMENT_COUNT",
    "DTV_ISDBT_LAYERB_TIME_INTERLEAVING",
    "DTV_ISDBT_LAYERC_FEC",
    "DTV_ISDBT_LAYERC_MODULATION",
    "DTV_ISDBT_LAYERC_SEGMENT_COUNT",
    "DTV_ISDBT_LAYERC_TIME_INTERLEAVING",
    "DTV_ISDBT_PARTIAL_RECEPTION",
    "DTV_ISDBT_SB_SEGMENT_COUNT",
    "DTV_ISDBT_SB_SEGMENT_IDX",
    "DTV_ISDBT_SB_SUBCHANNEL_ID",
    "DTV_ISDBT_SOUND_BROADCASTING",
    "DTV_LNA",
    "DTV_MODULATION",
    "DTV_PILOT",
    "DTV_ROLLOFF",
    "DTV_STAT_CNR",
    "DTV_STAT_ERROR_BLOCK_COUNT",
    "DTV_STAT_POST_ERROR_BIT_COUNT",
    "DTV_STAT_POST_TOTAL_BIT_COUNT",
    "DTV_STAT_PRE_ERROR_BIT_COUNT",
    "DTV_STAT_PRE_TOTAL_BIT_COUNT",
    "DTV_STAT_SIGNAL_STRENGTH",
    "DTV_STAT_TOTAL_BLOCK_COUNT",
    "DTV_STREAM_ID",
    "DTV_SYMBOL_RATE",
    "DTV_TONE",
    "DTV_TRANSMISSION_MODE",
    "DTV_VOLTAGE",
    "DTV_ENUM_DELSYS",
];

// From the big setter switch in kernel code
// drivers/media/dvb-core/dvb_frontend.c: dtv_property_process_set
static SET_PROPERTIES: &'static [&'static str] = &[
    "DTV_ATSCMH_PARADE_ID",
    "DTV_ATSCMH_RS_FRAME_ENSEMBLE",
    "DTV_BANDWIDTH_HZ",
    "DTV_CODE_RATE_HP",
    "DTV_CODE_RATE_LP",
    "DTV_DELIVERY_SYSTEM",
    // "DTV_DVBT2_PLP_ID_LEGACY",
    "DTV_FREQUENCY",
    "DTV_GUARD_INTERVAL",
    "DTV_HIERARCHY",
    "DTV_INNER_FEC",
    "DTV_INTERLEAVING",
    "DTV_INVERSION",
    "DTV_ISDBT_LAYER_ENABLED",
    "DTV_ISDBT_LAYERA_FEC",
    "DTV_ISDBT_LAYERA_MODULATION",
    "DTV_ISDBT_LAYERA_SEGMENT_COUNT",
    "DTV_ISDBT_LAYERA_TIME_INTERLEAVING",
    "DTV_ISDBT_LAYERB_FEC",
    "DTV_ISDBT_LAYERB_MODULATION",
    "DTV_ISDBT_LAYERB_SEGMENT_COUNT",
    "DTV_ISDBT_LAYERB_TIME_INTERLEAVING",
    "DTV_ISDBT_LAYERC_FEC",
    "DTV_ISDBT_LAYERC_MODULATION",
    "DTV_ISDBT_LAYERC_SEGMENT_COUNT",
    "DTV_ISDBT_LAYERC_TIME_INTERLEAVING",
    "DTV_ISDBT_PARTIAL_RECEPTION",
    "DTV_ISDBT_SB_SEGMENT_COUNT",
    "DTV_ISDBT_SB_SEGMENT_IDX",
    "DTV_ISDBT_SB_SUBCHANNEL_ID",
    "DTV_ISDBT_SOUND_BROADCASTING",
    "DTV_LNA",
    "DTV_MODULATION",
    "DTV_PILOT",
    "DTV_ROLLOFF",
    "DTV_STREAM_ID",
    "DTV_SYMBOL_RATE",
    "DTV_TONE",
    "DTV_TRANSMISSION_MODE",
    "DTV_TUNE",
    "DTV_VOLTAGE",
    "DTV_CLEAR",
];

type PropertyType = (&'static str, &'static str);

static PROPERTIES: &'static [PropertyType] = &[
    ("DTV_API_VERSION", "ApiVersion"),
    ("DTV_ATSCMH_FIC_VER", "u32"),
    ("DTV_ATSCMH_NOG", "u32"),
    ("DTV_ATSCMH_PARADE_ID", "u32"),
    ("DTV_ATSCMH_PRC", "u32"),
    ("DTV_ATSCMH_RS_CODE_MODE_PRI", "AtscmhRsCodeMode"),
    ("DTV_ATSCMH_RS_CODE_MODE_SEC", "AtscmhRsCodeMode"),
    ("DTV_ATSCMH_RS_FRAME_ENSEMBLE", "AtscmhRsFrameEnsemble"),
    ("DTV_ATSCMH_RS_FRAME_MODE", "AtscmhRsFrameMode"),
    ("DTV_ATSCMH_SCCC_BLOCK_MODE", "AtscmhScccBlockMode"),
    ("DTV_ATSCMH_SCCC_CODE_MODE_A", "AtscmhScccCodeMode"),
    ("DTV_ATSCMH_SCCC_CODE_MODE_B", "AtscmhScccCodeMode"),
    ("DTV_ATSCMH_SCCC_CODE_MODE_C", "AtscmhScccCodeMode"),
    ("DTV_ATSCMH_SCCC_CODE_MODE_D", "AtscmhScccCodeMode"),
    ("DTV_ATSCMH_SGN", "u32"),
    ("DTV_ATSCMH_TNOG", "u32"),
    ("DTV_BANDWIDTH_HZ", "u32"),
    ("DTV_CLEAR", ""),
    ("DTV_CODE_RATE_HP", "CodeRate"),
    ("DTV_CODE_RATE_LP", "CodeRate"),
    ("DTV_DELIVERY_SYSTEM", "DeliverySystem"),
    // ("DTV_DVBT2_PLP_ID_LEGACY", "u32"),
    ("DTV_ENUM_DELSYS", "SupportedDeliverySystems"),
    ("DTV_FREQUENCY", "u32"),
    ("DTV_GUARD_INTERVAL", "GuardInterval"),
    ("DTV_HIERARCHY", "Hierarchy"),
    ("DTV_INNER_FEC", "CodeRate"),
    ("DTV_INTERLEAVING", "Interleaving"),
    ("DTV_INVERSION", "SpectralInversion"),
    ("DTV_ISDBT_LAYER_ENABLED", "u32"),
    ("DTV_ISDBT_LAYERA_FEC", "IsdbtCodeRate"),
    ("DTV_ISDBT_LAYERA_MODULATION", "IsdbtModulation"),
    ("DTV_ISDBT_LAYERA_SEGMENT_COUNT", "i32"),
    ("DTV_ISDBT_LAYERA_TIME_INTERLEAVING", "i32"),
    ("DTV_ISDBT_LAYERB_FEC", "IsdbtCodeRate"),
    ("DTV_ISDBT_LAYERB_MODULATION", "IsdbtModulation"),
    ("DTV_ISDBT_LAYERB_SEGMENT_COUNT", "i32"),
    ("DTV_ISDBT_LAYERB_TIME_INTERLEAVING", "i32"),
    ("DTV_ISDBT_LAYERC_FEC", "IsdbtCodeRate"),
    ("DTV_ISDBT_LAYERC_MODULATION", "IsdbtModulation"),
    ("DTV_ISDBT_LAYERC_SEGMENT_COUNT", "i32"),
    ("DTV_ISDBT_LAYERC_TIME_INTERLEAVING", "i32"),
    ("DTV_ISDBT_PARTIAL_RECEPTION", "i32"),
    ("DTV_ISDBT_SB_SEGMENT_COUNT", "u32"),
    ("DTV_ISDBT_SB_SEGMENT_IDX", "u32"),
    ("DTV_ISDBT_SB_SUBCHANNEL_ID", "i32"),
    ("DTV_ISDBT_SOUND_BROADCASTING", "i32"),
    ("DTV_LNA", "Lna"),
    ("DTV_MODULATION", "Modulation"),
    ("DTV_PILOT", "Pilot"),
    ("DTV_ROLLOFF", "Rolloff"),
    ("DTV_STAT_CNR", "u32"),
    ("DTV_STAT_ERROR_BLOCK_COUNT", "u32"),
    ("DTV_STAT_POST_ERROR_BIT_COUNT", "u32"),
    ("DTV_STAT_POST_TOTAL_BIT_COUNT", "u32"),
    ("DTV_STAT_PRE_ERROR_BIT_COUNT", "u32"),
    ("DTV_STAT_PRE_TOTAL_BIT_COUNT", "u32"),
    ("DTV_STAT_SIGNAL_STRENGTH", "u32"),
    ("DTV_STAT_TOTAL_BLOCK_COUNT", "u32"),
    ("DTV_STREAM_ID", "u32"),
    ("DTV_SYMBOL_RATE", "u32"),
    ("DTV_TONE", "u32"),
    ("DTV_TRANSMISSION_MODE", "TransmitMode"),
    ("DTV_TUNE", ""),
    ("DTV_VOLTAGE", "Voltage"),
];

pub fn make_simple_from_property(f: &mut File, enum_name: &str, variants: &Vec<VariantInfo>) {
    writeln!(f, "impl FromProperty for {} {{", enum_name).unwrap();
    writeln!(f, "    fn from_property(property_name: GetProperty, property: ffi::Struct_dtv_property) -> PropertyMappingResult<Self> {{").unwrap();
    writeln!(f, "        match ffi_property_data(property) {{").unwrap();
    for variant in variants {
        writeln!(f, "            {}::{} => Ok({}::{}),", FFI_MOD, variant.ffi_name, enum_name, variant.formatted).unwrap();
    }
    writeln!(f, "            value => Err(PropertyMappingError::UnrecognizedValue(property_name, value))").unwrap();
    writeln!(f, "        }}").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

fn make_property_value_enum(f: &mut File, enum_name: &str, variants: &Vec<VariantInfo>, types: &[PropertyType], include_empty: bool) {
    writeln!(f, "#[derive(Copy,Clone,Debug,PartialEq)]").unwrap();
    writeln!(f, "pub enum {} {{", enum_name).unwrap();
    for variant in variants {
        let variant_info = types.iter().find(|t| t.0 == variant.ffi_name).unwrap();
        writeln!(f, "    /// {}", variant.ffi_name).unwrap();
        // There's no point in having a "value enum" when there's no type => no value to be carried
        if !variant_info.1.is_empty() {
            writeln!(f, "    {}({}),", variant.formatted, variant_info.1).unwrap();
        } else if include_empty {
            writeln!(f, "    {},", variant.formatted).unwrap();
        }
    }
    writeln!(f, "}}").unwrap();
}

fn make_property_value_getter_fn(f: &mut File, fn_name: &str, enum_name: &str, variants: &Vec<VariantInfo>, types: &[PropertyType]) {
    writeln!(f, "pub fn {}(property: ffi::Struct_dtv_property) -> PropertyMappingResult<{}> {{", fn_name, enum_name).unwrap();
    writeln!(f, "    match property.cmd {{").unwrap();
    for variant in variants {
        let variant_info = types.iter().find(|t| t.0 == variant.ffi_name).unwrap();
        if variant_info.1.is_empty() {
            writeln!(f, "        {}::{} => Ok({}::{}),", FFI_MOD, variant.ffi_name, enum_name, variant.formatted).unwrap();
        } else {
            writeln!(f, "        {}::{} => FromProperty::from_property(GetProperty::{}, property).map(|p| {}::{}(p)),", FFI_MOD, variant.ffi_name, variant.formatted, enum_name, variant.formatted).unwrap();
        }
    }
    writeln!(f, "        value => Err(PropertyMappingError::UnrecognizedProperty(value))").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

fn make_property_value_setter_fn(f: &mut File, fn_name: &str, enum_name: &str, variants: &Vec<VariantInfo>, types: &[PropertyType]) {
    writeln!(f, "pub fn {}(property: &{}) -> ffi::Struct_dtv_property {{", fn_name, enum_name).unwrap();
    writeln!(f, "    match *property {{").unwrap();
    for variant in variants {
        let variant_info = types.iter().find(|t| t.0 == variant.ffi_name).unwrap();
        if variant_info.1.is_empty() {
            writeln!(f, "        {}::{} => make_ffi_property({}::{}, 0),", enum_name, variant.formatted, FFI_MOD, variant.ffi_name).unwrap();
        } else if variant_info.1 == "i32" {
            writeln!(f, "        {}::{}(value) => make_ffi_property({}::{}, value as u32),", enum_name, variant.formatted, FFI_MOD, variant.ffi_name).unwrap();
        } else {
            writeln!(f, "        {}::{}(value) => make_ffi_property({}::{}, value.into()),", enum_name, variant.formatted, FFI_MOD, variant.ffi_name).unwrap();
        }
    }
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

fn make_property_from_str_fn(f: &mut File, fn_name: &str, enum_name: &str, variants: &Vec<VariantInfo>, types: &[PropertyType]) {
    writeln!(f, "pub fn {}(property: &str, parameter: &str) -> PropertyValueResult<{}> {{", fn_name, enum_name).unwrap();
    writeln!(f, "    match property {{").unwrap();
    for variant in variants {
        let variant_info = types.iter().find(|t| t.0 == variant.ffi_name).unwrap();
        if !variant_info.1.is_empty() {
            writeln!(f, "        \"{}\" => Ok({}::{}(try!(IntoPropertyValue::into_property_value(parameter)))),", variant.formatted, enum_name, variant.formatted).unwrap();
        }
    }
    writeln!(f, "        _ => Err(PropertyValueError::UnrecognizedProperty)").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

fn make_into_property_value(f: &mut File, enum_name: &str, variants: &Vec<VariantInfo>) {
    writeln!(f, "impl IntoPropertyValue for {} {{", enum_name).unwrap();
    writeln!(f, "    fn into_property_value(value_str: &str) -> PropertyValueResult<Self> {{").unwrap();
    writeln!(f, "        match value_str {{").unwrap();
    for variant in variants {
        writeln!(f, "            \"{}\" => Ok({}::{}),", variant.formatted, enum_name, variant.formatted).unwrap();
    }
    writeln!(f, "            _ => Err(PropertyValueError::UnrecognizedValue)").unwrap();
    writeln!(f, "        }}").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

fn make_enum(f: &mut File, enum_name: &str, variants: &[&str], variant_formatter: Option<&StringFormatter>) {
    let variant_info = make_variant_info(variants, variant_formatter);
    make_simple_enum(f, enum_name, &variant_info);
    make_simple_into(f, enum_name, &variant_info, FFI_MOD);
    make_simple_from_property(f, enum_name, &variant_info);
    make_into_property_value(f, enum_name, &variant_info);
}

fn make_property_enums(f: &mut File) {
    let get_variant_info = make_variant_info(GET_PROPERTIES, None);
    make_simple_enum(f, "GetProperty", &get_variant_info);
    make_simple_into(f, "GetProperty", &get_variant_info, FFI_MOD);

    let set_variant_info = make_variant_info(SET_PROPERTIES, None);
    make_property_value_enum(f, "GetPropertyValue", &get_variant_info, PROPERTIES, false);
    make_property_value_getter_fn(f, "get_property_value", "GetPropertyValue", &get_variant_info, PROPERTIES);
    make_property_value_enum(f, "SetPropertyValue", &set_variant_info, PROPERTIES, true);
    make_property_value_setter_fn(f, "set_property_value", "SetPropertyValue", &set_variant_info, PROPERTIES);
    make_property_from_str_fn(f, "make_set_property_value", "SetPropertyValue", &set_variant_info, PROPERTIES);
}

pub fn generate() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path =  Path::new(&out_dir).join("frontend-enums.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write(include_bytes!("frontend-non-generated.rs")).unwrap();

    let re = Regex::new(r"(\d+)_(\d+)").unwrap();
    let fec_re = re.clone();
    let rscode_re = re.clone();
    let gi_re = re.clone();
    let fix_fec = move |name: String| fec_re.replace_all(&name, "K$1_N$2");
    let fix_rscode = move |name: String| rscode_re.replace_all(&name, "N$1_K$2");
    let fix_gi = move |name: String| gi_re.replace_all(&name, "D$1_TU$2");

    // fe_modulation ->
    make_enum(&mut f, "Modulation", &[
        "QPSK",
        "QAM_16",
        "QAM_32",
        "QAM_64",
        "QAM_128",
        "QAM_256",
        "QAM_AUTO",
        "VSB_8",
        "VSB_16",
        "PSK_8",
        "APSK_16",
        "APSK_32",
        "DQPSK",
        "QAM_4_NR",
    ], None);

    // fe_spectral_inversion ->
    make_enum(&mut f, "SpectralInversion", &[
        "INVERSION_OFF",
        "INVERSION_ON",
        "INVERSION_AUTO",
    ], None);

    //fe_code_rate ->
    make_enum(&mut f, "CodeRate", &[
        "FEC_NONE",
        "FEC_AUTO",
        "FEC_1_2",
        "FEC_2_3",
        "FEC_3_4",
        "FEC_4_5",
        "FEC_5_6",
        "FEC_6_7",
        "FEC_7_8",
        "FEC_8_9",
        "FEC_9_10",
        "FEC_2_5",
        "FEC_3_5",
    ], Some(&fix_fec));

    // fe_sec_voltage ->
    make_enum(&mut f, "Voltage", &[
        "SEC_VOLTAGE_13",
        "SEC_VOLTAGE_18",
        "SEC_VOLTAGE_OFF",
    ], None);

    // fe_pilot_t ->
    make_enum(&mut f, "Pilot", &[
        "PILOT_ON",
        "PILOT_OFF",
        "PILOT_AUTO",
    ], None);

    // fe_rolloff_t ->
    make_enum(&mut f, "Rolloff", &[
        "ROLLOFF_35",
        "ROLLOFF_20",
        "ROLLOFF_25",
        "ROLLOFF_AUTO",
    ], None);

    // fe_delivery_system_t ->
    make_enum(&mut f, "DeliverySystem", &[
        "SYS_UNDEFINED",
        "SYS_DVBC_ANNEX_A",
        "SYS_DVBC_ANNEX_B",
        "SYS_DVBT",
        "SYS_DSS",
        "SYS_DVBS",
        "SYS_DVBS2",
        "SYS_DVBH",
        "SYS_ISDBT",
        "SYS_ISDBS",
        "SYS_ISDBC",
        "SYS_ATSC",
        "SYS_ATSCMH",
        "SYS_DTMB",
        "SYS_CMMB",
        "SYS_DAB",
        "SYS_DVBT2",
        "SYS_TURBO",
        "SYS_DVBC_ANNEX_C",
    ], None);

    // fe_code_rate ->
    make_enum(&mut f, "IsdbtCodeRate", &[
        "FEC_AUTO",
        "FEC_1_2",
        "FEC_2_3",
        "FEC_3_4",
        "FEC_5_6",
        "FEC_7_8",
    ], Some(&fix_fec));

    // ->
    make_enum(&mut f, "IsdbtModulation", &[
        "QAM_AUTO",
        "QPSK",
        "QAM_16",
        "QAM_64",
        "DQPSK",
    ], None);

    // Enum_atscmh_rs_frame_mode ->
    make_enum(&mut f, "AtscmhRsFrameMode", &[
        "ATSCMH_RSFRAME_PRI_ONLY",
        "ATSCMH_RSFRAME_PRI_SEC",
    ], None);

    // Enum_atscmh_rs_frame_ensemble ->
    make_enum(&mut f, "AtscmhRsFrameEnsemble", &[
        "ATSCMH_RSFRAME_ENS_PRI",
        "ATSCMH_RSFRAME_ENS_SEC",
    ], None);

    // Enum_atscmh_rs_code_mode ->
    make_enum(&mut f, "AtscmhRsCodeMode", &[
        "ATSCMH_RSCODE_211_187",
        "ATSCMH_RSCODE_223_187",
        "ATSCMH_RSCODE_235_187",
    ], Some(&fix_rscode));

    // Enum_atscmh_sccc_block_mode ->
    make_enum(&mut f, "AtscmhScccBlockMode", &[
        "ATSCMH_SCCC_BLK_SEP",
        "ATSCMH_SCCC_BLK_COMB",
    ], None);

    // Enum_atscmh_sccc_code_mode ->
    make_enum(&mut f, "AtscmhScccCodeMode", &[
        "ATSCMH_SCCC_CODE_HLF",
        "ATSCMH_SCCC_CODE_QTR",
    ], None);

    // Enum_fe_transmit_mode ->
    make_enum(&mut f, "TransmitMode", &[
        "TRANSMISSION_MODE_2K",
        "TRANSMISSION_MODE_8K",
        "TRANSMISSION_MODE_AUTO",
        "TRANSMISSION_MODE_4K",
        "TRANSMISSION_MODE_1K",
        "TRANSMISSION_MODE_16K",
        "TRANSMISSION_MODE_32K",
        "TRANSMISSION_MODE_C1",
        "TRANSMISSION_MODE_C3780",
    ], None);

    // Enum_fe_guard_interval ->
    make_enum(&mut f, "GuardInterval", &[
        "GUARD_INTERVAL_1_32",
        "GUARD_INTERVAL_1_16",
        "GUARD_INTERVAL_1_8",
        "GUARD_INTERVAL_1_4",
        "GUARD_INTERVAL_AUTO",
        "GUARD_INTERVAL_1_128",
        "GUARD_INTERVAL_19_128",
        "GUARD_INTERVAL_19_256",
        "GUARD_INTERVAL_PN420",
        "GUARD_INTERVAL_PN595",
        "GUARD_INTERVAL_PN945",
    ], Some(&fix_gi));

    // Enum_fe_hierarchy ->
    make_enum(&mut f, "Hierarchy", &[
        "HIERARCHY_NONE",
        "HIERARCHY_1",
        "HIERARCHY_2",
        "HIERARCHY_4",
        "HIERARCHY_AUTO",
    ], None);

    // Enum_fe_interleaving ->
    make_enum(&mut f, "Interleaving", &[
        "INTERLEAVING_NONE",
        "INTERLEAVING_AUTO",
        "INTERLEAVING_240",
        "INTERLEAVING_720",
    ], None);

    make_property_enums(&mut f);
}
