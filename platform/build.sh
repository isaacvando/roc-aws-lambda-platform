roc build ../main.roc --lib --output libapp.so

# Why does it append 1.0 to the output?
mv libapp.so.1.0 libapp.so

cp libapp.so target/debug/libapp.so.1

cargo lambda build --release

# Add libapp.so to the bootstrap.zip file

cd target/debug
zip -r ../lambda/host/bootstrap.zip libapp.so.1
