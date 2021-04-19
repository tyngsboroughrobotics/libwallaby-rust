#!/bin/bash

set -e

rm -rf temp && mkdir temp
rm -rf libwallaby && mkdir libwallaby

cd temp
curl -fsSL https://github.com/kipr/libwallaby/releases/latest/download/libwallaby.deb -o libwallaby.deb
ar -x libwallaby.deb
tar -xf data.tar.xz
cd ..

cp -r temp/usr/* libwallaby
rm -rf temp
