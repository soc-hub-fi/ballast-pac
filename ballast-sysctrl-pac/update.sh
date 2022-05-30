#!/usr/bin/env bash

# This script mimics original source from:
# https://github.com/riscv-rust/gd32vf103-pac/blob/master/update.sh.

# Before running this script, install the required software:
# cargo install svd2rust --version=0.18
# cargo install form --version=0.8
# pip3 install --upgrade --user svdtools

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
