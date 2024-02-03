roc build ../main.roc --lib --output libapp.so
mv libapp.so.1.0 libapp.so
cargo build
