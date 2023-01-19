#[doc = r"Register block"]
#[repr(C)]
pub struct PDP {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: S_STATUS,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: S_POINTER,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: D_OP_ENABLE,
    #[doc = "0x0c - Input data cubes width"]
    pub d_data_cube_in_width: D_DATA_CUBE_IN_WIDTH,
    #[doc = "0x10 - Input data cubes height"]
    pub d_data_cube_in_height: D_DATA_CUBE_IN_HEIGHT,
    #[doc = "0x14 - Input data cubes channel"]
    pub d_data_cube_in_channel: D_DATA_CUBE_IN_CHANNEL,
    #[doc = "0x18 - Output data cubes width"]
    pub d_data_cube_out_width: D_DATA_CUBE_OUT_WIDTH,
    #[doc = "0x1c - Output data cubes height"]
    pub d_data_cube_out_height: D_DATA_CUBE_OUT_HEIGHT,
    #[doc = "0x20 - Output data cubes channel"]
    pub d_data_cube_out_channel: D_DATA_CUBE_OUT_CHANNEL,
    #[doc = "0x24 - Split number"]
    pub d_operation_mode_cfg: D_OPERATION_MODE_CFG,
    #[doc = "0x28 - Option to flush input NaN to zero"]
    pub d_nan_flush_to_zero: D_NAN_FLUSH_TO_ZERO,
    #[doc = "0x2c - Partial width for first, last and middle partitions of input cube"]
    pub d_partial_width_in: D_PARTIAL_WIDTH_IN,
    #[doc = "0x30 - Partial width for first, last and middle partitions of output cube"]
    pub d_partial_width_out: D_PARTIAL_WIDTH_OUT,
    #[doc = "0x34 - Kernel width and kernel stride"]
    pub d_pooling_kernel_cfg: D_POOLING_KERNEL_CFG,
    #[doc = "0x38 - Reciprocal of pooling kernel width, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
    pub d_recip_kernel_width: D_RECIP_KERNEL_WIDTH,
    #[doc = "0x3c - Reciprocal of pooling kernel height, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
    pub d_recip_kernel_height: D_RECIP_KERNEL_HEIGHT,
    #[doc = "0x40 - Left/right/top/bottom padding size"]
    pub d_pooling_padding_cfg: D_POOLING_PADDING_CFG,
    #[doc = "0x44 - Padding_value*1"]
    pub d_pooling_padding_value_1_cfg: D_POOLING_PADDING_VALUE_1_CFG,
    #[doc = "0x48 - Padding_value*2"]
    pub d_pooling_padding_value_2_cfg: D_POOLING_PADDING_VALUE_2_CFG,
    #[doc = "0x4c - Padding_value*3"]
    pub d_pooling_padding_value_3_cfg: D_POOLING_PADDING_VALUE_3_CFG,
    #[doc = "0x50 - Padding_value*4"]
    pub d_pooling_padding_value_4_cfg: D_POOLING_PADDING_VALUE_4_CFG,
    #[doc = "0x54 - Padding_value*5"]
    pub d_pooling_padding_value_5_cfg: D_POOLING_PADDING_VALUE_5_CFG,
    #[doc = "0x58 - Padding_value*6"]
    pub d_pooling_padding_value_6_cfg: D_POOLING_PADDING_VALUE_6_CFG,
    #[doc = "0x5c - Padding_value*7"]
    pub d_pooling_padding_value_7_cfg: D_POOLING_PADDING_VALUE_7_CFG,
    #[doc = "0x60 - Lower 32bits of input data address"]
    pub d_src_base_addr_low: D_SRC_BASE_ADDR_LOW,
    #[doc = "0x64 - Higher 32bits of input data address when axi araddr is 64bits"]
    pub d_src_base_addr_high: D_SRC_BASE_ADDR_HIGH,
    #[doc = "0x68 - Line stride of input cube"]
    pub d_src_line_stride: D_SRC_LINE_STRIDE,
    #[doc = "0x6c - Surface stride of input cube"]
    pub d_src_surface_stride: D_SRC_SURFACE_STRIDE,
    #[doc = "0x70 - Lower 32bits of output data address"]
    pub d_dst_base_addr_low: D_DST_BASE_ADDR_LOW,
    #[doc = "0x74 - Higher 32bits of output data address when axi awaddr is 64bits"]
    pub d_dst_base_addr_high: D_DST_BASE_ADDR_HIGH,
    #[doc = "0x78 - Line stride of output cube"]
    pub d_dst_line_stride: D_DST_LINE_STRIDE,
    #[doc = "0x7c - Surface stride of output cube"]
    pub d_dst_surface_stride: D_DST_SURFACE_STRIDE,
    #[doc = "0x80 - RAM type of destination cube"]
    pub d_dst_ram_cfg: D_DST_RAM_CFG,
    #[doc = "0x84 - Precision of input data"]
    pub d_data_format: D_DATA_FORMAT,
    #[doc = "0x88 - Input infinity element number"]
    pub d_inf_input_num: D_INF_INPUT_NUM,
    #[doc = "0x8c - Input NaN element number"]
    pub d_nan_input_num: D_NAN_INPUT_NUM,
    #[doc = "0x90 - Output NaN element number"]
    pub d_nan_output_num: D_NAN_OUTPUT_NUM,
    #[doc = "0x94 - Enable/disable performance counting"]
    pub d_perf_enable: D_PERF_ENABLE,
    #[doc = "0x98 - Counting stalls of write requests"]
    pub d_perf_write_stall: D_PERF_WRITE_STALL,
    #[doc = "0x9c - "]
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
#[doc = "Input data cubes width"]
pub mod d_data_cube_in_width;
#[doc = "D_DATA_CUBE_IN_HEIGHT (rw) register accessor: an alias for `Reg<D_DATA_CUBE_IN_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_IN_HEIGHT = crate::Reg<d_data_cube_in_height::D_DATA_CUBE_IN_HEIGHT_SPEC>;
#[doc = "Input data cubes height"]
pub mod d_data_cube_in_height;
#[doc = "D_DATA_CUBE_IN_CHANNEL (rw) register accessor: an alias for `Reg<D_DATA_CUBE_IN_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_IN_CHANNEL = crate::Reg<d_data_cube_in_channel::D_DATA_CUBE_IN_CHANNEL_SPEC>;
#[doc = "Input data cubes channel"]
pub mod d_data_cube_in_channel;
#[doc = "D_DATA_CUBE_OUT_WIDTH (rw) register accessor: an alias for `Reg<D_DATA_CUBE_OUT_WIDTH_SPEC>`"]
pub type D_DATA_CUBE_OUT_WIDTH = crate::Reg<d_data_cube_out_width::D_DATA_CUBE_OUT_WIDTH_SPEC>;
#[doc = "Output data cubes width"]
pub mod d_data_cube_out_width;
#[doc = "D_DATA_CUBE_OUT_HEIGHT (rw) register accessor: an alias for `Reg<D_DATA_CUBE_OUT_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_OUT_HEIGHT = crate::Reg<d_data_cube_out_height::D_DATA_CUBE_OUT_HEIGHT_SPEC>;
#[doc = "Output data cubes height"]
pub mod d_data_cube_out_height;
#[doc = "D_DATA_CUBE_OUT_CHANNEL (rw) register accessor: an alias for `Reg<D_DATA_CUBE_OUT_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_OUT_CHANNEL =
    crate::Reg<d_data_cube_out_channel::D_DATA_CUBE_OUT_CHANNEL_SPEC>;
#[doc = "Output data cubes channel"]
pub mod d_data_cube_out_channel;
#[doc = "D_OPERATION_MODE_CFG (rw) register accessor: an alias for `Reg<D_OPERATION_MODE_CFG_SPEC>`"]
pub type D_OPERATION_MODE_CFG = crate::Reg<d_operation_mode_cfg::D_OPERATION_MODE_CFG_SPEC>;
#[doc = "Split number"]
pub mod d_operation_mode_cfg;
#[doc = "D_NAN_FLUSH_TO_ZERO (rw) register accessor: an alias for `Reg<D_NAN_FLUSH_TO_ZERO_SPEC>`"]
pub type D_NAN_FLUSH_TO_ZERO = crate::Reg<d_nan_flush_to_zero::D_NAN_FLUSH_TO_ZERO_SPEC>;
#[doc = "Option to flush input NaN to zero"]
pub mod d_nan_flush_to_zero;
#[doc = "D_PARTIAL_WIDTH_IN (rw) register accessor: an alias for `Reg<D_PARTIAL_WIDTH_IN_SPEC>`"]
pub type D_PARTIAL_WIDTH_IN = crate::Reg<d_partial_width_in::D_PARTIAL_WIDTH_IN_SPEC>;
#[doc = "Partial width for first, last and middle partitions of input cube"]
pub mod d_partial_width_in;
#[doc = "D_PARTIAL_WIDTH_OUT (rw) register accessor: an alias for `Reg<D_PARTIAL_WIDTH_OUT_SPEC>`"]
pub type D_PARTIAL_WIDTH_OUT = crate::Reg<d_partial_width_out::D_PARTIAL_WIDTH_OUT_SPEC>;
#[doc = "Partial width for first, last and middle partitions of output cube"]
pub mod d_partial_width_out;
#[doc = "D_POOLING_KERNEL_CFG (rw) register accessor: an alias for `Reg<D_POOLING_KERNEL_CFG_SPEC>`"]
pub type D_POOLING_KERNEL_CFG = crate::Reg<d_pooling_kernel_cfg::D_POOLING_KERNEL_CFG_SPEC>;
#[doc = "Kernel width and kernel stride"]
pub mod d_pooling_kernel_cfg;
#[doc = "D_RECIP_KERNEL_WIDTH (rw) register accessor: an alias for `Reg<D_RECIP_KERNEL_WIDTH_SPEC>`"]
pub type D_RECIP_KERNEL_WIDTH = crate::Reg<d_recip_kernel_width::D_RECIP_KERNEL_WIDTH_SPEC>;
#[doc = "Reciprocal of pooling kernel width, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
pub mod d_recip_kernel_width;
#[doc = "D_RECIP_KERNEL_HEIGHT (rw) register accessor: an alias for `Reg<D_RECIP_KERNEL_HEIGHT_SPEC>`"]
pub type D_RECIP_KERNEL_HEIGHT = crate::Reg<d_recip_kernel_height::D_RECIP_KERNEL_HEIGHT_SPEC>;
#[doc = "Reciprocal of pooling kernel height, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
pub mod d_recip_kernel_height;
#[doc = "D_POOLING_PADDING_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_CFG_SPEC>`"]
pub type D_POOLING_PADDING_CFG = crate::Reg<d_pooling_padding_cfg::D_POOLING_PADDING_CFG_SPEC>;
#[doc = "Left/right/top/bottom padding size"]
pub mod d_pooling_padding_cfg;
#[doc = "D_POOLING_PADDING_VALUE_1_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_1_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_1_CFG =
    crate::Reg<d_pooling_padding_value_1_cfg::D_POOLING_PADDING_VALUE_1_CFG_SPEC>;
#[doc = "Padding_value*1"]
pub mod d_pooling_padding_value_1_cfg;
#[doc = "D_POOLING_PADDING_VALUE_2_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_2_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_2_CFG =
    crate::Reg<d_pooling_padding_value_2_cfg::D_POOLING_PADDING_VALUE_2_CFG_SPEC>;
#[doc = "Padding_value*2"]
pub mod d_pooling_padding_value_2_cfg;
#[doc = "D_POOLING_PADDING_VALUE_3_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_3_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_3_CFG =
    crate::Reg<d_pooling_padding_value_3_cfg::D_POOLING_PADDING_VALUE_3_CFG_SPEC>;
#[doc = "Padding_value*3"]
pub mod d_pooling_padding_value_3_cfg;
#[doc = "D_POOLING_PADDING_VALUE_4_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_4_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_4_CFG =
    crate::Reg<d_pooling_padding_value_4_cfg::D_POOLING_PADDING_VALUE_4_CFG_SPEC>;
#[doc = "Padding_value*4"]
pub mod d_pooling_padding_value_4_cfg;
#[doc = "D_POOLING_PADDING_VALUE_5_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_5_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_5_CFG =
    crate::Reg<d_pooling_padding_value_5_cfg::D_POOLING_PADDING_VALUE_5_CFG_SPEC>;
#[doc = "Padding_value*5"]
pub mod d_pooling_padding_value_5_cfg;
#[doc = "D_POOLING_PADDING_VALUE_6_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_6_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_6_CFG =
    crate::Reg<d_pooling_padding_value_6_cfg::D_POOLING_PADDING_VALUE_6_CFG_SPEC>;
#[doc = "Padding_value*6"]
pub mod d_pooling_padding_value_6_cfg;
#[doc = "D_POOLING_PADDING_VALUE_7_CFG (rw) register accessor: an alias for `Reg<D_POOLING_PADDING_VALUE_7_CFG_SPEC>`"]
pub type D_POOLING_PADDING_VALUE_7_CFG =
    crate::Reg<d_pooling_padding_value_7_cfg::D_POOLING_PADDING_VALUE_7_CFG_SPEC>;
#[doc = "Padding_value*7"]
pub mod d_pooling_padding_value_7_cfg;
#[doc = "D_SRC_BASE_ADDR_LOW (rw) register accessor: an alias for `Reg<D_SRC_BASE_ADDR_LOW_SPEC>`"]
pub type D_SRC_BASE_ADDR_LOW = crate::Reg<d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of input data address"]
pub mod d_src_base_addr_low;
#[doc = "D_SRC_BASE_ADDR_HIGH (rw) register accessor: an alias for `Reg<D_SRC_BASE_ADDR_HIGH_SPEC>`"]
pub type D_SRC_BASE_ADDR_HIGH = crate::Reg<d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of input data address when axi araddr is 64bits"]
pub mod d_src_base_addr_high;
#[doc = "D_SRC_LINE_STRIDE (rw) register accessor: an alias for `Reg<D_SRC_LINE_STRIDE_SPEC>`"]
pub type D_SRC_LINE_STRIDE = crate::Reg<d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>;
#[doc = "Line stride of input cube"]
pub mod d_src_line_stride;
#[doc = "D_SRC_SURFACE_STRIDE (rw) register accessor: an alias for `Reg<D_SRC_SURFACE_STRIDE_SPEC>`"]
pub type D_SRC_SURFACE_STRIDE = crate::Reg<d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>;
#[doc = "Surface stride of input cube"]
pub mod d_src_surface_stride;
#[doc = "D_DST_BASE_ADDR_LOW (rw) register accessor: an alias for `Reg<D_DST_BASE_ADDR_LOW_SPEC>`"]
pub type D_DST_BASE_ADDR_LOW = crate::Reg<d_dst_base_addr_low::D_DST_BASE_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of output data address"]
pub mod d_dst_base_addr_low;
#[doc = "D_DST_BASE_ADDR_HIGH (rw) register accessor: an alias for `Reg<D_DST_BASE_ADDR_HIGH_SPEC>`"]
pub type D_DST_BASE_ADDR_HIGH = crate::Reg<d_dst_base_addr_high::D_DST_BASE_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of output data address when axi awaddr is 64bits"]
pub mod d_dst_base_addr_high;
#[doc = "D_DST_LINE_STRIDE (rw) register accessor: an alias for `Reg<D_DST_LINE_STRIDE_SPEC>`"]
pub type D_DST_LINE_STRIDE = crate::Reg<d_dst_line_stride::D_DST_LINE_STRIDE_SPEC>;
#[doc = "Line stride of output cube"]
pub mod d_dst_line_stride;
#[doc = "D_DST_SURFACE_STRIDE (rw) register accessor: an alias for `Reg<D_DST_SURFACE_STRIDE_SPEC>`"]
pub type D_DST_SURFACE_STRIDE = crate::Reg<d_dst_surface_stride::D_DST_SURFACE_STRIDE_SPEC>;
#[doc = "Surface stride of output cube"]
pub mod d_dst_surface_stride;
#[doc = "D_DST_RAM_CFG (rw) register accessor: an alias for `Reg<D_DST_RAM_CFG_SPEC>`"]
pub type D_DST_RAM_CFG = crate::Reg<d_dst_ram_cfg::D_DST_RAM_CFG_SPEC>;
#[doc = "RAM type of destination cube"]
pub mod d_dst_ram_cfg;
#[doc = "D_DATA_FORMAT (rw) register accessor: an alias for `Reg<D_DATA_FORMAT_SPEC>`"]
pub type D_DATA_FORMAT = crate::Reg<d_data_format::D_DATA_FORMAT_SPEC>;
#[doc = "Precision of input data"]
pub mod d_data_format;
#[doc = "D_INF_INPUT_NUM (r) register accessor: an alias for `Reg<D_INF_INPUT_NUM_SPEC>`"]
pub type D_INF_INPUT_NUM = crate::Reg<d_inf_input_num::D_INF_INPUT_NUM_SPEC>;
#[doc = "Input infinity element number"]
pub mod d_inf_input_num;
#[doc = "D_NAN_INPUT_NUM (r) register accessor: an alias for `Reg<D_NAN_INPUT_NUM_SPEC>`"]
pub type D_NAN_INPUT_NUM = crate::Reg<d_nan_input_num::D_NAN_INPUT_NUM_SPEC>;
#[doc = "Input NaN element number"]
pub mod d_nan_input_num;
#[doc = "D_NAN_OUTPUT_NUM (r) register accessor: an alias for `Reg<D_NAN_OUTPUT_NUM_SPEC>`"]
pub type D_NAN_OUTPUT_NUM = crate::Reg<d_nan_output_num::D_NAN_OUTPUT_NUM_SPEC>;
#[doc = "Output NaN element number"]
pub mod d_nan_output_num;
#[doc = "D_PERF_ENABLE (rw) register accessor: an alias for `Reg<D_PERF_ENABLE_SPEC>`"]
pub type D_PERF_ENABLE = crate::Reg<d_perf_enable::D_PERF_ENABLE_SPEC>;
#[doc = "Enable/disable performance counting"]
pub mod d_perf_enable;
#[doc = "D_PERF_WRITE_STALL (r) register accessor: an alias for `Reg<D_PERF_WRITE_STALL_SPEC>`"]
pub type D_PERF_WRITE_STALL = crate::Reg<d_perf_write_stall::D_PERF_WRITE_STALL_SPEC>;
#[doc = "Counting stalls of write requests"]
pub mod d_perf_write_stall;
#[doc = "D_CYA (rw) register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
