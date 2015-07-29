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

extern crate linuxdvb;

use std::error::Error;
use std::path::Path;

use linuxdvb::{Frontend,ReadWriteMode,BlockMode};
use linuxdvb::properties::GetProperty as GP;

type SimpleResult<T> = Result<T, Box<Error>>;

fn show_frontend_properties(path: &Path) -> SimpleResult<()> {
    let frontend = try!(Frontend::open(path, ReadWriteMode::ReadOnly, BlockMode::NonBlocking));
    let get_properties = [
        GP::DtvApiVersion,
        GP::DtvAtscmhFicVer,
        GP::DtvAtscmhNog,
        GP::DtvAtscmhParadeId,
        GP::DtvAtscmhPrc,
        GP::DtvAtscmhRsCodeModePri,
        GP::DtvAtscmhRsCodeModeSec,
        GP::DtvAtscmhRsFrameEnsemble,
        GP::DtvAtscmhRsFrameMode,
        GP::DtvAtscmhScccBlockMode,
        GP::DtvAtscmhScccCodeModeA,
        GP::DtvAtscmhScccCodeModeB,
        GP::DtvAtscmhScccCodeModeC,
        GP::DtvAtscmhScccCodeModeD,
        GP::DtvAtscmhSgn,
        GP::DtvAtscmhTnog,
        GP::DtvBandwidthHz,
        GP::DtvCodeRateHp,
        GP::DtvCodeRateLp,
        GP::DtvDeliverySystem,
        GP::DtvFrequency,
        GP::DtvGuardInterval,
        GP::DtvHierarchy,
        GP::DtvInnerFec,
        GP::DtvInterleaving,
        GP::DtvInversion,
        GP::DtvIsdbtLayerEnabled,
        GP::DtvIsdbtLayeraFec,
        GP::DtvIsdbtLayeraModulation,
        GP::DtvIsdbtLayeraSegmentCount,
        GP::DtvIsdbtLayeraTimeInterleaving,
        GP::DtvIsdbtLayerbFec,
        GP::DtvIsdbtLayerbModulation,
        GP::DtvIsdbtLayerbSegmentCount,
        GP::DtvIsdbtLayerbTimeInterleaving,
        GP::DtvIsdbtLayercFec,
        GP::DtvIsdbtLayercModulation,
        GP::DtvIsdbtLayercSegmentCount,
        GP::DtvIsdbtLayercTimeInterleaving,
        GP::DtvIsdbtPartialReception,
        GP::DtvIsdbtSbSegmentCount,
        GP::DtvIsdbtSbSegmentIdx,
        GP::DtvIsdbtSbSubchannelId,
        GP::DtvIsdbtSoundBroadcasting,
        GP::DtvLna,
        GP::DtvModulation,
        GP::DtvPilot,
        GP::DtvRolloff,
        GP::DtvStatCnr,
        GP::DtvStatErrorBlockCount,
        GP::DtvStatPostErrorBitCount,
        GP::DtvStatPostTotalBitCount,
        GP::DtvStatPreErrorBitCount,
        GP::DtvStatPreTotalBitCount,
        GP::DtvStatSignalStrength,
        GP::DtvStatTotalBlockCount,
        GP::DtvStreamId,
        GP::DtvSymbolRate,
        GP::DtvTone,
        GP::DtvTransmissionMode,
        GP::DtvVoltage,
        GP::DtvEnumDelsys,
    ];
    let properties = try!(frontend.get_properties(&get_properties));
    println!("{:#?}", properties);
    Ok(())
}


fn main() {
    let string_path = match std::env::args().nth(1) {
        Some(string_path) => string_path,
        None => {
            println!("Specify frontend device");
            return;
        }
    };
    if let Err(error) = show_frontend_properties(Path::new(&string_path)) {
        println!("Error: {:?}", error);
    };
}
