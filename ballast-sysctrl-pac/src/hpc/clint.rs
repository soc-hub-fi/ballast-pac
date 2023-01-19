#[doc = r"Register block"]
#[repr(C)]
pub struct CLINT {
    #[doc = "0x00..0x08 - Machine mode software interrupt (IPI)"]
    pub msip: MSIP,
    _reserved1: [u8; 0x3ff8],
    #[doc = "0x4000..0x4010 - Machine mode timer compare register for Hart 0"]
    pub mtimecmp: [MTIMECMP; 2],
    _reserved2: [u8; 0x7fe8],
    #[doc = "0xbff8..0xc000 - Timer register"]
    pub mtime: MTIME,
}
#[doc = "msip (rw) register accessor: an alias for `Reg<MSIP_SPEC>`"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Machine mode software interrupt (IPI)"]
pub mod msip;
#[doc = "mtimecmp (rw) register accessor: an alias for `Reg<MTIMECMP_SPEC>`"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Machine mode timer compare register for Hart 0"]
pub mod mtimecmp;
#[doc = "mtime (rw) register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Timer register"]
pub mod mtime;
