#[doc = r"Register block"]
#[repr(C)]
pub struct SOC_EVENT_GENERATOR {
    #[doc = "0x00 - SoC software events trigger register"]
    pub sw_event: SW_EVENT,
    #[doc = "0x04..0x24 - Events 0-31 dispatch mask to FC"]
    pub fc_mask: [FC_MASK; 8],
    _reserved2: [u8; 0x20],
    #[doc = "0x44..0x64 - Events 0-31 dispatch mask to peripherals"]
    pub pr_mask: [PR_MASK; 8],
    #[doc = "0x64..0x84 - Events 0-31 event queue overflow"]
    pub err: [ERR; 8],
    #[doc = "0x84 - "]
    pub timer_lo: TIMER_LO,
    #[doc = "0x88 - Trigger Timer HI of APB Timer with event"]
    pub timer_hi: TIMER_HI,
}
#[doc = "SW_EVENT (w) register accessor: an alias for `Reg<SW_EVENT_SPEC>`"]
pub type SW_EVENT = crate::Reg<sw_event::SW_EVENT_SPEC>;
#[doc = "SoC software events trigger register"]
pub mod sw_event;
#[doc = "FC_MASK (rw) register accessor: an alias for `Reg<FC_MASK_SPEC>`"]
pub type FC_MASK = crate::Reg<fc_mask::FC_MASK_SPEC>;
#[doc = "Events 0-31 dispatch mask to FC"]
pub mod fc_mask;
#[doc = "ERR (r) register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Events 0-31 event queue overflow"]
pub mod err;
#[doc = "PR_MASK (rw) register accessor: an alias for `Reg<PR_MASK_SPEC>`"]
pub type PR_MASK = crate::Reg<pr_mask::PR_MASK_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripherals"]
pub mod pr_mask;
#[doc = "TIMER_HI (rw) register accessor: an alias for `Reg<TIMER_HI_SPEC>`"]
pub type TIMER_HI = crate::Reg<timer_hi::TIMER_HI_SPEC>;
#[doc = "Trigger Timer HI of APB Timer with event"]
pub mod timer_hi;
#[doc = "TIMER_LO (rw) register accessor: an alias for `Reg<TIMER_LO_SPEC>`"]
pub type TIMER_LO = crate::Reg<timer_lo::TIMER_LO_SPEC>;
#[doc = ""]
pub mod timer_lo;
