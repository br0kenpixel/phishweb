#!/bin/sh

TARGETS=("x86_64-unknown-linux-gnu" "x86_64-unknown-linux-musl" "x86_64-pc-windows-gnu" "aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl")
CROSS_TARGETS=("aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl")
BIN_NAME="phishweb"
UPX_FLAGS="--best --lzma"

rm -rf releases/
mkdir releases/

for target in ${TARGETS[@]}; do
    if [[ ${CROSS_TARGETS[@]} =~ $target ]]; then
        cross build --target=$target --release
    else
        cargo build --target=$target --release
    fi

    if [ -f target/$target/release/$BIN_NAME ]; then
        mv target/$target/release/$BIN_NAME releases/$BIN_NAME-$target

        if [ -x "$(command -v upx)" ]; then
            cp releases/$BIN_NAME-$target releases/$BIN_NAME-$target-compressed

            if ! upx $UPX_FLAGS releases/$BIN_NAME-$target-compressed; then
                rm releases/$BIN_NAME-$target-compressed
            fi
        fi
    fi

    if [ -f target/$target/release/$BIN_NAME.exe ]; then
        mv target/$target/release/$BIN_NAME.exe releases/$BIN_NAME-$target.exe

        if [ -x "$(command -v upx)" ]; then
            cp releases/$BIN_NAME-$target.exe releases/$BIN_NAME-$target-compressed.exe

            if ! upx $UPX_FLAGS releases/$BIN_NAME-$target-compressed.exe; then
                rm releases/$BIN_NAME-$target-compressed.exe
            fi
        fi
    fi
done