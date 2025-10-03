#!/bin/bash

CXX=g++
CXXFLAGS="-Wall -Wextra -O2 -std=c++20"

LDLIBS="-L. -ltdtp"
TDTPLIB="libtdtp.a"
TDTP_BUILD_DIR="../target/release"

SRCS="client.cxx"
OUT="client.a"

function build() {
    echo "Building..."

    if [ ! -f $TDTPLIB ]; then
        echo "$TDTPLIB not found. Attempting to copy from build directory..."
        if [ ! -f "$TDTP_BUILD_DIR/$TDTPLIB" ]; then
            echo "$TDTP_BUILD_DIR not found. Attempting to build and then copy."
            cargo b --release --package tdtp
            if [ $? -ne 0 ]; then
                echo "Library build failed. Exiting."
                exit 1
            fi
        fi
        cp "$TDTP_BUILD_DIR/$TDTPLIB" .
    fi

    $CXX $CXXFLAGS $SRCS -o $OUT $LDLIBS

    if [ $? -ne 0 ]; then
        echo "Build failed"
        exit 1
    fi
};

function run() {
    build
    echo "Running..."
    ./$OUT
};

function clean() {
    echo "Cleaning..."
    rm -f $OUT
};

case $1 in
    build)
        build
        ;;
    run)
        run
        ;;
    clean)
        clean
        ;;
    *)
        echo "Error: Usage: $0 {build|run|clean}"
        exit 1
        ;;
esac

echo "Done."
