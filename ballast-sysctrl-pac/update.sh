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

# Re-export the local processor subsystem with a special name (author: Henri Lunnikivi)
echo "/// Re-export the local processor subsystem with a special name. This allows us to refer to" >> src/lib.rs
echo "/// the current processor from all distinct cores." >> src/lib.rs
echo "#[allow(unused_imports)]" >> src/lib.rs
echo "pub use crate::{sysctrl as cpu_subsys, SYSCTRL as CPU_SUBSYS};" >> src/lib.rs

cargo check
