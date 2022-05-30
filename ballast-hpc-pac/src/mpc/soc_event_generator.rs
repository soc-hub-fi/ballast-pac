#[doc = "SW_EVENT register accessor: an alias for `Reg<SW_EVENT_SPEC>`"]
pub type SW_EVENT = crate::Reg<sw_event::SW_EVENT_SPEC>;
#[doc = "SoC software events trigger register"]
pub mod sw_event;
#[doc = "FC_MASK0 register accessor: an alias for `Reg<FC_MASK0_SPEC>`"]
pub type FC_MASK0 = crate::Reg<fc_mask0::FC_MASK0_SPEC>;
#[doc = "Events 0-31 dispatch mask to FC"]
pub mod fc_mask0;
#[doc = "FC_MASK1 register accessor: an alias for `Reg<FC_MASK1_SPEC>`"]
pub type FC_MASK1 = crate::Reg<fc_mask1::FC_MASK1_SPEC>;
#[doc = "Events 32-63 dispatch mask to FC"]
pub mod fc_mask1;
#[doc = "FC_MASK2 register accessor: an alias for `Reg<FC_MASK2_SPEC>`"]
pub type FC_MASK2 = crate::Reg<fc_mask2::FC_MASK2_SPEC>;
#[doc = "Events 64-95 dispatch mask to FC"]
pub mod fc_mask2;
#[doc = "FC_MASK3 register accessor: an alias for `Reg<FC_MASK3_SPEC>`"]
pub type FC_MASK3 = crate::Reg<fc_mask3::FC_MASK3_SPEC>;
#[doc = "Events 96-127 dispatch mask to FC"]
pub mod fc_mask3;
#[doc = "FC_MASK4 register accessor: an alias for `Reg<FC_MASK4_SPEC>`"]
pub type FC_MASK4 = crate::Reg<fc_mask4::FC_MASK4_SPEC>;
#[doc = "Events 128-159 dispatch mask to FC"]
pub mod fc_mask4;
#[doc = "FC_MASK5 register accessor: an alias for `Reg<FC_MASK5_SPEC>`"]
pub type FC_MASK5 = crate::Reg<fc_mask5::FC_MASK5_SPEC>;
#[doc = "Events 160-191 dispatch mask to FC"]
pub mod fc_mask5;
#[doc = "FC_MASK6 register accessor: an alias for `Reg<FC_MASK6_SPEC>`"]
pub type FC_MASK6 = crate::Reg<fc_mask6::FC_MASK6_SPEC>;
#[doc = "Events 191-223 dispatch mask to FC"]
pub mod fc_mask6;
#[doc = "FC_MASK7 register accessor: an alias for `Reg<FC_MASK7_SPEC>`"]
pub type FC_MASK7 = crate::Reg<fc_mask7::FC_MASK7_SPEC>;
#[doc = "F Events 224-255 dispatch mask to peripherals"]
pub mod fc_mask7;
#[doc = "PR_MASK0 register accessor: an alias for `Reg<PR_MASK0_SPEC>`"]
pub type PR_MASK0 = crate::Reg<pr_mask0::PR_MASK0_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripherals"]
pub mod pr_mask0;
#[doc = "PR_MASK1 register accessor: an alias for `Reg<PR_MASK1_SPEC>`"]
pub type PR_MASK1 = crate::Reg<pr_mask1::PR_MASK1_SPEC>;
#[doc = ""]
pub mod pr_mask1;
#[doc = "PR_MASK2 register accessor: an alias for `Reg<PR_MASK2_SPEC>`"]
pub type PR_MASK2 = crate::Reg<pr_mask2::PR_MASK2_SPEC>;
#[doc = "Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
pub mod pr_mask2;
#[doc = "PR_MASK3 register accessor: an alias for `Reg<PR_MASK3_SPEC>`"]
pub type PR_MASK3 = crate::Reg<pr_mask3::PR_MASK3_SPEC>;
#[doc = "Events 96-127 dispatch mask to peripherals"]
pub mod pr_mask3;
#[doc = "PR_MASK4 register accessor: an alias for `Reg<PR_MASK4_SPEC>`"]
pub type PR_MASK4 = crate::Reg<pr_mask4::PR_MASK4_SPEC>;
#[doc = "Events 128-159 dispatch mask to peripheral"]
pub mod pr_mask4;
#[doc = "PR_MASK5 register accessor: an alias for `Reg<PR_MASK5_SPEC>`"]
pub type PR_MASK5 = crate::Reg<pr_mask5::PR_MASK5_SPEC>;
#[doc = "Events 160-191 dispatch mask to peripherals"]
pub mod pr_mask5;
#[doc = "PR_MASK6 register accessor: an alias for `Reg<PR_MASK6_SPEC>`"]
pub type PR_MASK6 = crate::Reg<pr_mask6::PR_MASK6_SPEC>;
#[doc = "Events 191-223 dispatch mask to peripherals"]
pub mod pr_mask6;
#[doc = "PR_MASK7 register accessor: an alias for `Reg<PR_MASK7_SPEC>`"]
pub type PR_MASK7 = crate::Reg<pr_mask7::PR_MASK7_SPEC>;
#[doc = "Events 224-255 dispatch mask to peripherals"]
pub mod pr_mask7;
#[doc = "ERR0 register accessor: an alias for `Reg<ERR0_SPEC>`"]
pub type ERR0 = crate::Reg<err0::ERR0_SPEC>;
#[doc = "Events 0-31 event queue overflow"]
pub mod err0;
#[doc = "ERR1 register accessor: an alias for `Reg<ERR1_SPEC>`"]
pub type ERR1 = crate::Reg<err1::ERR1_SPEC>;
#[doc = "Events 32-63 event queue overflow"]
pub mod err1;
#[doc = "ERR2 register accessor: an alias for `Reg<ERR2_SPEC>`"]
pub type ERR2 = crate::Reg<err2::ERR2_SPEC>;
#[doc = "Events 64-95 event queue overflow"]
pub mod err2;
#[doc = "ERR3 register accessor: an alias for `Reg<ERR3_SPEC>`"]
pub type ERR3 = crate::Reg<err3::ERR3_SPEC>;
#[doc = "Events 96-127 event queue overflow"]
pub mod err3;
#[doc = "ERR4 register accessor: an alias for `Reg<ERR4_SPEC>`"]
pub type ERR4 = crate::Reg<err4::ERR4_SPEC>;
#[doc = "Events 128-159 event queue overflow"]
pub mod err4;
#[doc = "ERR5 register accessor: an alias for `Reg<ERR5_SPEC>`"]
pub type ERR5 = crate::Reg<err5::ERR5_SPEC>;
#[doc = "Events 160-191 event queue overflow"]
pub mod err5;
#[doc = "ERR6 register accessor: an alias for `Reg<ERR6_SPEC>`"]
pub type ERR6 = crate::Reg<err6::ERR6_SPEC>;
#[doc = "Events 191-223 event queue overflow"]
pub mod err6;
#[doc = "ERR7 register accessor: an alias for `Reg<ERR7_SPEC>`"]
pub type ERR7 = crate::Reg<err7::ERR7_SPEC>;
#[doc = "Events 224-255 event queue overflow"]
pub mod err7;
#[doc = "TIMER_LO register accessor: an alias for `Reg<TIMER_LO_SPEC>`"]
pub type TIMER_LO = crate::Reg<timer_lo::TIMER_LO_SPEC>;
#[doc = ""]
pub mod timer_lo;
#[doc = "TIMER_HI register accessor: an alias for `Reg<TIMER_HI_SPEC>`"]
pub type TIMER_HI = crate::Reg<timer_hi::TIMER_HI_SPEC>;
#[doc = "Trigger Timer HI of APB Timer with event"]
pub mod timer_hi;
