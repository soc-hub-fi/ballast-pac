use ballast_sysctrl_pac as pac;

#[test]
fn sysctrl() {
    let ptr = pac::SYSCTRL::ptr() as *const u32;

    let addr = ptr as u32;
    assert_eq!(
        0x1A00_0000, addr,
        "expected 0x{:x}, was 0x{:x}",
        0x1A00_0000, addr
    );
}

#[test]
fn mpc() {
    let ptr = pac::MPC::ptr() as *const u32;

    let addr = ptr as u32;
    assert_eq!(
        0x2A00_0000, addr,
        "expected 0x{:x}, was 0x{:x}",
        0x2A00_0000, addr
    );
}
