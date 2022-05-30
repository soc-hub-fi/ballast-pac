#[doc = "HW_VERSION register accessor: an alias for `Reg<HW_VERSION_SPEC>`"]
pub type HW_VERSION = crate::Reg<hw_version::HW_VERSION_SPEC>;
#[doc = "HW version of NVDLA"]
pub mod hw_version;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask control"]
pub mod intr_mask;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set control"]
pub mod intr_set;
#[doc = "INTR_STATUS register accessor: an alias for `Reg<INTR_STATUS_SPEC>`"]
pub type INTR_STATUS = crate::Reg<intr_status::INTR_STATUS_SPEC>;
#[doc = "Interrupt status"]
pub mod intr_status;
