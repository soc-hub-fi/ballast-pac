#[doc = r"Register block"]
#[repr(C)]
pub struct BOOT_CONFIG {
    #[doc = "0x00 - "]
    pub boot_cfg: BOOT_CFG,
    #[doc = "0x04 - "]
    pub boot_status: BOOT_STATUS,
}
#[doc = "BOOT_CFG (rw) register accessor: an alias for `Reg<BOOT_CFG_SPEC>`"]
pub type BOOT_CFG = crate::Reg<boot_cfg::BOOT_CFG_SPEC>;
#[doc = ""]
pub mod boot_cfg;
#[doc = "BOOT_STATUS (rw) register accessor: an alias for `Reg<BOOT_STATUS_SPEC>`"]
pub type BOOT_STATUS = crate::Reg<boot_status::BOOT_STATUS_SPEC>;
#[doc = ""]
pub mod boot_status;
