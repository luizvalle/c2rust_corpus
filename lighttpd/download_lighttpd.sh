#!/bin/bash
ROOT_DIR=/tmp/lighttpd
rm -rf $ROOT_DIR
git clone git@github.com:lighttpd/lighttpd1.4.git $ROOT_DIR
cd $ROOT_DIR
sudo apt install autoconf automake libtool m4 pkg-config
./autogen.sh
./configure -C
make check
make clean
