#!/bin/bash

sudo apt update
sudo apt install -y gcc-10-x86-64-linux-gnu libc6-dev-amd64-i386-cross

echo 'export QEMU_LD_PREFIX=/usr/x86_64-linux-gnu' >> ~/.bashrc
echo 'export CC=/usr/bin/x86_64-linux-gnu-gcc-10' >> ~/.bashrc
