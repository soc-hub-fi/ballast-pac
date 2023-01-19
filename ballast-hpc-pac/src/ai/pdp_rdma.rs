#[doc = r"Register block"]
#[repr(C)]
pub struct PDP_RDMA {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: S_STATUS,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: S_POINTER,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: D_OP_ENABLE,
    #[doc = "0x0c - "]
    pub d_data_cube_in_width: D_DATA_CUBE_IN_WIDTH,
    #[doc = "0x10 - "]
    pub d_data_cube_in_height: D_DATA_CUBE_IN_HEIGHT,
    #[doc = "0x14 - "]
    pub d_data_cube_in_channel: D_DATA_CUBE_IN_CHANNEL,
    #[doc = "0x18 - "]
    pub d_flying_mode: D_FLYING_MODE,
    #[doc = "0x1c - "]
    pub d_src_base_addr_low: D_SRC_BASE_ADDR_LOW,
    #[doc = "0x20 - "]
    pub d_src_base_addr_high: D_SRC_BASE_ADDR_HIGH,
    #[doc = "0x24 - "]
    pub d_src_line_stride: D_SRC_LINE_STRIDE,
    #[doc = "0x28 - "]
    pub d_src_surface_stride: D_SRC_SURFACE_STRIDE,
    #[doc = "0x2c - "]
    pub d_src_ram_cfg: D_SRC_RAM_CFG,
    #[doc = "0x30 - "]
    pub d_data_format: D_DATA_FORMAT,
    #[doc = "0x34 - "]
    pub d_operation_mode_cfg: D_OPERATION_MODE_CFG,
    #[doc = "0x38 - "]
    pub d_pooling_kernel_cfg: D_POOLING_KERNEL_CFG,
    #[doc = "0x3c - "]
    pub d_pooling_padding_cfg: D_POOLING_PADDING_CFG,
    #[doc = "0x40 - "]
    pub d_partial_width_in: D_PARTIAL_WIDTH_IN,
    #[doc = "0x44 - "]
    pub d_perf_enable: D_PERF_ENABLE,
    #[doc = "0x48 - "]
    pub d_perf_read_stall: D_PERF_READ_STALL,
    #[doc = "0x4c - "]
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
#[doc = "D_DATA_CUBE_IN_WIDTH (rw) register accessor: an alias for `Reg<D_DATA_CUBE_IN_WIDTH_SPEC>`"]
pub type D_DATA_CUBE_IN_WIDTH = crate::Reg<d_data_cube_in_width::D_DATA_CUBE_IN_WIDTH_SPEC>;
#[doc = ""]
pub mod d_data_cube_in_width;
#[doc = "D_DATA_CUBE_IN_HEIGHT (rw) register accessor: an alias for `Reg<D_DATA_CUBE_IN_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_IN_HEIGHT = crate::Reg<d_data_cube_in_height::D_DATA_CUBE_IN_HEIGHT_SPEC>;
#[doc = ""]
pub mod d_data_cube_in_height;
#[doc = "D_DATA_CUBE_IN_CHANNEL (rw) register accessor: an alias for `Reg<D_DATA_CUBE_IN_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_IN_CHANNEL = crate::Reg<d_data_cube_in_channel::D_DATA_CUBE_IN_CHANNEL_SPEC>;
#[doc = ""]
pub mod d_data_cube_in_channel;
#[doc = "D_FLYING_MODE (rw) register accessor: an alias for `Reg<D_FLYING_MODE_SPEC>`"]
pub type D_FLYING_MODE = crate::Reg<d_flying_mode::D_FLYING_MODE_SPEC>;
#[doc = ""]
pub mod d_flying_mode;
#[doc = "D_SRC_BASE_ADDR_LOW (rw) register accessor: an alias for `Reg<D_SRC_BASE_ADDR_LOW_SPEC>`"]
pub type D_SRC_BASE_ADDR_LOW = crate::Reg<d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>;
#[doc = ""]
pub mod d_src_base_addr_low;
#[doc = "D_SRC_BASE_ADDR_HIGH (rw) register accessor: an alias for `Reg<D_SRC_BASE_ADDR_HIGH_SPEC>`"]
pub type D_SRC_BASE_ADDR_HIGH = crate::Reg<d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>;
#[doc = ""]
pub mod d_src_base_addr_high;
#[doc = "D_SRC_LINE_STRIDE (rw) register accessor: an alias for `Reg<D_SRC_LINE_STRIDE_SPEC>`"]
pub type D_SRC_LINE_STRIDE = crate::Reg<d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_src_line_stride;
#[doc = "D_SRC_SURFACE_STRIDE (rw) register accessor: an alias for `Reg<D_SRC_SURFACE_STRIDE_SPEC>`"]
pub type D_SRC_SURFACE_STRIDE = crate::Reg<d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_src_surface_stride;
#[doc = "D_SRC_RAM_CFG (rw) register accessor: an alias for `Reg<D_SRC_RAM_CFG_SPEC>`"]
pub type D_SRC_RAM_CFG = crate::Reg<d_src_ram_cfg::D_SRC_RAM_CFG_SPEC>;
#[doc = ""]
pub mod d_src_ram_cfg;
#[doc = "D_DATA_FORMAT (rw) register accessor: an alias for `Reg<D_DATA_FORMAT_SPEC>`"]
pub type D_DATA_FORMAT = crate::Reg<d_data_format::D_DATA_FORMAT_SPEC>;
#[doc = ""]
pub mod d_data_format;
#[doc = "D_OPERATION_MODE_CFG (rw) register accessor: an alias for `Reg<D_OPERATION_MODE_CFG_SPEC>`"]
pub type D_OPERATION_MODE_CFG = crate::Reg<d_operation_mode_cfg::D_OPERATION_MODE_CFG_SPEC>;
#[doc = ""]
pub mod d_operation_mode_cfg;
#[doc = "D_POOLING_KERNEL_CFG (rw) register accessor: an alias for `Reg<D_POOLING_KERNEL_CFG_SPEC>`"]
pub type D_POOLING_KERNEL_CFG = crate::Reg<d_pooling_kernel_cfg::D_POOLING_KERNEL_CFG_SPEC>;
#[doc = ""]
pub mod d_pooling_kernel_cfg;
#[doc = "D_POOLING_PADDING_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_CFG_SPEC>`"]
pub type D_POOLING_PADDING_CFG = crate::Reg<d_pooling_padding_cfg::D_POOLING_PADDING_CFG_SPEC>;
#[doc = ""]
pub mod d_pooling_padding_cfg;
#[doc = "D_PARTIAL_WIDTH_IN (rw) register accessor: an alias for `Reg<D_PARTIAL_WIDTH_IN_SPEC>`"]
pub type D_PARTIAL_WIDTH_IN = crate::Reg<d_partial_width_in::D_PARTIAL_WIDTH_IN_SPEC>;
#[doc = ""]
pub mod d_partial_width_in;
#[doc = "D_PERF_ENABLE (rw) register accessor: an alias for `Reg<D_PERF_ENABLE_SPEC>`"]
pub type D_PERF_ENABLE = crate::Reg<d_perf_enable::D_PERF_ENABLE_SPEC>;
#[doc = ""]
pub mod d_perf_enable;
#[doc = "D_PERF_READ_STALL (r) register accessor: an alias for `Reg<D_PERF_READ_STALL_SPEC>`"]
pub type D_PERF_READ_STALL = crate::Reg<d_perf_read_stall::D_PERF_READ_STALL_SPEC>;
#[doc = ""]
pub mod d_perf_read_stall;
#[doc = "D_CYA (rw) register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
