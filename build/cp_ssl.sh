#!/bin/sh
set -xe

if [ "$(uname -s)" = "Linux" ]; then
    dir=$(pkg-config --static --libs-only-L openssl)
    if [ -z "$(echo $dir)" ]; then
        exit 0
    fi
    dir=$(echo $dir | sed 's/^-L//')
    cp "${dir}/libssl.a" "$OUT_DIR"
    cp "${dir}/libcrypto.a" "$OUT_DIR"
fi
