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
    ./main.lua
cleanup

day 02
    ./run.sh
cleanup

day 03
    go run main.go
cleanup

day 04
    ./main.py
cleanup
