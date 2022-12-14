#!/bin/sh

day() {
    echo "\033[0;32mDAY $1\033[0m"
    cd $1
}
cleanup() {
    cd ..
    echo
}

day 01
    ./main.lua input.txt
cleanup

day 02
    ./build.sh && ./main input.txt
cleanup

day 03
    go run main.go input.txt
cleanup

day 04
    ./main.py input.txt
cleanup

day 05
    ./main.lua input.txt
cleanup

day 06
    ./build.sh && ./main input.txt
cleanup

day 07
    cargo run input.txt
cleanup

day 08
    cargo run input.txt
cleanup

day 09
    ./build.sh && ./main input.txt
cleanup

day 10
    ./main.lua input.txt
cleanup

day 11
    cargo run input.txt
cleanup

day 12
    cargo run --release input.txt
cleanup

day 13
    cargo run input.txt
cleanup

day 14
    cargo run input.txt
cleanup

day 15
    cargo run --release input.txt
cleanup

day 16
    ./build.sh "-O3" && ./main input.txt
cleanup
