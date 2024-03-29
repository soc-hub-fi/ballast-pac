#[doc = r"Register block"]
#[repr(C)]
pub struct CMAC_A {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: S_STATUS,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: S_POINTER,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: D_OP_ENABLE,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, etc."]
    pub d_misc_cfg: D_MISC_CFG,
}
#[doc = "S_STATUS (r) register accessor: an alias for `Reg<S_STATUS_SPEC>`"]
pub type S_STATUS = crate::Reg<s_status::S_STATUS_SPEC>;
#[doc = "Idle status of two register groups"]
pub mod s_status;
#[doc = "S_POINTER (rw) register accessor: an alias for `Reg<S_POINTER_SPEC>`"]
pub type S_POINTER = crate::Reg<s_pointer::S_POINTER_SPEC>;
#[doc = "Pointer for CSB master and data path to access groups"]
pub mod s_pointer;
#[doc = "D_OP_ENABLE (rw) register accessor: an alias for `Reg<D_OP_ENABLE_SPEC>`"]
pub type D_OP_ENABLE = crate::Reg<d_op_enable::D_OP_ENABLE_SPEC>;
#[doc = "Set it to 1 to kick off operation for current register group"]
pub mod d_op_enable;
#[doc = "D_MISC_CFG (rw) register accessor: an alias for `Reg<D_MISC_CFG_SPEC>`"]
pub type D_MISC_CFG = crate::Reg<d_misc_cfg::D_MISC_CFG_SPEC>;
#[doc = "Configuration of operation: convolution mode, precision, etc."]
pub mod d_misc_cfg;
