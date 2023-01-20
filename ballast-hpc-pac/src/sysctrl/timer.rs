#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Timer Low Configuration register"]
    pub cfg_lo: CFG_LO,
    #[doc = "0x04 - Timer High Configuration register"]
    pub cfg_hi: CFG_HI,
    #[doc = "0x08 - Timer Low counter value register"]
    pub cnt_lo: CNT_LO,
    #[doc = "0x0c - Timer High counter value register"]
    pub cnt_hi: CNT_HI,
    #[doc = "0x10 - Timer Low comparator value register"]
    pub cmp_lo: CMP_LO,
    #[doc = "0x14 - Timer High comparator value register"]
    pub cmp_hi: CMP_HI,
    #[doc = "0x18 - Start Timer Low counting register"]
    pub start_lo: START_LO,
    #[doc = "0x1c - Start Timer High counting register"]
    pub start_hi: START_HI,
    #[doc = "0x20 - Reset Timer Low counter register"]
    pub reset_lo: RESET_LO,
    #[doc = "0x24 - Reset Timer High counter register"]
    pub reset_hi: RESET_HI,
}
#[doc = "CFG_LO (rw) register accessor: an alias for `Reg<CFG_LO_SPEC>`"]
pub type CFG_LO = crate::Reg<cfg_lo::CFG_LO_SPEC>;
#[doc = "Timer Low Configuration register"]
pub mod cfg_lo;
#[doc = "CFG_HI (rw) register accessor: an alias for `Reg<CFG_HI_SPEC>`"]
pub type CFG_HI = crate::Reg<cfg_hi::CFG_HI_SPEC>;
#[doc = "Timer High Configuration register"]
pub mod cfg_hi;
#[doc = "CNT_LO (rw) register accessor: an alias for `Reg<CNT_LO_SPEC>`"]
pub type CNT_LO = crate::Reg<cnt_lo::CNT_LO_SPEC>;
#[doc = "Timer Low counter value register"]
pub mod cnt_lo;
#[doc = "CNT_HI (rw) register accessor: an alias for `Reg<CNT_HI_SPEC>`"]
pub type CNT_HI = crate::Reg<cnt_hi::CNT_HI_SPEC>;
#[doc = "Timer High counter value register"]
pub mod cnt_hi;
#[doc = "CMP_LO (rw) register accessor: an alias for `Reg<CMP_LO_SPEC>`"]
pub type CMP_LO = crate::Reg<cmp_lo::CMP_LO_SPEC>;
#[doc = "Timer Low comparator value register"]
pub mod cmp_lo;
#[doc = "CMP_HI (rw) register accessor: an alias for `Reg<CMP_HI_SPEC>`"]
pub type CMP_HI = crate::Reg<cmp_hi::CMP_HI_SPEC>;
#[doc = "Timer High comparator value register"]
pub mod cmp_hi;
#[doc = "START_LO (rw) register accessor: an alias for `Reg<START_LO_SPEC>`"]
pub type START_LO = crate::Reg<start_lo::START_LO_SPEC>;
#[doc = "Start Timer Low counting register"]
pub mod start_lo;
#[doc = "START_HI (rw) register accessor: an alias for `Reg<START_HI_SPEC>`"]
pub type START_HI = crate::Reg<start_hi::START_HI_SPEC>;
#[doc = "Start Timer High counting register"]
pub mod start_hi;
#[doc = "RESET_LO (rw) register accessor: an alias for `Reg<RESET_LO_SPEC>`"]
pub type RESET_LO = crate::Reg<reset_lo::RESET_LO_SPEC>;
#[doc = "Reset Timer Low counter register"]
pub mod reset_lo;
#[doc = "RESET_HI (rw) register accessor: an alias for `Reg<RESET_HI_SPEC>`"]
pub type RESET_HI = crate::Reg<reset_hi::RESET_HI_SPEC>;
#[doc = "Reset Timer High counter register"]
pub mod reset_hi;
