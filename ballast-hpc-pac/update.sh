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
svd patch patches/HPC.yaml
svd2rust --target riscv -i HPC.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt

grep -E 'feature = "rt"|extern crate riscv_rt' src/lib.rs | tee librs-patch
grep -Ev 'feature = "rt"|extern crate riscv_rt' src/lib.rs > librs-temp && mv librs-temp src/lib.rs

# Re-export the local processor subsystem with a special name (author: Henri Lunnikivi)
echo "/// Re-export the local processor subsystem with a special name. This allows us to refer to" >> src/lib.rs
echo "/// the current processor from all distinct cores." >> src/lib.rs
echo "#[allow(unused_imports)]" >> src/lib.rs
echo "pub use crate::{hpc as cpu_subsys, HPC as CPU_SUBSYS};" >> src/lib.rs

cargo check
cargo test

