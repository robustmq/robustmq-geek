#!/bin/sh
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

