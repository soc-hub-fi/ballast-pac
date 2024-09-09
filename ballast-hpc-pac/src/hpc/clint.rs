pub use riscv_peripheral::{
    aclint::{Clint, HartIdNumber},
    clint_codegen,
};

/// CLINT HART IDs of the target
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum HartId {
    #[doc = "0: HART 0"]
    HART0 = 0,
    #[doc = "1: HART 1"]
    HART1 = 1,
}

unsafe impl riscv_peripheral::aclint::HartIdNumber for HartId {
    const MAX_HART_ID_NUMBER: u16 = Self::HART0 as _;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        match number {
            0 => Ok(Self::HART0),
            _ => Err(number),
        }
    }
}

clint_codegen!(base 0x20_0000 , freq 30_000_000,mtimecmps [mtimecmp0=(HartId::HART0,"0"),mtimecmp1=(HartId::HART1,"1")],); // Double check - ballast soc frequency
