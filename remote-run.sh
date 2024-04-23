#!/bin/bash

BinPath=target/aarch64-unknown-linux-gnu/debug
BinName=lcd-demo
PWD=`pwd`


cargo build
scp $PWD/$BinPath/$BinName pi:/tmp/$BinName
ssh pi /tmp/$BinName