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

# It also needs to be present in these two places when compiling the rust code
cp libapp.so.1 platform/libapp.so
mkdir -p platform/target/debug
cp libapp.so.1 platform/target/debug/libapp.so.1

cd platform
cargo lambda build --release --lambda-dir ..
rm libapp.so

# Cleanup
cd ..
mv host/bootstrap bootstrap
rm -r host
rm -f bootstrap.zip

# Create the archive which will be uploaded to AWS
zip bootstrap.zip bootstrap libapp.so.1

# Cleanup
rm bootstrap
rm libapp.so.1
