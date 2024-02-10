set -euo pipefail

roc build main.roc --lib --output libapp.so

mv libapp.so.1.0 libapp.so.1

cp libapp.so.1 platform/libapp.so

mkdir -p platform/target/debug
cp libapp.so.1 platform/target/debug/libapp.so.1

cd platform
cargo lambda build --release --lambda-dir ..
rm libapp.so

cd ..
mv host/bootstrap bootstrap
rm -r host
rm -f bootstrap.zip

zip bootstrap.zip bootstrap libapp.so.1

rm bootstrap
rm libapp.so.1
