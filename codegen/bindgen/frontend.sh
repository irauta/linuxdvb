#!/bin/bash

# Copyright 2015 Ilkka Rauta
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# To update the ffi.rs for frontend functionality, run this script.
# You need to have bindgen installed; to install it, run
#     cargo install bindgen

# Get the path to this script
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# This is the header that we're dealing with
FRONTEND_H=/usr/include/linux/dvb/frontend.h

# Output file
OUTPUT="$DIR/../../src/frontend/ffi.rs"

# --with-derive-default because Default::default() is used in a couple places
# --constified-enum and --no-prepend-enum-name are used because that matches
# the older version of bindgen outputed the enum varients as bare constants, so
# basically to avoid code breakage after updating to new bindgen.
bindgen -o "$OUTPUT" --with-derive-default --constified-enum='.+' --no-prepend-enum-name "$FRONTEND_H"
