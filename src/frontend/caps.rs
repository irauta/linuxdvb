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

#![allow(dead_code,non_upper_case_globals)]

use super::ffi as ffi;

macro_rules! frontend_caps (
    ({
        $($variant:ident => $value:path),+
    }) => (
        bitflags!(
            pub struct FrontendCaps: u32 {
                $(const $variant = $value;)+
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
                    if !FrontendCaps::$variant.is_empty() && self.contains(FrontendCaps::$variant) {
                        caps.push(FrontendCapsEnum::$variant);
                    }
                )+
                caps
            }
        }
    )
);

frontend_caps!({
    IsStupid => ffi::FE_IS_STUPID,
    CanInversionAuto => ffi::FE_CAN_INVERSION_AUTO,
    CanFec12 => ffi::FE_CAN_FEC_1_2,
    CanFec23 => ffi::FE_CAN_FEC_2_3,
    CanFec34 => ffi::FE_CAN_FEC_3_4,
    CanFec45 => ffi::FE_CAN_FEC_4_5,
    CanFec56 => ffi::FE_CAN_FEC_5_6,
    CanFec67 => ffi::FE_CAN_FEC_6_7,
    CanFec78 => ffi::FE_CAN_FEC_7_8,
    CanFec89 => ffi::FE_CAN_FEC_8_9,
    CanFecAuto => ffi::FE_CAN_FEC_AUTO,
    CanQpsk => ffi::FE_CAN_QPSK,
    CanQam16 => ffi::FE_CAN_QAM_16,
    CanQam32 => ffi::FE_CAN_QAM_32,
    CanQam64 => ffi::FE_CAN_QAM_64,
    CanQam128 => ffi::FE_CAN_QAM_128,
    CanQam256 => ffi::FE_CAN_QAM_256,
    CanQamAuto => ffi::FE_CAN_QAM_AUTO,
    CanTransmissionModeAuto => ffi::FE_CAN_TRANSMISSION_MODE_AUTO,
    CanBandwidthAuto => ffi::FE_CAN_BANDWIDTH_AUTO,
    CanGuardIntervalAuto => ffi::FE_CAN_GUARD_INTERVAL_AUTO,
    CanHierarchyAuto => ffi::FE_CAN_HIERARCHY_AUTO,
    Can8vsb => ffi::FE_CAN_8VSB,
    Can16vsb => ffi::FE_CAN_16VSB,
    HasExtendedCaps => ffi::FE_HAS_EXTENDED_CAPS,
    CanMultistream => ffi::FE_CAN_MULTISTREAM,
    CanTurboFec => ffi::FE_CAN_TURBO_FEC,
    Can2gModulation => ffi::FE_CAN_2G_MODULATION,
    NeedsBending => ffi::FE_NEEDS_BENDING,
    CanRecover => ffi::FE_CAN_RECOVER,
    CanMuteTs => ffi::FE_CAN_MUTE_TS
});
