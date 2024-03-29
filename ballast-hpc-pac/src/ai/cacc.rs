#[doc = r"Register block"]
#[repr(C)]
pub struct CACC {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: S_STATUS,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: S_POINTER,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: D_OP_ENABLE,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, etc."]
    pub d_misc_cfg: D_MISC_CFG,
    #[doc = "0x10 - Input cube’s width and height after extension"]
    pub d_dataout_size_0: D_DATAOUT_SIZE_0,
    #[doc = "0x14 - Input cube’s channel after extension"]
    pub d_dataout_size_1: D_DATAOUT_SIZE_1,
    #[doc = "0x18 - Address of output cube"]
    pub d_dataout_addr: D_DATAOUT_ADDR,
    #[doc = "0x1c - Number of batches"]
    pub d_batch_number: D_BATCH_NUMBER,
    #[doc = "0x20 - Line stride of output cube"]
    pub d_line_stride: D_LINE_STRIDE,
    #[doc = "0x24 - Line stride of surface cube"]
    pub d_surf_stride: D_SURF_STRIDE,
    #[doc = "0x28 - Whether output cube is line packed or surface packed"]
    pub d_dataout_map: D_DATAOUT_MAP,
    #[doc = "0x2c - Number of bits to be truncated before sending to SDP"]
    pub d_clip_cfg: D_CLIP_CFG,
    #[doc = "0x30 - Output saturation count"]
    pub d_out_saturation: D_OUT_SATURATION,
    #[doc = "0x34 - "]
    pub d_cya: D_CYA,
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
#[doc = "D_DATAOUT_SIZE_0 (rw) register accessor: an alias for `Reg<D_DATAOUT_SIZE_0_SPEC>`"]
pub type D_DATAOUT_SIZE_0 = crate::Reg<d_dataout_size_0::D_DATAOUT_SIZE_0_SPEC>;
#[doc = "Input cube’s width and height after extension"]
pub mod d_dataout_size_0;
#[doc = "D_DATAOUT_SIZE_1 (rw) register accessor: an alias for `Reg<D_DATAOUT_SIZE_1_SPEC>`"]
pub type D_DATAOUT_SIZE_1 = crate::Reg<d_dataout_size_1::D_DATAOUT_SIZE_1_SPEC>;
#[doc = "Input cube’s channel after extension"]
pub mod d_dataout_size_1;
#[doc = "D_DATAOUT_ADDR (rw) register accessor: an alias for `Reg<D_DATAOUT_ADDR_SPEC>`"]
pub type D_DATAOUT_ADDR = crate::Reg<d_dataout_addr::D_DATAOUT_ADDR_SPEC>;
#[doc = "Address of output cube"]
pub mod d_dataout_addr;
#[doc = "D_BATCH_NUMBER (rw) register accessor: an alias for `Reg<D_BATCH_NUMBER_SPEC>`"]
pub type D_BATCH_NUMBER = crate::Reg<d_batch_number::D_BATCH_NUMBER_SPEC>;
#[doc = "Number of batches"]
pub mod d_batch_number;
#[doc = "D_LINE_STRIDE (rw) register accessor: an alias for `Reg<D_LINE_STRIDE_SPEC>`"]
pub type D_LINE_STRIDE = crate::Reg<d_line_stride::D_LINE_STRIDE_SPEC>;
#[doc = "Line stride of output cube"]
pub mod d_line_stride;
#[doc = "D_SURF_STRIDE (rw) register accessor: an alias for `Reg<D_SURF_STRIDE_SPEC>`"]
pub type D_SURF_STRIDE = crate::Reg<d_surf_stride::D_SURF_STRIDE_SPEC>;
#[doc = "Line stride of surface cube"]
pub mod d_surf_stride;
#[doc = "D_DATAOUT_MAP (rw) register accessor: an alias for `Reg<D_DATAOUT_MAP_SPEC>`"]
pub type D_DATAOUT_MAP = crate::Reg<d_dataout_map::D_DATAOUT_MAP_SPEC>;
#[doc = "Whether output cube is line packed or surface packed"]
pub mod d_dataout_map;
#[doc = "D_CLIP_CFG (rw) register accessor: an alias for `Reg<D_CLIP_CFG_SPEC>`"]
pub type D_CLIP_CFG = crate::Reg<d_clip_cfg::D_CLIP_CFG_SPEC>;
#[doc = "Number of bits to be truncated before sending to SDP"]
pub mod d_clip_cfg;
#[doc = "D_OUT_SATURATION (r) register accessor: an alias for `Reg<D_OUT_SATURATION_SPEC>`"]
pub type D_OUT_SATURATION = crate::Reg<d_out_saturation::D_OUT_SATURATION_SPEC>;
#[doc = "Output saturation count"]
pub mod d_out_saturation;
#[doc = "D_CYA (rw) register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
