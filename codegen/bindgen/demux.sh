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

# See frontend.sh for information on what is going on here

DMX_H=/usr/include/linux/dvb/dmx.h

echo '#include <sys/ioctl.h>'

sed -re 's/(#define\W+)([A-Z0-9_]+)(.+)/\1\2_RUST_BIND\3/' \
    -e 's/\[DMX_FILTER_SIZE\]/\[DMX_FILTER_SIZE_RUST_BIND\]/' $DMX_H

echo 'typedef enum fe_rust_bind_defines {'

sed -nre "s/#define +([A-Z0-9_]+)\W.+/\t\1 = \1_RUST_BIND,/p" $DMX_H

echo '} fe_rust_bind_defines_t;'