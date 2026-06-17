#!/bin/bash

set -euo pipefail

rm -rf build

mkdir -p build/Payload/Library/Application\ Support/org.ChefKiss.DiskDbg
mkdir -p build/Payload/Library/LaunchDaemons
mkdir -p build/bin

cp -a org.ChefKiss.DiskDbg.plist build/Payload/Library/LaunchDaemons/org.ChefKiss.DiskDbg.plist
clang++ -target x86_64-apple-macos10.13 -std=c++17 -O3 -ffast-math -fvisibility=hidden -flto -Wl,-dead_strip DiskDbg.cpp -o build/bin/DiskDbg-x86_64
clang++ -target arm64-apple-macos11.0 -std=c++17 -O3 -ffast-math -fvisibility=hidden -flto -Wl,-dead_strip DiskDbg.cpp -o build/bin/DiskDbg-arm64
strip build/bin/DiskDbg-x86_64
strip build/bin/DiskDbg-arm64
lipo -create build/bin/DiskDbg-x86_64 build/bin/DiskDbg-arm64 -output build/Payload/Library/Application\ Support/org.ChefKiss.DiskDbg/DiskDbg
pkgbuild --root build/Payload --identifier org.ChefKiss.DiskDbg --version 2.0.0 --ownership recommended build/DiskDbg.pkg
