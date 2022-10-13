use ballast_hpc_pac as pac;

fn check_address(ptr: *const u64, expected: u64) {
    let ptr = ptr as u64;
    assert_eq!(expected, ptr, "expected 0x{:x}, was 0x{:x}", expected, ptr);
}

#[test]
fn hpc() {
    let ptr = pac::HPC::ptr() as *const u64;

    check_address(ptr, 0x0000_0000_0000_0000);
}

#[test]
fn mpc() {
    let ptr = pac::MPC::ptr() as *const u64;

    check_address(ptr, 0x0001_2A00_0000);
}
