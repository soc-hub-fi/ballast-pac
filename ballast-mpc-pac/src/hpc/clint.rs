#[doc = "msip register accessor: an alias for `Reg<MSIP_SPEC>`"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Machine mode software interrupt (IPI)"]
pub mod msip;
#[doc = "mtimecmp register accessor: an alias for `Reg<MTIMECMP_SPEC>`"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Machine mode timer compare register for Hart 0"]
pub mod mtimecmp;
#[doc = "mtime register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Timer register"]
pub mod mtime;
