roc build ../main.roc --lib --output libapp.so

# Why does it append 1.0 to the output?
mv libapp.so.1.0 libapp.so

mkdir -p target/debug

cp libapp.so target/debug/libapp.so.1

cargo lambda build --release --lambda-dir .

# Add libapp.so to the bootstrap.zip file

cd target/debug
zip -r ../lambda/host/bootstrap.zip libapp.so.1
