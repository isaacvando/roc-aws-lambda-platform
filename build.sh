#!/bin/bash
set -uo pipefail

if [ -z "$1" ]; then
    echo "Error: No file name provided."
    echo "Usage: ./build.sh <file_name>"
    exit 1
fi

roc build $1 --lib --output libapp.so --target linux-x64

roc_exit_code=$?
# Exit code 2 means there were warnings but no errors
if [[ $roc_exit_code -ne 0 && $roc_exit_code -ne 2 ]]; then
    exit $roc_exit_code
fi

# We need to include libapp.so.1 in the final zip archive
mv libapp.so.1.0 libapp.so.1

# It also needs to be present here when compiling the rust code
cp libapp.so.1 platform/libapp.so

cd platform
RUSTFLAGS="-C link-args=-rdynamic" cargo build --release --target x86_64-unknown-linux-gnu
cargo_exit_code=$?
if [ $cargo_exit_code -ne 0 ]; then
    exit $cargo_exit_code
fi
rm libapp.so
cp target/release/host ../bootstrap

# Create the archive which will be uploaded to AWS
cd ..
rm -f bootstrap.zip
zip bootstrap.zip bootstrap libapp.so.1

# Cleanup
rm bootstrap
rm libapp.so.1
