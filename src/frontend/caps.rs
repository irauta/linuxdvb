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

use super::ffi as ffi;

macro_rules! frontend_caps (
    ({
        $($variant:ident : $flag_name:ident => $value:ident),+
    }) => (
        bitflags!(
            pub struct FrontendCaps: u32 {
                $(const $flag_name = ffi::$value;)+
            }
        );
        #[derive(Debug,Copy,Clone)]
        pub enum FrontendCapsEnum {
            $($variant),+
        }
        impl Into<Vec<FrontendCapsEnum>> for FrontendCaps {
            fn into(self) -> Vec<FrontendCapsEnum> {
                let mut caps = Vec::new();
                if self.is_empty() {
                    return caps;
                }
                $(
                    if !FrontendCaps::$flag_name.is_empty() && self.contains(FrontendCaps::$flag_name) {
                        caps.push(FrontendCapsEnum::$variant);
                    }
                )+
                caps
            }
        }
    )
);

frontend_caps!({
    IsStupid : IS_STUPID => FE_IS_STUPID,
    CanInversionAuto : CAN_INVERSION_AUTO => FE_CAN_INVERSION_AUTO,
    CanFec12 : CAN_FEC_1_2 => FE_CAN_FEC_1_2,
    CanFec23 : CAN_FEC_2_3 => FE_CAN_FEC_2_3,
    CanFec34 : CAN_FEC_3_4 => FE_CAN_FEC_3_4,
    CanFec45 : CAN_FEC_4_5 => FE_CAN_FEC_4_5,
    CanFec56 : CAN_FEC_5_6 => FE_CAN_FEC_5_6,
    CanFec67 : CAN_FEC_6_7 => FE_CAN_FEC_6_7,
    CanFec78 : CAN_FEC_7_8 => FE_CAN_FEC_7_8,
    CanFec89 : CAN_FEC_8_9 => FE_CAN_FEC_8_9,
    CanFecAuto : CAN_FEC_AUTO => FE_CAN_FEC_AUTO,
    CanQpsk : CAN_QPSK => FE_CAN_QPSK,
    CanQam16 : CAN_QAM_16 => FE_CAN_QAM_16,
    CanQam32 : CAN_QAM_32 => FE_CAN_QAM_32,
    CanQam64 : CAN_QAM_64 => FE_CAN_QAM_64,
    CanQam128 : CAN_QAM_128 => FE_CAN_QAM_128,
    CanQam256 : CAN_QAM_256 => FE_CAN_QAM_256,
    CanQamAuto : CAN_QAM_AUTO => FE_CAN_QAM_AUTO,
    CanTransmissionModeAuto : CAN_TRANSMISSION_MODE_AUTO => FE_CAN_TRANSMISSION_MODE_AUTO,
    CanBandwidthAuto : CAN_BANDWIDTH_AUTO => FE_CAN_BANDWIDTH_AUTO,
    CanGuardIntervalAuto : CAN_GUARD_INTERVAL_AUTO => FE_CAN_GUARD_INTERVAL_AUTO,
    CanHierarchyAuto : CAN_HIERARCHY_AUTO => FE_CAN_HIERARCHY_AUTO,
    Can8vsb : CAN_8VSB => FE_CAN_8VSB,
    Can16vsb : CAN_16VSB => FE_CAN_16VSB,
    HasExtendedCaps : HAS_EXTENDED_CAPS => FE_HAS_EXTENDED_CAPS,
    CanMultistream : CAN_MULTISTREAM => FE_CAN_MULTISTREAM,
    CanTurboFec : CAN_TURBO_FEC => FE_CAN_TURBO_FEC,
    Can2gModulation : CAN_2G_MODULATION => FE_CAN_2G_MODULATION,
    NeedsBending : NEEDS_BENDING => FE_NEEDS_BENDING,
    CanRecover : CAN_RECOVER => FE_CAN_RECOVER,
    CanMuteTs : CAN_MUTE_TS => FE_CAN_MUTE_TS
});
