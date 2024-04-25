#!/bin/bash

BinPath=target/aarch64-unknown-linux-gnu/debug
BinName=lcd-demo
PWD=`pwd`


cargo build
if [ $? -ne 0 ]; then
    echo "Build failed"
    exit 1
fi
scp $PWD/$BinPath/$BinName pi:/tmp/$BinName
ssh pi /tmp/$BinName