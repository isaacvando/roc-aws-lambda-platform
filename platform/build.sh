roc build ../main.roc --lib --output libapp.so

# Why does it append 1.0 to the output?
mv libapp.so.1.0 libapp.so

cargo build

mv libapp.so target/debug/libapp.so.1

cargo run
