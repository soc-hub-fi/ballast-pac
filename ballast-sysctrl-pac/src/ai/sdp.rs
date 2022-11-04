#[doc = "S_STATUS register accessor: an alias for `Reg<S_STATUS_SPEC>`"]
pub type S_STATUS = crate::Reg<s_status::S_STATUS_SPEC>;
#[doc = "Idle status of two register groups"]
pub mod s_status;
#[doc = "S_POINTER register accessor: an alias for `Reg<S_POINTER_SPEC>`"]
pub type S_POINTER = crate::Reg<s_pointer::S_POINTER_SPEC>;
#[doc = "Pointer for CSB master and data path to access groups"]
pub mod s_pointer;
#[doc = "S_LUT_ACCESS_CFG register accessor: an alias for `Reg<S_LUT_ACCESS_CFG_SPEC>`"]
pub type S_LUT_ACCESS_CFG = crate::Reg<s_lut_access_cfg::S_LUT_ACCESS_CFG_SPEC>;
#[doc = "LUT access address and type"]
pub mod s_lut_access_cfg;
#[doc = "S_LUT_ACCESS_DATA register accessor: an alias for `Reg<S_LUT_ACCESS_DATA_SPEC>`"]
pub type S_LUT_ACCESS_DATA = crate::Reg<s_lut_access_data::S_LUT_ACCESS_DATA_SPEC>;
#[doc = "Data register of read or write LUT"]
pub mod s_lut_access_data;
#[doc = "S_LUT_CFG register accessor: an alias for `Reg<S_LUT_CFG_SPEC>`"]
pub type S_LUT_CFG = crate::Reg<s_lut_cfg::S_LUT_CFG_SPEC>;
#[doc = "LUTs type: exponent or linear. And the selection between LE and LO tables."]
pub mod s_lut_cfg;
#[doc = "S_LUT_INFO register accessor: an alias for `Reg<S_LUT_INFO_SPEC>`"]
pub type S_LUT_INFO = crate::Reg<s_lut_info::S_LUT_INFO_SPEC>;
#[doc = "LE and LO LUT index offset and selection"]
pub mod s_lut_info;
#[doc = "S_LUT_LE_START register accessor: an alias for `Reg<S_LUT_LE_START_SPEC>`"]
pub type S_LUT_LE_START = crate::Reg<s_lut_le_start::S_LUT_LE_START_SPEC>;
#[doc = "Start of LE LUTs range"]
pub mod s_lut_le_start;
#[doc = "S_LUT_LE_END register accessor: an alias for `Reg<S_LUT_LE_END_SPEC>`"]
pub type S_LUT_LE_END = crate::Reg<s_lut_le_end::S_LUT_LE_END_SPEC>;
#[doc = "End of LE LUTs range"]
pub mod s_lut_le_end;
#[doc = "S_LUT_LO_START register accessor: an alias for `Reg<S_LUT_LO_START_SPEC>`"]
pub type S_LUT_LO_START = crate::Reg<s_lut_lo_start::S_LUT_LO_START_SPEC>;
#[doc = "Start of LO LUTs range"]
pub mod s_lut_lo_start;
#[doc = "S_LUT_LO_END register accessor: an alias for `Reg<S_LUT_LO_END_SPEC>`"]
pub type S_LUT_LO_END = crate::Reg<s_lut_lo_end::S_LUT_LO_END_SPEC>;
#[doc = "End of LO LUTs range"]
pub mod s_lut_lo_end;
#[doc = "S_LUT_LE_SLOPE_SCALE register accessor: an alias for `Reg<S_LUT_LE_SLOPE_SCALE_SPEC>`"]
pub type S_LUT_LE_SLOPE_SCALE = crate::Reg<s_lut_le_slope_scale::S_LUT_LE_SLOPE_SCALE_SPEC>;
#[doc = "Slope scale parameter for LE LUT underflow and overflow, signed value"]
pub mod s_lut_le_slope_scale;
#[doc = "S_LUT_LE_SLOPE_SHIFT register accessor: an alias for `Reg<S_LUT_LE_SLOPE_SHIFT_SPEC>`"]
pub type S_LUT_LE_SLOPE_SHIFT = crate::Reg<s_lut_le_slope_shift::S_LUT_LE_SLOPE_SHIFT_SPEC>;
#[doc = "Slope shift parameter for LE_LUT underflow and overflow, signed value"]
pub mod s_lut_le_slope_shift;
#[doc = "S_LUT_LO_SLOPE_SCALE register accessor: an alias for `Reg<S_LUT_LO_SLOPE_SCALE_SPEC>`"]
pub type S_LUT_LO_SLOPE_SCALE = crate::Reg<s_lut_lo_slope_scale::S_LUT_LO_SLOPE_SCALE_SPEC>;
#[doc = "Slope scale parameter for LO LUT underflow and overflow, signed value"]
pub mod s_lut_lo_slope_scale;
#[doc = "S_LUT_LO_SLOPE_SHIFT register accessor: an alias for `Reg<S_LUT_LO_SLOPE_SHIFT_SPEC>`"]
pub type S_LUT_LO_SLOPE_SHIFT = crate::Reg<s_lut_lo_slope_shift::S_LUT_LO_SLOPE_SHIFT_SPEC>;
#[doc = "Slope shift parameter for LO_LUT underflow and overflow, signed value"]
pub mod s_lut_lo_slope_shift;
#[doc = "D_OP_ENABLE register accessor: an alias for `Reg<D_OP_ENABLE_SPEC>`"]
pub type D_OP_ENABLE = crate::Reg<d_op_enable::D_OP_ENABLE_SPEC>;
#[doc = "Set it to 1 to kick off operation for current register group"]
pub mod d_op_enable;
#[doc = "D_DATA_CUBE_WIDTH register accessor: an alias for `Reg<D_DATA_CUBE_WIDTH_SPEC>`"]
pub type D_DATA_CUBE_WIDTH = crate::Reg<d_data_cube_width::D_DATA_CUBE_WIDTH_SPEC>;
#[doc = "Input cubes width"]
pub mod d_data_cube_width;
#[doc = "D_DATA_CUBE_HEIGHT register accessor: an alias for `Reg<D_DATA_CUBE_HEIGHT_SPEC>`"]
pub type D_DATA_CUBE_HEIGHT = crate::Reg<d_data_cube_height::D_DATA_CUBE_HEIGHT_SPEC>;
#[doc = "Input cubes height"]
pub mod d_data_cube_height;
#[doc = "D_DATA_CUBE_CHANNEL register accessor: an alias for `Reg<D_DATA_CUBE_CHANNEL_SPEC>`"]
pub type D_DATA_CUBE_CHANNEL = crate::Reg<d_data_cube_channel::D_DATA_CUBE_CHANNEL_SPEC>;
#[doc = "Input cubes channel"]
pub mod d_data_cube_channel;
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
#[doc = "Line stride of output data cube"]
pub mod d_dst_line_stride;
#[doc = "D_DST_SURFACE_STRIDE register accessor: an alias for `Reg<D_DST_SURFACE_STRIDE_SPEC>`"]
pub type D_DST_SURFACE_STRIDE = crate::Reg<d_dst_surface_stride::D_DST_SURFACE_STRIDE_SPEC>;
#[doc = "Surface stride of output data cube"]
pub mod d_dst_surface_stride;
#[doc = "D_DP_BS_CFG register accessor: an alias for `Reg<D_DP_BS_CFG_SPEC>`"]
pub type D_DP_BS_CFG = crate::Reg<d_dp_bs_cfg::D_DP_BS_CFG_SPEC>;
#[doc = "Configurations of BS module: bypass, algorithm, etc."]
pub mod d_dp_bs_cfg;
#[doc = "D_DP_BS_ALU_CFG register accessor: an alias for `Reg<D_DP_BS_ALU_CFG_SPEC>`"]
pub type D_DP_BS_ALU_CFG = crate::Reg<d_dp_bs_alu_cfg::D_DP_BS_ALU_CFG_SPEC>;
#[doc = "Source type and shifter value of BS ALU"]
pub mod d_dp_bs_alu_cfg;
#[doc = "D_DP_BS_ALU_SRC_VALUE register accessor: an alias for `Reg<D_DP_BS_ALU_SRC_VALUE_SPEC>`"]
pub type D_DP_BS_ALU_SRC_VALUE = crate::Reg<d_dp_bs_alu_src_value::D_DP_BS_ALU_SRC_VALUE_SPEC>;
#[doc = "Operand value of BS ALU"]
pub mod d_dp_bs_alu_src_value;
#[doc = "D_DP_BS_MUL_CFG register accessor: an alias for `Reg<D_DP_BS_MUL_CFG_SPEC>`"]
pub type D_DP_BS_MUL_CFG = crate::Reg<d_dp_bs_mul_cfg::D_DP_BS_MUL_CFG_SPEC>;
#[doc = "Source type and shifter value of BS MUL"]
pub mod d_dp_bs_mul_cfg;
#[doc = "D_DP_BS_MUL_SRC_VALUE register accessor: an alias for `Reg<D_DP_BS_MUL_SRC_VALUE_SPEC>`"]
pub type D_DP_BS_MUL_SRC_VALUE = crate::Reg<d_dp_bs_mul_src_value::D_DP_BS_MUL_SRC_VALUE_SPEC>;
#[doc = "Operand value of BS MUL"]
pub mod d_dp_bs_mul_src_value;
#[doc = "D_DP_BN_CFG register accessor: an alias for `Reg<D_DP_BN_CFG_SPEC>`"]
pub type D_DP_BN_CFG = crate::Reg<d_dp_bn_cfg::D_DP_BN_CFG_SPEC>;
#[doc = "Configurations of BN module: bypass, algorithm, etc."]
pub mod d_dp_bn_cfg;
#[doc = "D_DP_BN_ALU_CFG register accessor: an alias for `Reg<D_DP_BN_ALU_CFG_SPEC>`"]
pub type D_DP_BN_ALU_CFG = crate::Reg<d_dp_bn_alu_cfg::D_DP_BN_ALU_CFG_SPEC>;
#[doc = "Source type and shifter value of BN ALU"]
pub mod d_dp_bn_alu_cfg;
#[doc = "D_DP_BN_ALU_SRC_VALUE register accessor: an alias for `Reg<D_DP_BN_ALU_SRC_VALUE_SPEC>`"]
pub type D_DP_BN_ALU_SRC_VALUE = crate::Reg<d_dp_bn_alu_src_value::D_DP_BN_ALU_SRC_VALUE_SPEC>;
#[doc = "Operand value of BN ALU"]
pub mod d_dp_bn_alu_src_value;
#[doc = "D_DP_BN_MUL_CFG register accessor: an alias for `Reg<D_DP_BN_MUL_CFG_SPEC>`"]
pub type D_DP_BN_MUL_CFG = crate::Reg<d_dp_bn_mul_cfg::D_DP_BN_MUL_CFG_SPEC>;
#[doc = "Source type and shifter value of BN MUL"]
pub mod d_dp_bn_mul_cfg;
#[doc = "D_DP_BN_MUL_SRC_VALUE register accessor: an alias for `Reg<D_DP_BN_MUL_SRC_VALUE_SPEC>`"]
pub type D_DP_BN_MUL_SRC_VALUE = crate::Reg<d_dp_bn_mul_src_value::D_DP_BN_MUL_SRC_VALUE_SPEC>;
#[doc = "Operand value of BN MUL"]
pub mod d_dp_bn_mul_src_value;
#[doc = "D_DP_EW_CFG register accessor: an alias for `Reg<D_DP_EW_CFG_SPEC>`"]
pub type D_DP_EW_CFG = crate::Reg<d_dp_ew_cfg::D_DP_EW_CFG_SPEC>;
#[doc = "Configurations of EW module: bypass, algorithm, etc."]
pub mod d_dp_ew_cfg;
#[doc = "D_DP_EW_ALU_CFG register accessor: an alias for `Reg<D_DP_EW_ALU_CFG_SPEC>`"]
pub type D_DP_EW_ALU_CFG = crate::Reg<d_dp_ew_alu_cfg::D_DP_EW_ALU_CFG_SPEC>;
#[doc = "Source type and bypass control of EW ALU"]
pub mod d_dp_ew_alu_cfg;
#[doc = "D_DP_EW_ALU_SRC_VALUE register accessor: an alias for `Reg<D_DP_EW_ALU_SRC_VALUE_SPEC>`"]
pub type D_DP_EW_ALU_SRC_VALUE = crate::Reg<d_dp_ew_alu_src_value::D_DP_EW_ALU_SRC_VALUE_SPEC>;
#[doc = "Operand value of EW ALU"]
pub mod d_dp_ew_alu_src_value;
#[doc = "D_DP_EW_ALU_CVT_OFFSET_VALUE register accessor: an alias for `Reg<D_DP_EW_ALU_CVT_OFFSET_VALUE_SPEC>`"]
pub type D_DP_EW_ALU_CVT_OFFSET_VALUE =
    crate::Reg<d_dp_ew_alu_cvt_offset_value::D_DP_EW_ALU_CVT_OFFSET_VALUE_SPEC>;
#[doc = "Converter offset of EW ALU"]
pub mod d_dp_ew_alu_cvt_offset_value;
#[doc = "D_DP_EW_ALU_CVT_SCALE_VALUE register accessor: an alias for `Reg<D_DP_EW_ALU_CVT_SCALE_VALUE_SPEC>`"]
pub type D_DP_EW_ALU_CVT_SCALE_VALUE =
    crate::Reg<d_dp_ew_alu_cvt_scale_value::D_DP_EW_ALU_CVT_SCALE_VALUE_SPEC>;
#[doc = "Converter scale of EW ALU"]
pub mod d_dp_ew_alu_cvt_scale_value;
#[doc = "D_DP_EW_ALU_CVT_TRUNCATE_VALUE register accessor: an alias for `Reg<D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>`"]
pub type D_DP_EW_ALU_CVT_TRUNCATE_VALUE =
    crate::Reg<d_dp_ew_alu_cvt_truncate_value::D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>;
#[doc = "Converter truncate of EW ALU"]
pub mod d_dp_ew_alu_cvt_truncate_value;
#[doc = "D_DP_EW_MUL_CFG register accessor: an alias for `Reg<D_DP_EW_MUL_CFG_SPEC>`"]
pub type D_DP_EW_MUL_CFG = crate::Reg<d_dp_ew_mul_cfg::D_DP_EW_MUL_CFG_SPEC>;
#[doc = "Source type and bypass control of EW MUL"]
pub mod d_dp_ew_mul_cfg;
#[doc = "D_DP_EW_MUL_SRC_VALUE register accessor: an alias for `Reg<D_DP_EW_MUL_SRC_VALUE_SPEC>`"]
pub type D_DP_EW_MUL_SRC_VALUE = crate::Reg<d_dp_ew_mul_src_value::D_DP_EW_MUL_SRC_VALUE_SPEC>;
#[doc = "Operand value of EW MUL"]
pub mod d_dp_ew_mul_src_value;
#[doc = "D_DP_EW_MUL_CVT_OFFSET_VALUE register accessor: an alias for `Reg<D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>`"]
pub type D_DP_EW_MUL_CVT_OFFSET_VALUE =
    crate::Reg<d_dp_ew_mul_cvt_offset_value::D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>;
#[doc = "Converter offset of EW MUL"]
pub mod d_dp_ew_mul_cvt_offset_value;
#[doc = "D_DP_EW_MUL_CVT_SCALE_VALUE register accessor: an alias for `Reg<D_DP_EW_MUL_CVT_SCALE_VALUE_SPEC>`"]
pub type D_DP_EW_MUL_CVT_SCALE_VALUE =
    crate::Reg<d_dp_ew_mul_cvt_scale_value::D_DP_EW_MUL_CVT_SCALE_VALUE_SPEC>;
#[doc = "Converter scale of EW MUL"]
pub mod d_dp_ew_mul_cvt_scale_value;
#[doc = "D_DP_EW_MUL_CVT_TRUNCATE_VALUE register accessor: an alias for `Reg<D_DP_EW_MUL_CVT_TRUNCATE_VALUE_SPEC>`"]
pub type D_DP_EW_MUL_CVT_TRUNCATE_VALUE =
    crate::Reg<d_dp_ew_mul_cvt_truncate_value::D_DP_EW_MUL_CVT_TRUNCATE_VALUE_SPEC>;
#[doc = "Converter truncate of EW MUL"]
pub mod d_dp_ew_mul_cvt_truncate_value;
#[doc = "D_DP_EW_TRUNCATE_VALUE register accessor: an alias for `Reg<D_DP_EW_TRUNCATE_VALUE_SPEC>`"]
pub type D_DP_EW_TRUNCATE_VALUE = crate::Reg<d_dp_ew_truncate_value::D_DP_EW_TRUNCATE_VALUE_SPEC>;
#[doc = "Truncate of EW"]
pub mod d_dp_ew_truncate_value;
#[doc = "D_FEATURE_MODE_CFG register accessor: an alias for `Reg<D_FEATURE_MODE_CFG_SPEC>`"]
pub type D_FEATURE_MODE_CFG = crate::Reg<d_feature_mode_cfg::D_FEATURE_MODE_CFG_SPEC>;
#[doc = "Operation configuration: flying mode, output destination, Direct or Winograd mode, flush NaN to zero, batch number."]
pub mod d_feature_mode_cfg;
#[doc = "D_DST_DMA_CFG register accessor: an alias for `Reg<D_DST_DMA_CFG_SPEC>`"]
pub type D_DST_DMA_CFG = crate::Reg<d_dst_dma_cfg::D_DST_DMA_CFG_SPEC>;
#[doc = "Destination RAM type"]
pub mod d_dst_dma_cfg;
#[doc = "D_DST_BATCH_STRIDE register accessor: an alias for `Reg<D_DST_BATCH_STRIDE_SPEC>`"]
pub type D_DST_BATCH_STRIDE = crate::Reg<d_dst_batch_stride::D_DST_BATCH_STRIDE_SPEC>;
#[doc = "Stride of output cubes in batch mode"]
pub mod d_dst_batch_stride;
#[doc = "D_DATA_FORMAT register accessor: an alias for `Reg<D_DATA_FORMAT_SPEC>`"]
pub type D_DATA_FORMAT = crate::Reg<d_data_format::D_DATA_FORMAT_SPEC>;
#[doc = "Data precision"]
pub mod d_data_format;
#[doc = "D_CVT_OFFSET register accessor: an alias for `Reg<D_CVT_OFFSET_SPEC>`"]
pub type D_CVT_OFFSET = crate::Reg<d_cvt_offset::D_CVT_OFFSET_SPEC>;
#[doc = "Output converter offset"]
pub mod d_cvt_offset;
#[doc = "D_CVT_SCALE register accessor: an alias for `Reg<D_CVT_SCALE_SPEC>`"]
pub type D_CVT_SCALE = crate::Reg<d_cvt_scale::D_CVT_SCALE_SPEC>;
#[doc = "Output converter scale"]
pub mod d_cvt_scale;
#[doc = "D_CVT_SHIFT register accessor: an alias for `Reg<D_CVT_SHIFT_SPEC>`"]
pub type D_CVT_SHIFT = crate::Reg<d_cvt_shift::D_CVT_SHIFT_SPEC>;
#[doc = "Output converter shifter value"]
pub mod d_cvt_shift;
#[doc = "D_STATUS register accessor: an alias for `Reg<D_STATUS_SPEC>`"]
pub type D_STATUS = crate::Reg<d_status::D_STATUS_SPEC>;
#[doc = "Output of equal mode"]
pub mod d_status;
#[doc = "D_STATUS_NAN_INPUT_NUM register accessor: an alias for `Reg<D_STATUS_NAN_INPUT_NUM_SPEC>`"]
pub type D_STATUS_NAN_INPUT_NUM = crate::Reg<d_status_nan_input_num::D_STATUS_NAN_INPUT_NUM_SPEC>;
#[doc = "Input NaN element number"]
pub mod d_status_nan_input_num;
#[doc = "D_STATUS_INF_INPUT_NUM register accessor: an alias for `Reg<D_STATUS_INF_INPUT_NUM_SPEC>`"]
pub type D_STATUS_INF_INPUT_NUM = crate::Reg<d_status_inf_input_num::D_STATUS_INF_INPUT_NUM_SPEC>;
#[doc = "Input Infinity element number"]
pub mod d_status_inf_input_num;
#[doc = "D_STATUS_NAN_OUTPUT_NUM register accessor: an alias for `Reg<D_STATUS_NAN_OUTPUT_NUM_SPEC>`"]
pub type D_STATUS_NAN_OUTPUT_NUM =
    crate::Reg<d_status_nan_output_num::D_STATUS_NAN_OUTPUT_NUM_SPEC>;
#[doc = "Output NaN element number"]
pub mod d_status_nan_output_num;
#[doc = "D_PERF_ENABLE register accessor: an alias for `Reg<D_PERF_ENABLE_SPEC>`"]
pub type D_PERF_ENABLE = crate::Reg<d_perf_enable::D_PERF_ENABLE_SPEC>;
#[doc = "Enable/Disable performance counting"]
pub mod d_perf_enable;
#[doc = "D_PERF_WDMA_WRITE_STALL register accessor: an alias for `Reg<D_PERF_WDMA_WRITE_STALL_SPEC>`"]
pub type D_PERF_WDMA_WRITE_STALL =
    crate::Reg<d_perf_wdma_write_stall::D_PERF_WDMA_WRITE_STALL_SPEC>;
#[doc = "Count stall cycles of write DMA for one layer"]
pub mod d_perf_wdma_write_stall;
#[doc = "D_PERF_LUT_UFLOW register accessor: an alias for `Reg<D_PERF_LUT_UFLOW_SPEC>`"]
pub type D_PERF_LUT_UFLOW = crate::Reg<d_perf_lut_uflow::D_PERF_LUT_UFLOW_SPEC>;
#[doc = "Element number of both table underflow"]
pub mod d_perf_lut_uflow;
#[doc = "D_PERF_LUT_OFLOW register accessor: an alias for `Reg<D_PERF_LUT_OFLOW_SPEC>`"]
pub type D_PERF_LUT_OFLOW = crate::Reg<d_perf_lut_oflow::D_PERF_LUT_OFLOW_SPEC>;
#[doc = "Element number of both table overflow"]
pub mod d_perf_lut_oflow;
#[doc = "D_PERF_OUT_SATURATION register accessor: an alias for `Reg<D_PERF_OUT_SATURATION_SPEC>`"]
pub type D_PERF_OUT_SATURATION = crate::Reg<d_perf_out_saturation::D_PERF_OUT_SATURATION_SPEC>;
#[doc = "Element number of both table saturation"]
pub mod d_perf_out_saturation;
#[doc = "D_PERF_LUT_HYBRID register accessor: an alias for `Reg<D_PERF_LUT_HYBRID_SPEC>`"]
pub type D_PERF_LUT_HYBRID = crate::Reg<d_perf_lut_hybrid::D_PERF_LUT_HYBRID_SPEC>;
#[doc = "Element number of both hit, or both miss situation that element underflow one table and at the same time overflow the other."]
pub mod d_perf_lut_hybrid;
#[doc = "D_PERF_LUT_LE_HIT register accessor: an alias for `Reg<D_PERF_LUT_LE_HIT_SPEC>`"]
pub type D_PERF_LUT_LE_HIT = crate::Reg<d_perf_lut_le_hit::D_PERF_LUT_LE_HIT_SPEC>;
#[doc = "Element number of only LE table hit"]
pub mod d_perf_lut_le_hit;
#[doc = "D_PERF_LUT_LO_HIT register accessor: an alias for `Reg<D_PERF_LUT_LO_HIT_SPEC>`"]
pub type D_PERF_LUT_LO_HIT = crate::Reg<d_perf_lut_lo_hit::D_PERF_LUT_LO_HIT_SPEC>;
#[doc = "Element number of only LO table hit"]
pub mod d_perf_lut_lo_hit;
