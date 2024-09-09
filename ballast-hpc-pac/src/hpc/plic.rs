pub use riscv_peripheral::{
    aclint::HartIdNumber,
    plic::{ContextNumber, InterruptNumber, Plic, PriorityNumber},
};

#[doc = r"Enumeration of HPC external interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PlicInterrupt {
    #[doc = "1 - TIMER0"]
    TIMER0 = 1,
    #[doc = "2 - TIMER1"]
    TIMER1 = 2,
    #[doc = "3 - EXTERNAL"]
    EXTERNAL = 3,
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Priority {
    #[doc = "0: Priority 0 (never interrupt)"]
    P0 = 0,
    #[doc = "1: Priority 1"]
    P1 = 1,
    #[doc = "2: Priority 2"]
    P2 = 2,
    #[doc = "3: Priority 3"]
    P3 = 3,
    #[doc = "4: Priority 4"]
    P4 = 4,
    #[doc = "5: Priority 5"]
    P5 = 5,
    #[doc = "6: Priority 6"]
    P6 = 6,
    #[doc = "7: Priority 7"]
    P7 = 7,
}

/// PLIC context of the target
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum Context {
    #[doc = "0: Context 0"]
    C0 = 0,
    #[doc = "1: Context 1"]
    C1 = 1,
    #[doc = "2: Context 2"]
    C2 = 2,
    #[doc = "3: Context 3"]
    C3 = 3,
}

unsafe impl ContextNumber for Context {
    const MAX_CONTEXT_NUMBER: u16 = 3;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        if number > Self::MAX_CONTEXT_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid context number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

unsafe impl InterruptNumber for PlicInterrupt {
    // TODO: check this value for HPC
    const MAX_INTERRUPT_NUMBER: u16 = 3;

    #[inline]
    fn number(self) -> u16 {
        self as _
    }

    #[inline]
    fn from_number(number: u16) -> Result<Self, u16> {
        if number == 0 || number > Self::MAX_INTERRUPT_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid interrupt number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}

unsafe impl PriorityNumber for Priority {
    const MAX_PRIORITY_NUMBER: u8 = 7;

    #[inline]
    fn number(self) -> u8 {
        self as _
    }

    #[inline]
    fn from_number(number: u8) -> Result<Self, u8> {
        if number > Self::MAX_PRIORITY_NUMBER {
            Err(number)
        } else {
            // SAFETY: valid priority number
            Ok(unsafe { core::mem::transmute(number) })
        }
    }
}
riscv_peripheral::plic_codegen!(base 0x100_0000, ctxs [ctx0 = (Context::C0, "`C0`"), ctx1 = (Context::C1, "`C1`"), ctx2 = (Context::C2, "`C2`"),  ctx3 = (Context::C3, "`C3`")],);
