#!/bin/bash
set -euo pipefail

if [ -z "$1" ]; then
    echo "Error: No file name provided."
    echo "Usage: ./build.sh <file_name>"
    exit 1
fi

roc build $1 --lib --output libapp.so --target linux-x64

# We need to include libapp.so.1 in the final zip archive
mv libapp.so.1.0 libapp.so.1

# It also needs to be present here when compiling the rust code
cp libapp.so.1 platform/libapp.so

cd platform
RUSTFLAGS="-C link-args=-rdynamic" cargo build --release
rm libapp.so
cp target/release/host ../bootstrap

# Create the archive which will be uploaded to AWS
cd ..
rm -f bootstrap.zip
zip bootstrap.zip bootstrap libapp.so.1

# Cleanup
rm bootstrap
rm libapp.so.1
