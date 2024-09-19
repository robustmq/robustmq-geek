#!/bin/sh
# Copyright 2023 RobustMQ Team
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

start_placement_center(){
    nohup cargo run --package cmd --bin $placement_center_process_name -- --conf=tests/config/$placement_center_process_name.toml >/dev/null 2>&1 &
    sleep 3
    while ! ps aux | grep -v grep | grep "$placement_center_process_name" > /dev/null; do
        echo "Process $placement_center_process_name has not started yet, wait 1s...."
        sleep 1  
    done
    echo "Process $placement_center_process_name starts successfully and starts running the test case"
}

stop_placement_center(){
    pc_no=`ps aux | grep -v grep | grep "$placement_center_process_name" | awk '{print $2}'`
    echo "placement center num: $pc_no"
    kill $pc_no
    sleep 3

    while ps aux | grep -v grep | grep "$placement_center_process_name" > /dev/null; do
        echo "”Process $placement_center_process_name stopped successfully"
        sleep 1  
    done
}

start_placement_center


# Run Cargo Test
cargo test

if [ $? -ne 0 ]; then
    echo "Test case failed to run"
    stop_placement_center
    exit 1
else
    echo "Test case runs successfully"
    stop_placement_center
fi

