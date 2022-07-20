#!/usr/bin/env bash

# This script mimics original source from:
# https://github.com/riscv-rust/gd32vf103-pac/blob/master/update.sh.

# Before running this script, install the required software. See README.md for
# compatible versions:
# cargo install svd2rust
# cargo install form
# pip3 install --upgrade --user svdtools

SVD2RUST_VER="0.23"
if [ $SVD2RUST_VER != $(svd2rust --version | grep -Po '\d+\.\d+') ];
    then echo "Unexpected svd2rust version. Install $SVD2RUST_VER using \`cargo install svd2rust --version=\"^$SVD2RUST_VER\"\` or run this script with \`FORCE=1 ./update.sh\`"
    [ -z "$FORCE" ] && exit 1
fi

set -x
set -e

rm -rf src
mkdir src
svd patch patches/SysCtrl.yaml
svd2rust --target riscv -i SysCtrl.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt

grep -E 'feature = "rt"|extern crate riscv_rt' src/lib.rs | tee librs-patch
grep -Ev 'feature = "rt"|extern crate riscv_rt' src/lib.rs > librs-temp && mv librs-temp src/lib.rs

cargo check
cargo test

