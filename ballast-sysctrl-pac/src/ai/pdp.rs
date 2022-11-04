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
#[doc = "D_DATA_CUBE_IN_WIDTH register accessor: an alias for `Reg<D_DATA_CUBE_IN_WIDTH_SPEC>`"]
pub type D_DATA_CUBE_IN_WIDTH = crate::Reg<d_data_cube_in_width::D_DATA_CUBE_IN_WIDTH_SPEC>;
#[doc = "Input data cubes width"]
pub mod d_data_cube_in_width;
#[doc = "D_DATA_CUBE_IN_HEIGHT register accessor: an alias for `Reg<D_DATA_CUBE_IN_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_IN_HEIGHT = crate::Reg<d_data_cube_in_height::D_DATA_CUBE_IN_HEIGHT_SPEC>;
#[doc = "Input data cubes height"]
pub mod d_data_cube_in_height;
#[doc = "D_DATA_CUBE_IN_CHANNEL register accessor: an alias for `Reg<D_DATA_CUBE_IN_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_IN_CHANNEL = crate::Reg<d_data_cube_in_channel::D_DATA_CUBE_IN_CHANNEL_SPEC>;
#[doc = "Input data cubes channel"]
pub mod d_data_cube_in_channel;
#[doc = "D_DATA_CUBE_OUT_WIDTH register accessor: an alias for `Reg<D_DATA_CUBE_OUT_WIDTH_SPEC>`"]
pub type D_DATA_CUBE_OUT_WIDTH = crate::Reg<d_data_cube_out_width::D_DATA_CUBE_OUT_WIDTH_SPEC>;
#[doc = "Output data cubes width"]
pub mod d_data_cube_out_width;
#[doc = "D_DATA_CUBE_OUT_HEIGHT register accessor: an alias for `Reg<D_DATA_CUBE_OUT_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_OUT_HEIGHT = crate::Reg<d_data_cube_out_height::D_DATA_CUBE_OUT_HEIGHT_SPEC>;
#[doc = "Output data cubes height"]
pub mod d_data_cube_out_height;
#[doc = "D_DATA_CUBE_OUT_CHANNEL register accessor: an alias for `Reg<D_DATA_CUBE_OUT_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_OUT_CHANNEL =
    crate::Reg<d_data_cube_out_channel::D_DATA_CUBE_OUT_CHANNEL_SPEC>;
#[doc = "Output data cubes channel"]
pub mod d_data_cube_out_channel;
#[doc = "D_OPERATION_MODE_CFG register accessor: an alias for `Reg<D_OPERATION_MODE_CFG_SPEC>`"]
pub type D_OPERATION_MODE_CFG = crate::Reg<d_operation_mode_cfg::D_OPERATION_MODE_CFG_SPEC>;
#[doc = "Split number"]
pub mod d_operation_mode_cfg;
#[doc = "D_NAN_FLUSH_TO_ZERO register accessor: an alias for `Reg<D_NAN_FLUSH_TO_ZERO_SPEC>`"]
pub type D_NAN_FLUSH_TO_ZERO = crate::Reg<d_nan_flush_to_zero::D_NAN_FLUSH_TO_ZERO_SPEC>;
#[doc = "Option to flush input NaN to zero"]
pub mod d_nan_flush_to_zero;
#[doc = "D_PARTIAL_WIDTH_IN register accessor: an alias for `Reg<D_PARTIAL_WIDTH_IN_SPEC>`"]
pub type D_PARTIAL_WIDTH_IN = crate::Reg<d_partial_width_in::D_PARTIAL_WIDTH_IN_SPEC>;
#[doc = "Partial width for first, last and middle partitions of input cube"]
pub mod d_partial_width_in;
#[doc = "D_PARTIAL_WIDTH_OUT register accessor: an alias for `Reg<D_PARTIAL_WIDTH_OUT_SPEC>`"]
pub type D_PARTIAL_WIDTH_OUT = crate::Reg<d_partial_width_out::D_PARTIAL_WIDTH_OUT_SPEC>;
#[doc = "Partial width for first, last and middle partitions of output cube"]
pub mod d_partial_width_out;
#[doc = "D_POOLING_KERNEL_CFG register accessor: an alias for `Reg<D_POOLING_KERNEL_CFG_SPEC>`"]
pub type D_POOLING_KERNEL_CFG = crate::Reg<d_pooling_kernel_cfg::D_POOLING_KERNEL_CFG_SPEC>;
#[doc = "Kernel width and kernel stride"]
pub mod d_pooling_kernel_cfg;
#[doc = "D_RECIP_KERNEL_WIDTH register accessor: an alias for `Reg<D_RECIP_KERNEL_WIDTH_SPEC>`"]
pub type D_RECIP_KERNEL_WIDTH = crate::Reg<d_recip_kernel_width::D_RECIP_KERNEL_WIDTH_SPEC>;
#[doc = "Reciprocal of pooling kernel width, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
pub mod d_recip_kernel_width;
#[doc = "D_RECIP_KERNEL_HEIGHT register accessor: an alias for `Reg<D_RECIP_KERNEL_HEIGHT_SPEC>`"]
pub type D_RECIP_KERNEL_HEIGHT = crate::Reg<d_recip_kernel_height::D_RECIP_KERNEL_HEIGHT_SPEC>;
#[doc = "Reciprocal of pooling kernel height, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
pub mod d_recip_kernel_height;
#[doc = "D_POOLING_PADDING_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_CFG_SPEC>`"]
pub type D_POOLING_PADDING_CFG = crate::Reg<d_pooling_padding_cfg::D_POOLING_PADDING_CFG_SPEC>;
#[doc = "Left/right/top/bottom padding size"]
pub mod d_pooling_padding_cfg;
#[doc = "D_POOLING_PADDING_VALUE_1_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_1_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_1_CFG =
    crate::Reg<d_pooling_padding_value_1_cfg::D_POOLING_PADDING_VALUE_1_CFG_SPEC>;
#[doc = "Padding_value*1"]
pub mod d_pooling_padding_value_1_cfg;
#[doc = "D_POOLING_PADDING_VALUE_2_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_2_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_2_CFG =
    crate::Reg<d_pooling_padding_value_2_cfg::D_POOLING_PADDING_VALUE_2_CFG_SPEC>;
#[doc = "Padding_value*2"]
pub mod d_pooling_padding_value_2_cfg;
#[doc = "D_POOLING_PADDING_VALUE_3_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_3_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_3_CFG =
    crate::Reg<d_pooling_padding_value_3_cfg::D_POOLING_PADDING_VALUE_3_CFG_SPEC>;
#[doc = "Padding_value*3"]
pub mod d_pooling_padding_value_3_cfg;
#[doc = "D_POOLING_PADDING_VALUE_4_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_4_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_4_CFG =
    crate::Reg<d_pooling_padding_value_4_cfg::D_POOLING_PADDING_VALUE_4_CFG_SPEC>;
#[doc = "Padding_value*4"]
pub mod d_pooling_padding_value_4_cfg;
#[doc = "D_POOLING_PADDING_VALUE_5_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_5_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_5_CFG =
    crate::Reg<d_pooling_padding_value_5_cfg::D_POOLING_PADDING_VALUE_5_CFG_SPEC>;
#[doc = "Padding_value*5"]
pub mod d_pooling_padding_value_5_cfg;
#[doc = "D_POOLING_PADDING_VALUE_6_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_6_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_6_CFG =
    crate::Reg<d_pooling_padding_value_6_cfg::D_POOLING_PADDING_VALUE_6_CFG_SPEC>;
#[doc = "Padding_value*6"]
pub mod d_pooling_padding_value_6_cfg;
#[doc = "D_POOLING_PADDING_VALUE_7_CFG register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_7_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_7_CFG =
    crate::Reg<d_pooling_padding_value_7_cfg::D_POOLING_PADDING_VALUE_7_CFG_SPEC>;
#[doc = "Padding_value*7"]
pub mod d_pooling_padding_value_7_cfg;
#[doc = "D_SRC_BASE_ADDR_LOW register accessor: an alias for `Reg<D_SRC_BASE_ADDR_LOW_SPEC>`"]
pub type D_SRC_BASE_ADDR_LOW = crate::Reg<d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of input data address"]
pub mod d_src_base_addr_low;
#[doc = "D_SRC_BASE_ADDR_HIGH register accessor: an alias for `Reg<D_SRC_BASE_ADDR_HIGH_SPEC>`"]
pub type D_SRC_BASE_ADDR_HIGH = crate::Reg<d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of input data address when axi araddr is 64bits"]
pub mod d_src_base_addr_high;
#[doc = "D_SRC_LINE_STRIDE register accessor: an alias for `Reg<D_SRC_LINE_STRIDE_SPEC>`"]
pub type D_SRC_LINE_STRIDE = crate::Reg<d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>;
#[doc = "Line stride of input cube"]
pub mod d_src_line_stride;
#[doc = "D_SRC_SURFACE_STRIDE register accessor: an alias for `Reg<D_SRC_SURFACE_STRIDE_SPEC>`"]
pub type D_SRC_SURFACE_STRIDE = crate::Reg<d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>;
#[doc = "Surface stride of input cube"]
pub mod d_src_surface_stride;
#[doc = "D_DST_BASE_ADDR_LOW register accessor: an alias for `Reg<D_DST_BASE_ADDR_LOW_SPEC>`"]
pub type D_DST_BASE_ADDR_LOW = crate::Reg<d_dst_base_addr_low::D_DST_BASE_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of output data address"]
pub mod d_dst_base_addr_low;
#[doc = "D_DST_BASE_ADDR_HIGH register accessor: an alias for `Reg<D_DST_BASE_ADDR_HIGH_SPEC>`"]
pub type D_DST_BASE_ADDR_HIGH = crate::Reg<d_dst_base_addr_high::D_DST_BASE_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of output data address when axi awaddr is 64bits"]
pub mod d_dst_base_addr_high;
#[doc = "D_DST_LINE_STRIDE register accessor: an alias for `Reg<D_DST_LINE_STRIDE_SPEC>`"]
pub type D_DST_LINE_STRIDE = crate::Reg<d_dst_line_stride::D_DST_LINE_STRIDE_SPEC>;
#[doc = "Line stride of output cube"]
pub mod d_dst_line_stride;
#[doc = "D_DST_SURFACE_STRIDE register accessor: an alias for `Reg<D_DST_SURFACE_STRIDE_SPEC>`"]
pub type D_DST_SURFACE_STRIDE = crate::Reg<d_dst_surface_stride::D_DST_SURFACE_STRIDE_SPEC>;
#[doc = "Surface stride of output cube"]
pub mod d_dst_surface_stride;
#[doc = "D_DST_RAM_CFG register accessor: an alias for `Reg<D_DST_RAM_CFG_SPEC>`"]
pub type D_DST_RAM_CFG = crate::Reg<d_dst_ram_cfg::D_DST_RAM_CFG_SPEC>;
#[doc = "RAM type of destination cube"]
pub mod d_dst_ram_cfg;
#[doc = "D_DATA_FORMAT register accessor: an alias for `Reg<D_DATA_FORMAT_SPEC>`"]
pub type D_DATA_FORMAT = crate::Reg<d_data_format::D_DATA_FORMAT_SPEC>;
#[doc = "Precision of input data"]
pub mod d_data_format;
#[doc = "D_INF_INPUT_NUM register accessor: an alias for `Reg<D_INF_INPUT_NUM_SPEC>`"]
pub type D_INF_INPUT_NUM = crate::Reg<d_inf_input_num::D_INF_INPUT_NUM_SPEC>;
#[doc = "Input infinity element number"]
pub mod d_inf_input_num;
#[doc = "D_NAN_INPUT_NUM register accessor: an alias for `Reg<D_NAN_INPUT_NUM_SPEC>`"]
pub type D_NAN_INPUT_NUM = crate::Reg<d_nan_input_num::D_NAN_INPUT_NUM_SPEC>;
#[doc = "Input NaN element number"]
pub mod d_nan_input_num;
#[doc = "D_NAN_OUTPUT_NUM register accessor: an alias for `Reg<D_NAN_OUTPUT_NUM_SPEC>`"]
pub type D_NAN_OUTPUT_NUM = crate::Reg<d_nan_output_num::D_NAN_OUTPUT_NUM_SPEC>;
#[doc = "Output NaN element number"]
pub mod d_nan_output_num;
#[doc = "D_PERF_ENABLE register accessor: an alias for `Reg<D_PERF_ENABLE_SPEC>`"]
pub type D_PERF_ENABLE = crate::Reg<d_perf_enable::D_PERF_ENABLE_SPEC>;
#[doc = "Enable/disable performance counting"]
pub mod d_perf_enable;
#[doc = "D_PERF_WRITE_STALL register accessor: an alias for `Reg<D_PERF_WRITE_STALL_SPEC>`"]
pub type D_PERF_WRITE_STALL = crate::Reg<d_perf_write_stall::D_PERF_WRITE_STALL_SPEC>;
#[doc = "Counting stalls of write requests"]
pub mod d_perf_write_stall;
#[doc = "D_CYA register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
