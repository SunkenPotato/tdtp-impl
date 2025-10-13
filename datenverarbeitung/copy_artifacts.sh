#!/bin/bash

ROOT_DIR=`git rev-parse --show-toplevel`

echo "Rebuilding tdtp..."
cargo b --release

echo "Copying artifacts from root directory $ROOT_DIR"

cp $ROOT_DIR/target/release/libtdtp.a $ROOT_DIR/datenverarbeitung/lib/libtdtp.a
cp $ROOT_DIR/tdtp/bindings.h $ROOT_DIR/datenverarbeitung/include/libtdtp.h

echo "Done"
