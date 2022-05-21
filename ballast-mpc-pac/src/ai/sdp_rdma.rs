#[doc = "S_STATUS register accessor: an alias for `Reg<S_STATUS_SPEC>`"]
pub type S_STATUS = crate::Reg<s_status::S_STATUS_SPEC>;
#[doc = "Idle status of two register groups"]
pub mod s_status;
#[doc = "S_POINTER register accessor: an alias for `Reg<S_POINTER_SPEC>`"]
pub type S_POINTER = crate::Reg<s_pointer::S_POINTER_SPEC>;
#[doc = "Pointer for CSB master and data path to access groups"]
pub mod s_pointer;
#[doc = "D_OP_ENABLE register accessor: an alias for `Reg<D_OP_ENABLE_SPEC>`"]
pub type D_OP_ENABLE = crate::Reg<d_op_enable::D_OP_ENABLE_SPEC>;
#[doc = "Set it to 1 to kick off operation for current register group"]
pub mod d_op_enable;
#[doc = "D_DATA_CUBE_WIDTH register accessor: an alias for `Reg<D_DATA_CUBE_WIDTH_SPEC>`"]
pub type D_DATA_CUBE_WIDTH = crate::Reg<d_data_cube_width::D_DATA_CUBE_WIDTH_SPEC>;
#[doc = ""]
pub mod d_data_cube_width;
#[doc = "D_DATA_CUBE_HEIGHT register accessor: an alias for `Reg<D_DATA_CUBE_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_HEIGHT = crate::Reg<d_data_cube_height::D_DATA_CUBE_HEIGHT_SPEC>;
#[doc = ""]
pub mod d_data_cube_height;
#[doc = "D_DATA_CUBE_CHANNEL register accessor: an alias for `Reg<D_DATA_CUBE_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_CHANNEL = crate::Reg<d_data_cube_channel::D_DATA_CUBE_CHANNEL_SPEC>;
#[doc = ""]
pub mod d_data_cube_channel;
#[doc = "D_SRC_BASE_ADDR_LOW register accessor: an alias for `Reg<D_SRC_BASE_ADDR_LOW_SPEC>`"]
pub type D_SRC_BASE_ADDR_LOW = crate::Reg<d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>;
#[doc = ""]
pub mod d_src_base_addr_low;
#[doc = "D_SRC_BASE_ADDR_HIGH register accessor: an alias for `Reg<D_SRC_BASE_ADDR_HIGH_SPEC>`"]
pub type D_SRC_BASE_ADDR_HIGH = crate::Reg<d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>;
#[doc = ""]
pub mod d_src_base_addr_high;
#[doc = "D_SRC_LINE_STRIDE register accessor: an alias for `Reg<D_SRC_LINE_STRIDE_SPEC>`"]
pub type D_SRC_LINE_STRIDE = crate::Reg<d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_src_line_stride;
#[doc = "D_SRC_SURFACE_STRIDE register accessor: an alias for `Reg<D_SRC_SURFACE_STRIDE_SPEC>`"]
pub type D_SRC_SURFACE_STRIDE = crate::Reg<d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_src_surface_stride;
#[doc = "D_BRDMA_CFG register accessor: an alias for `Reg<D_BRDMA_CFG_SPEC>`"]
pub type D_BRDMA_CFG = crate::Reg<d_brdma_cfg::D_BRDMA_CFG_SPEC>;
#[doc = ""]
pub mod d_brdma_cfg;
#[doc = "D_BS_BASE_ADDR_LOW register accessor: an alias for `Reg<D_BS_BASE_ADDR_LOW_SPEC>`"]
pub type D_BS_BASE_ADDR_LOW = crate::Reg<d_bs_base_addr_low::D_BS_BASE_ADDR_LOW_SPEC>;
#[doc = ""]
pub mod d_bs_base_addr_low;
#[doc = "D_BS_BASE_ADDR_HIGH register accessor: an alias for `Reg<D_BS_BASE_ADDR_HIGH_SPEC>`"]
pub type D_BS_BASE_ADDR_HIGH = crate::Reg<d_bs_base_addr_high::D_BS_BASE_ADDR_HIGH_SPEC>;
#[doc = ""]
pub mod d_bs_base_addr_high;
#[doc = "D_BS_LINE_STRIDE register accessor: an alias for `Reg<D_BS_LINE_STRIDE_SPEC>`"]
pub type D_BS_LINE_STRIDE = crate::Reg<d_bs_line_stride::D_BS_LINE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_bs_line_stride;
#[doc = "D_BS_SURFACE_STRIDE register accessor: an alias for `Reg<D_BS_SURFACE_STRIDE_SPEC>`"]
pub type D_BS_SURFACE_STRIDE = crate::Reg<d_bs_surface_stride::D_BS_SURFACE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_bs_surface_stride;
#[doc = "D_BS_BATCH_STRIDE register accessor: an alias for `Reg<D_BS_BATCH_STRIDE_SPEC>`"]
pub type D_BS_BATCH_STRIDE = crate::Reg<d_bs_batch_stride::D_BS_BATCH_STRIDE_SPEC>;
#[doc = ""]
pub mod d_bs_batch_stride;
#[doc = "D_NRDMA_CFG register accessor: an alias for `Reg<D_NRDMA_CFG_SPEC>`"]
pub type D_NRDMA_CFG = crate::Reg<d_nrdma_cfg::D_NRDMA_CFG_SPEC>;
#[doc = ""]
pub mod d_nrdma_cfg;
#[doc = "D_BN_BASE_ADDR_LOW register accessor: an alias for `Reg<D_BN_BASE_ADDR_LOW_SPEC>`"]
pub type D_BN_BASE_ADDR_LOW = crate::Reg<d_bn_base_addr_low::D_BN_BASE_ADDR_LOW_SPEC>;
#[doc = ""]
pub mod d_bn_base_addr_low;
#[doc = "D_BN_BASE_ADDR_HIGH register accessor: an alias for `Reg<D_BN_BASE_ADDR_HIGH_SPEC>`"]
pub type D_BN_BASE_ADDR_HIGH = crate::Reg<d_bn_base_addr_high::D_BN_BASE_ADDR_HIGH_SPEC>;
#[doc = ""]
pub mod d_bn_base_addr_high;
#[doc = "D_BN_LINE_STRIDE register accessor: an alias for `Reg<D_BN_LINE_STRIDE_SPEC>`"]
pub type D_BN_LINE_STRIDE = crate::Reg<d_bn_line_stride::D_BN_LINE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_bn_line_stride;
#[doc = "D_BN_SURFACE_STRIDE register accessor: an alias for `Reg<D_BN_SURFACE_STRIDE_SPEC>`"]
pub type D_BN_SURFACE_STRIDE = crate::Reg<d_bn_surface_stride::D_BN_SURFACE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_bn_surface_stride;
#[doc = "D_BN_BATCH_STRIDE register accessor: an alias for `Reg<D_BN_BATCH_STRIDE_SPEC>`"]
pub type D_BN_BATCH_STRIDE = crate::Reg<d_bn_batch_stride::D_BN_BATCH_STRIDE_SPEC>;
#[doc = ""]
pub mod d_bn_batch_stride;
#[doc = "D_ERDMA_CFG register accessor: an alias for `Reg<D_ERDMA_CFG_SPEC>`"]
pub type D_ERDMA_CFG = crate::Reg<d_erdma_cfg::D_ERDMA_CFG_SPEC>;
#[doc = ""]
pub mod d_erdma_cfg;
#[doc = "D_EW_BASE_ADDR_LOW register accessor: an alias for `Reg<D_EW_BASE_ADDR_LOW_SPEC>`"]
pub type D_EW_BASE_ADDR_LOW = crate::Reg<d_ew_base_addr_low::D_EW_BASE_ADDR_LOW_SPEC>;
#[doc = ""]
pub mod d_ew_base_addr_low;
#[doc = "D_EW_BASE_ADDR_HIGH register accessor: an alias for `Reg<D_EW_BASE_ADDR_HIGH_SPEC>`"]
pub type D_EW_BASE_ADDR_HIGH = crate::Reg<d_ew_base_addr_high::D_EW_BASE_ADDR_HIGH_SPEC>;
#[doc = ""]
pub mod d_ew_base_addr_high;
#[doc = "D_EW_LINE_STRIDE register accessor: an alias for `Reg<D_EW_LINE_STRIDE_SPEC>`"]
pub type D_EW_LINE_STRIDE = crate::Reg<d_ew_line_stride::D_EW_LINE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_ew_line_stride;
#[doc = "D_EW_SURFACE_STRIDE register accessor: an alias for `Reg<D_EW_SURFACE_STRIDE_SPEC>`"]
pub type D_EW_SURFACE_STRIDE = crate::Reg<d_ew_surface_stride::D_EW_SURFACE_STRIDE_SPEC>;
#[doc = ""]
pub mod d_ew_surface_stride;
#[doc = "D_EW_BATCH_STRIDE register accessor: an alias for `Reg<D_EW_BATCH_STRIDE_SPEC>`"]
pub type D_EW_BATCH_STRIDE = crate::Reg<d_ew_batch_stride::D_EW_BATCH_STRIDE_SPEC>;
#[doc = ""]
pub mod d_ew_batch_stride;
#[doc = "D_FEATURE_MODE_CFG register accessor: an alias for `Reg<D_FEATURE_MODE_CFG_SPEC>`"]
pub type D_FEATURE_MODE_CFG = crate::Reg<d_feature_mode_cfg::D_FEATURE_MODE_CFG_SPEC>;
#[doc = ""]
pub mod d_feature_mode_cfg;
#[doc = "D_SRC_DMA_CFG register accessor: an alias for `Reg<D_SRC_DMA_CFG_SPEC>`"]
pub type D_SRC_DMA_CFG = crate::Reg<d_src_dma_cfg::D_SRC_DMA_CFG_SPEC>;
#[doc = ""]
pub mod d_src_dma_cfg;
#[doc = "D_STATUS_NAN_INPUT_NUM register accessor: an alias for `Reg<D_STATUS_NAN_INPUT_NUM_SPEC>`"]
pub type D_STATUS_NAN_INPUT_NUM = crate::Reg<d_status_nan_input_num::D_STATUS_NAN_INPUT_NUM_SPEC>;
#[doc = ""]
pub mod d_status_nan_input_num;
#[doc = "D_STATUS_INF_INPUT_NUM register accessor: an alias for `Reg<D_STATUS_INF_INPUT_NUM_SPEC>`"]
pub type D_STATUS_INF_INPUT_NUM = crate::Reg<d_status_inf_input_num::D_STATUS_INF_INPUT_NUM_SPEC>;
#[doc = ""]
pub mod d_status_inf_input_num;
#[doc = "D_PERF_ERDMA_READ_STALL register accessor: an alias for `Reg<D_PERF_ERDMA_READ_STALL_SPEC>`"]
pub type D_PERF_ERDMA_READ_STALL =
    crate::Reg<d_perf_erdma_read_stall::D_PERF_ERDMA_READ_STALL_SPEC>;
#[doc = ""]
pub mod d_perf_erdma_read_stall;
#[doc = "D_PERF_BRDMA_READ_STALL register accessor: an alias for `Reg<D_PERF_BRDMA_READ_STALL_SPEC>`"]
pub type D_PERF_BRDMA_READ_STALL =
    crate::Reg<d_perf_brdma_read_stall::D_PERF_BRDMA_READ_STALL_SPEC>;
#[doc = ""]
pub mod d_perf_brdma_read_stall;
#[doc = "D_PERF_ENABLE register accessor: an alias for `Reg<D_PERF_ENABLE_SPEC>`"]
pub type D_PERF_ENABLE = crate::Reg<d_perf_enable::D_PERF_ENABLE_SPEC>;
#[doc = ""]
pub mod d_perf_enable;
#[doc = "D_PERF_NRDMA_READ_STALL register accessor: an alias for `Reg<D_PERF_NRDMA_READ_STALL_SPEC>`"]
pub type D_PERF_NRDMA_READ_STALL =
    crate::Reg<d_perf_nrdma_read_stall::D_PERF_NRDMA_READ_STALL_SPEC>;
#[doc = ""]
pub mod d_perf_nrdma_read_stall;
#[doc = "D_PERF_MRDMA_READ_STALL register accessor: an alias for `Reg<D_PERF_MRDMA_READ_STALL_SPEC>`"]
pub type D_PERF_MRDMA_READ_STALL =
    crate::Reg<d_perf_mrdma_read_stall::D_PERF_MRDMA_READ_STALL_SPEC>;
#[doc = ""]
pub mod d_perf_mrdma_read_stall;
