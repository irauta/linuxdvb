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

# This script outputs a C header to stdout, intended to be used with rust-bindgen to generate
# basic data structures and constants. This is very much an ugly hack, specific only to the file
# linux/dvb/frontend.h and may become outdated as the actual header is updated in futre. To atone
# for the hackiness somewhat, I'll try to explain what is going on here. The main reason for this
# scripts existence is that rust-bindgen doesn't generate constants of preprocessor definitions,
# but frontend.h contains a bunch of important definitions.

# How to use this script:
#
#     ./frontend.sh | rust-bindgen -o >(sed -re 's/-[0-9]+/\1i32 as ::libc::c_uint/' > output.rs) -x c -
#
# Pipe it's output to rust-bindgen, which then outputs to some file. The input is read from stdin
# (the last dash, normally it would be the file name). The -x c tells the language - this is
# mandatory as stdin naturally doesn't give a hint of the format/language like file name suffix .h
# would do. The sed command within the >() process substitution part is to make sure negative int
# literals won't make the Rust compiler complain.

# This is the header that we're dealing with
FRONTEND_H=/usr/include/linux/dvb/frontend.h

# This is necessary, because frontend.h uses the _IO* macros (for example FE_GET_INFO)
echo '#include <sys/ioctl.h>'

# Output the header
# Replace all preprocessor definitions (that define some value) with ones that have _RUST_BIND
# appended to their name. Also replace the single instance of MAX_DTV_STATS being *used* in the
# header (all the other cases are just definitions).
sed -re 's/(#define\W+)([A-Z0-9_]+)(.+)/\1\2_RUST_BIND\3/' \
    -e 's/\[MAX_DTV_STATS\]/\[MAX_DTV_STATS_RUST_BIND\]/' $FRONTEND_H

# Define list of named values by creating an enum. The bing generation doesn't output preprocessor
# definitions, so we need to define something more concrete to grab their values.
echo 'typedef enum fe_rust_bind_defines {'

# Grab only the preprocessor definition names and use them as enum labels. Use the _RUST_BIND
# versions as their values, avoiding collisions with the definition and label names.
sed -nre "s/#define +([A-Z0-9_]+)\W.+/\t\1 = \1_RUST_BIND,/p" $FRONTEND_H

echo '} fe_rust_bind_defines_t;'