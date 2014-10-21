#!/bin/sh
set -xe

cp_static () {
    pkg-config $1 || exit 0

    dir=$(pkg-config --static --libs-only-L $1)
    if [ -z "$(echo $dir)" ]; then
        exit 0
    fi
    dir=$(echo $dir | sed 's/^-L//')

    if [ -f "${dir}/$2" ]; then
        cp "${dir}/$2" "$OUT_DIR"
    fi
}

if [ "$(uname -s)" = "Linux" ]; then
    cp_static openssl libssl.a
    cp_static libcrypto libcrypto.a
fi
