#[doc = "SW_EVENT register accessor: an alias for `Reg<SW_EVENT_SPEC>`"]
pub type SW_EVENT = crate::Reg<sw_event::SW_EVENT_SPEC>;
#[doc = "SoC software events trigger register"]
pub mod sw_event;
#[doc = "FC_MASK register accessor: an alias for `Reg<FC_MASK_SPEC>`"]
pub type FC_MASK = crate::Reg<fc_mask::FC_MASK_SPEC>;
#[doc = "Events 0-31 dispatch mask to FC"]
pub mod fc_mask;
#[doc = "PR_MASK register accessor: an alias for `Reg<PR_MASK_SPEC>`"]
pub type PR_MASK = crate::Reg<pr_mask::PR_MASK_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripherals"]
pub mod pr_mask;
#[doc = "ERR register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Events 0-31 event queue overflow"]
pub mod err;
#[doc = "TIMER_LO register accessor: an alias for `Reg<TIMER_LO_SPEC>`"]
pub type TIMER_LO = crate::Reg<timer_lo::TIMER_LO_SPEC>;
#[doc = ""]
pub mod timer_lo;
#[doc = "TIMER_HI register accessor: an alias for `Reg<TIMER_HI_SPEC>`"]
pub type TIMER_HI = crate::Reg<timer_hi::TIMER_HI_SPEC>;
#[doc = "Trigger Timer HI of APB Timer with event"]
pub mod timer_hi;
