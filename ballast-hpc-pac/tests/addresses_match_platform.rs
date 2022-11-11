// This warning is incorrect in our case; formatting is not actually fully supported in panic
#![allow(non_fmt_panics)]

use ballast_hpc_pac as pac;

fn verify_ptr<T>(ptr: *const T, expected_addr: usize) {
    let addr = ptr as usize;

    if addr != expected_addr {
        panic!(format!(
            "incorrect address in memory map:\nexpected: 0x{expected_addr:x}\n  actual: 0x{addr:x}"
        ))
    }
}

#[test]
fn hpc() {
    verify_ptr(pac::HPC::ptr(), 0x0000_0000_0000_0000);
}

#[test]
fn mpc() {
    verify_ptr(pac::MPC::ptr(), 0x0001_2A00_0000);
}
