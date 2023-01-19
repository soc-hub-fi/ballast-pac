#[doc = r"Register block"]
#[repr(C)]
pub struct PLIC {
    _reserved0: [u8; 0x04],
    #[doc = "0x04..0x0c - "]
    pub timer_0_int_priority: [TIMER_0_INT_PRIORITY; 2],
    #[doc = "0x0c..0x14 - "]
    pub timer_1_int_priority: [TIMER_1_INT_PRIORITY; 2],
    #[doc = "0x14..0x1c - "]
    pub external_0_int_priority: [EXTERNAL_0_INT_PRIORITY; 2],
    _reserved3: [u8; 0x1fe4],
    #[doc = "0x2000..0x2010 - "]
    pub enable_context: [ENABLE_CONTEXT; 4],
    _reserved4: [u8; 0x001f_dff0],
    #[doc = "0x200000..0x200010 - "]
    pub priority_threshold_context: [PRIORITY_THRESHOLD_CONTEXT; 4],
    #[doc = "0x200010..0x200020 - "]
    pub claim_complete_context: [CLAIM_COMPLETE_CONTEXT; 4],
}
#[doc = "timer_0_int_priority (rw) register accessor: an alias for `Reg<TIMER_0_INT_PRIORITY_SPEC>`"]
pub type TIMER_0_INT_PRIORITY = crate::Reg<timer_0_int_priority::TIMER_0_INT_PRIORITY_SPEC>;
#[doc = ""]
pub mod timer_0_int_priority;
#[doc = "timer_1_int_priority (rw) register accessor: an alias for `Reg<TIMER_1_INT_PRIORITY_SPEC>`"]
pub type TIMER_1_INT_PRIORITY = crate::Reg<timer_1_int_priority::TIMER_1_INT_PRIORITY_SPEC>;
#[doc = ""]
pub mod timer_1_int_priority;
#[doc = "external_0_int_priority (rw) register accessor: an alias for `Reg<EXTERNAL_0_INT_PRIORITY_SPEC>`"]
pub type EXTERNAL_0_INT_PRIORITY =
    crate::Reg<external_0_int_priority::EXTERNAL_0_INT_PRIORITY_SPEC>;
#[doc = ""]
pub mod external_0_int_priority;
#[doc = "enable_context (rw) register accessor: an alias for `Reg<ENABLE_CONTEXT_SPEC>`"]
pub type ENABLE_CONTEXT = crate::Reg<enable_context::ENABLE_CONTEXT_SPEC>;
#[doc = ""]
pub mod enable_context;
#[doc = "priority_threshold_context (rw) register accessor: an alias for `Reg<PRIORITY_THRESHOLD_CONTEXT_SPEC>`"]
pub type PRIORITY_THRESHOLD_CONTEXT =
    crate::Reg<priority_threshold_context::PRIORITY_THRESHOLD_CONTEXT_SPEC>;
#[doc = ""]
pub mod priority_threshold_context;
#[doc = "claim_complete_context (rw) register accessor: an alias for `Reg<CLAIM_COMPLETE_CONTEXT_SPEC>`"]
pub type CLAIM_COMPLETE_CONTEXT = crate::Reg<claim_complete_context::CLAIM_COMPLETE_CONTEXT_SPEC>;
#[doc = ""]
pub mod claim_complete_context;
