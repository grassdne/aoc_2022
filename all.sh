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

day 08
    cargo run input.txt
cleanup
