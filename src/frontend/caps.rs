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

bitflags!(
      flags FrontendCaps: u32 {
            const IsStupid = ffi::FE_IS_STUPID,
            const CanInversionAuto = ffi::FE_CAN_INVERSION_AUTO,
            const CanFec12 = ffi::FE_CAN_FEC_1_2,
            const CanFec23 = ffi::FE_CAN_FEC_2_3,
            const CanFec34 = ffi::FE_CAN_FEC_3_4,
            const CanFec45 = ffi::FE_CAN_FEC_4_5,
            const CanFec56 = ffi::FE_CAN_FEC_5_6,
            const CanFec67 = ffi::FE_CAN_FEC_6_7,
            const CanFec78 = ffi::FE_CAN_FEC_7_8,
            const CanFec89 = ffi::FE_CAN_FEC_8_9,
            const CanFecAuto = ffi::FE_CAN_FEC_AUTO,
            const CanQpsk = ffi::FE_CAN_QPSK,
            const CanQam16 = ffi::FE_CAN_QAM_16,
            const CanQam32 = ffi::FE_CAN_QAM_32,
            const CanQam64 = ffi::FE_CAN_QAM_64,
            const CanQam128 = ffi::FE_CAN_QAM_128,
            const CanQam256 = ffi::FE_CAN_QAM_256,
            const CanQamAuto = ffi::FE_CAN_QAM_AUTO,
            const CanTransmissionModeAuto = ffi::FE_CAN_TRANSMISSION_MODE_AUTO,
            const CanBandwidthAuto = ffi::FE_CAN_BANDWIDTH_AUTO,
            const CanGuardIntervalAuto = ffi::FE_CAN_GUARD_INTERVAL_AUTO,
            const CanHierarchyAuto = ffi::FE_CAN_HIERARCHY_AUTO,
            const Can8vsb = ffi::FE_CAN_8VSB,
            const Can16vsb = ffi::FE_CAN_16VSB,
            const HasExtendedCaps = ffi::FE_HAS_EXTENDED_CAPS,
            const CanMultistream = ffi::FE_CAN_MULTISTREAM,
            const CanTurboFec = ffi::FE_CAN_TURBO_FEC,
            const Can2gModulation = ffi::FE_CAN_2G_MODULATION,
            const NeedsBending = ffi::FE_NEEDS_BENDING,
            const CanRecover = ffi::FE_CAN_RECOVER,
            const CanMuteTs = ffi::FE_CAN_MUTE_TS,
      }
);