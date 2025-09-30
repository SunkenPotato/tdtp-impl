cargo b --release
cp ../../target/release/libtdtp.a .
cp ../bindings.h libtdtp.h

for f in *.cxx; do g++ "$f" -o "${f%.cxx}" -L. -ltdtp; done
