#!/bin/sh
set -xe

${CC:=cc} -o main main.c -Wall "$@"
