// This warning is incorrect in our case; formatting is not actually fully supported in panic
#![allow(non_fmt_panics)]

use ballast_mpc_pac as pac;

fn verify_ptr<T>(ptr: *const T, expected_addr: usize) {
    let addr = ptr as usize;

    if addr != expected_addr {
        panic!(format!(
            "incorrect address in memory map:\nexpected: 0x{expected_addr:x}\n  actual: 0x{addr:x}"
        ))
    }
}

#[test]
fn sysctrl() {
    verify_ptr(pac::SYSCTRL::ptr(), 0x2A00_0000);
}

#[test]
fn mpc() {
    verify_ptr(pac::MPC::ptr(), 0x1A00_0000);
}

#[test]
fn tlp() {
    verify_ptr(pac::TLP::ptr(), 0x2000_0000);
}
