#[doc = "S_STATUS register accessor: an alias for `Reg<S_STATUS_SPEC>`"]
pub type S_STATUS = crate::Reg<s_status::S_STATUS_SPEC>;
#[doc = "Idle status of two register groups"]
pub mod s_status;
#[doc = "S_POINTER register accessor: an alias for `Reg<S_POINTER_SPEC>`"]
pub type S_POINTER = crate::Reg<s_pointer::S_POINTER_SPEC>;
#[doc = "Pointer for CSB master and data path to access groups"]
pub mod s_pointer;
#[doc = "S_ARBITER register accessor: an alias for `Reg<S_ARBITER_SPEC>`"]
pub type S_ARBITER = crate::Reg<s_arbiter::S_ARBITER_SPEC>;
#[doc = "WMB and Weight share same port to access external memory. This register controls the weight factor in the arbiter."]
pub mod s_arbiter;
#[doc = "S_CBUF_FLUSH_STATUS register accessor: an alias for `Reg<S_CBUF_FLUSH_STATUS_SPEC>`"]
pub type S_CBUF_FLUSH_STATUS = crate::Reg<s_cbuf_flush_status::S_CBUF_FLUSH_STATUS_SPEC>;
#[doc = "Indicates whether CBUF flush is finished after reset."]
pub mod s_cbuf_flush_status;
#[doc = "D_OP_ENABLE register accessor: an alias for `Reg<D_OP_ENABLE_SPEC>`"]
pub type D_OP_ENABLE = crate::Reg<d_op_enable::D_OP_ENABLE_SPEC>;
#[doc = "Set it to 1 to kick off operation for current register group"]
pub mod d_op_enable;
#[doc = "D_MISC_CFG register accessor: an alias for `Reg<D_MISC_CFG_SPEC>`"]
pub type D_MISC_CFG = crate::Reg<d_misc_cfg::D_MISC_CFG_SPEC>;
#[doc = "Configuration of operation: convolution mode, precision, weight reuse, data reuse."]
pub mod d_misc_cfg;
#[doc = "D_DATAIN_FORMAT register accessor: an alias for `Reg<D_DATAIN_FORMAT_SPEC>`"]
pub type D_DATAIN_FORMAT = crate::Reg<d_datain_format::D_DATAIN_FORMAT_SPEC>;
#[doc = "Input data format and pixel format"]
pub mod d_datain_format;
#[doc = "D_DATAIN_SIZE_0 register accessor: an alias for `Reg<D_DATAIN_SIZE_0_SPEC>`"]
pub type D_DATAIN_SIZE_0 = crate::Reg<d_datain_size_0::D_DATAIN_SIZE_0_SPEC>;
#[doc = "Input cube’s width and height"]
pub mod d_datain_size_0;
#[doc = "D_DATAIN_SIZE_1 register accessor: an alias for `Reg<D_DATAIN_SIZE_1_SPEC>`"]
pub type D_DATAIN_SIZE_1 = crate::Reg<d_datain_size_1::D_DATAIN_SIZE_1_SPEC>;
#[doc = "Input cube’s channel"]
pub mod d_datain_size_1;
#[doc = "D_DATAIN_SIZE_EXT_0 register accessor: an alias for `Reg<D_DATAIN_SIZE_EXT_0_SPEC>`"]
pub type D_DATAIN_SIZE_EXT_0 = crate::Reg<d_datain_size_ext_0::D_DATAIN_SIZE_EXT_0_SPEC>;
#[doc = "Input cube’s width and height after extension"]
pub mod d_datain_size_ext_0;
#[doc = "D_PIXEL_OFFSET register accessor: an alias for `Reg<D_PIXEL_OFFSET_SPEC>`"]
pub type D_PIXEL_OFFSET = crate::Reg<d_pixel_offset::D_PIXEL_OFFSET_SPEC>;
#[doc = "For image-in mode, horizontal offset and vertical offset of the 1 st pixel."]
pub mod d_pixel_offset;
#[doc = "D_DAIN_RAM_TYPE register accessor: an alias for `Reg<D_DAIN_RAM_TYPE_SPEC>`"]
pub type D_DAIN_RAM_TYPE = crate::Reg<d_dain_ram_type::D_DAIN_RAM_TYPE_SPEC>;
#[doc = "Ram type of input RAM"]
pub mod d_dain_ram_type;
#[doc = "D_DAIN_ADDR_HIGH_0 register accessor: an alias for `Reg<D_DAIN_ADDR_HIGH_0_SPEC>`"]
pub type D_DAIN_ADDR_HIGH_0 = crate::Reg<d_dain_addr_high_0::D_DAIN_ADDR_HIGH_0_SPEC>;
#[doc = "Higher 32bits of input data address when axi araddr is 64bits"]
pub mod d_dain_addr_high_0;
#[doc = "D_DAIN_ADDR_LOW_0 register accessor: an alias for `Reg<D_DAIN_ADDR_LOW_0_SPEC>`"]
pub type D_DAIN_ADDR_LOW_0 = crate::Reg<d_dain_addr_low_0::D_DAIN_ADDR_LOW_0_SPEC>;
#[doc = "Lower 32bits of input data address"]
pub mod d_dain_addr_low_0;
#[doc = "D_DAIN_ADDR_HIGH_1 register accessor: an alias for `Reg<D_DAIN_ADDR_HIGH_1_SPEC>`"]
pub type D_DAIN_ADDR_HIGH_1 = crate::Reg<d_dain_addr_high_1::D_DAIN_ADDR_HIGH_1_SPEC>;
#[doc = "Higher 32bits of input data address of UV plane when axi araddr is 64bits"]
pub mod d_dain_addr_high_1;
#[doc = "D_DAIN_ADDR_LOW_1 register accessor: an alias for `Reg<D_DAIN_ADDR_LOW_1_SPEC>`"]
pub type D_DAIN_ADDR_LOW_1 = crate::Reg<d_dain_addr_low_1::D_DAIN_ADDR_LOW_1_SPEC>;
#[doc = "Lower 32bits of input data address of UV plane"]
pub mod d_dain_addr_low_1;
#[doc = "D_LINE_STRIDE register accessor: an alias for `Reg<D_LINE_STRIDE_SPEC>`"]
pub type D_LINE_STRIDE = crate::Reg<d_line_stride::D_LINE_STRIDE_SPEC>;
#[doc = "Line stride of input cube"]
pub mod d_line_stride;
#[doc = "D_LINE_UV_STRIDE register accessor: an alias for `Reg<D_LINE_UV_STRIDE_SPEC>`"]
pub type D_LINE_UV_STRIDE = crate::Reg<d_line_uv_stride::D_LINE_UV_STRIDE_SPEC>;
#[doc = "Line stride of input cube’s UV plane"]
pub mod d_line_uv_stride;
#[doc = "D_SURF_STRIDE register accessor: an alias for `Reg<D_SURF_STRIDE_SPEC>`"]
pub type D_SURF_STRIDE = crate::Reg<d_surf_stride::D_SURF_STRIDE_SPEC>;
#[doc = "Surface stride of input cube"]
pub mod d_surf_stride;
#[doc = "D_DAIN_MAP register accessor: an alias for `Reg<D_DAIN_MAP_SPEC>`"]
pub type D_DAIN_MAP = crate::Reg<d_dain_map::D_DAIN_MAP_SPEC>;
#[doc = "Whether input cube is line packed or surface packed"]
pub mod d_dain_map;
#[doc = "D_BATCH_NUMBER register accessor: an alias for `Reg<D_BATCH_NUMBER_SPEC>`"]
pub type D_BATCH_NUMBER = crate::Reg<d_batch_number::D_BATCH_NUMBER_SPEC>;
#[doc = "Number of batches"]
pub mod d_batch_number;
#[doc = "D_BATCH_STRIDE register accessor: an alias for `Reg<D_BATCH_STRIDE_SPEC>`"]
pub type D_BATCH_STRIDE = crate::Reg<d_batch_stride::D_BATCH_STRIDE_SPEC>;
#[doc = "The stride of input data cubes when batches > 1"]
pub mod d_batch_stride;
#[doc = "D_ENTRY_PER_SLICE register accessor: an alias for `Reg<D_ENTRY_PER_SLICE_SPEC>`"]
pub type D_ENTRY_PER_SLICE = crate::Reg<d_entry_per_slice::D_ENTRY_PER_SLICE_SPEC>;
#[doc = "Number of CBUF entries used for one input slice"]
pub mod d_entry_per_slice;
#[doc = "D_FETCH_GRAIN register accessor: an alias for `Reg<D_FETCH_GRAIN_SPEC>`"]
pub type D_FETCH_GRAIN = crate::Reg<d_fetch_grain::D_FETCH_GRAIN_SPEC>;
#[doc = "Number of slices to be fetched before sending update information to CSC"]
pub mod d_fetch_grain;
#[doc = "D_WEIGHT_FORMAT register accessor: an alias for `Reg<D_WEIGHT_FORMAT_SPEC>`"]
pub type D_WEIGHT_FORMAT = crate::Reg<d_weight_format::D_WEIGHT_FORMAT_SPEC>;
#[doc = "Whether weight is compressed or not"]
pub mod d_weight_format;
#[doc = "D_WEIGHT_SIZE_0 register accessor: an alias for `Reg<D_WEIGHT_SIZE_0_SPEC>`"]
pub type D_WEIGHT_SIZE_0 = crate::Reg<d_weight_size_0::D_WEIGHT_SIZE_0_SPEC>;
#[doc = "The size of one kernel in bytes"]
pub mod d_weight_size_0;
#[doc = "D_WEIGHT_SIZE_1 register accessor: an alias for `Reg<D_WEIGHT_SIZE_1_SPEC>`"]
pub type D_WEIGHT_SIZE_1 = crate::Reg<d_weight_size_1::D_WEIGHT_SIZE_1_SPEC>;
#[doc = "Number of kernels"]
pub mod d_weight_size_1;
#[doc = "D_WEIGHT_RAM_TYPE register accessor: an alias for `Reg<D_WEIGHT_RAM_TYPE_SPEC>`"]
pub type D_WEIGHT_RAM_TYPE = crate::Reg<d_weight_ram_type::D_WEIGHT_RAM_TYPE_SPEC>;
#[doc = "Ram type of weight"]
pub mod d_weight_ram_type;
#[doc = "D_WEIGHT_ADDR_HIGH register accessor: an alias for `Reg<D_WEIGHT_ADDR_HIGH_SPEC>`"]
pub type D_WEIGHT_ADDR_HIGH = crate::Reg<d_weight_addr_high::D_WEIGHT_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of weight address when axi araddr is 64bits"]
pub mod d_weight_addr_high;
#[doc = "D_WEIGHT_ADDR_LOW register accessor: an alias for `Reg<D_WEIGHT_ADDR_LOW_SPEC>`"]
pub type D_WEIGHT_ADDR_LOW = crate::Reg<d_weight_addr_low::D_WEIGHT_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of weight address"]
pub mod d_weight_addr_low;
#[doc = "D_WEIGHT_BYTES register accessor: an alias for `Reg<D_WEIGHT_BYTES_SPEC>`"]
pub type D_WEIGHT_BYTES = crate::Reg<d_weight_bytes::D_WEIGHT_BYTES_SPEC>;
#[doc = "Total bytes of Weight"]
pub mod d_weight_bytes;
#[doc = "D_WGS_ADDR_HIGH register accessor: an alias for `Reg<D_WGS_ADDR_HIGH_SPEC>`"]
pub type D_WGS_ADDR_HIGH = crate::Reg<d_wgs_addr_high::D_WGS_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of wgs address when axi araddr is 64bits"]
pub mod d_wgs_addr_high;
#[doc = "D_WGS_ADDR_LOW register accessor: an alias for `Reg<D_WGS_ADDR_LOW_SPEC>`"]
pub type D_WGS_ADDR_LOW = crate::Reg<d_wgs_addr_low::D_WGS_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of wgs address"]
pub mod d_wgs_addr_low;
#[doc = "D_WMB_ADDR_HIGH register accessor: an alias for `Reg<D_WMB_ADDR_HIGH_SPEC>`"]
pub type D_WMB_ADDR_HIGH = crate::Reg<d_wmb_addr_high::D_WMB_ADDR_HIGH_SPEC>;
#[doc = "Higher 32bits of wmb address when axi araddr is 64bits"]
pub mod d_wmb_addr_high;
#[doc = "D_WMB_ADDR_LOW register accessor: an alias for `Reg<D_WMB_ADDR_LOW_SPEC>`"]
pub type D_WMB_ADDR_LOW = crate::Reg<d_wmb_addr_low::D_WMB_ADDR_LOW_SPEC>;
#[doc = "Lower 32bits of wmb address"]
pub mod d_wmb_addr_low;
#[doc = "D_WMB_BYTES register accessor: an alias for `Reg<D_WMB_BYTES_SPEC>`"]
pub type D_WMB_BYTES = crate::Reg<d_wmb_bytes::D_WMB_BYTES_SPEC>;
#[doc = "Total bytes of WMB"]
pub mod d_wmb_bytes;
#[doc = "D_MEAN_FORMAT register accessor: an alias for `Reg<D_MEAN_FORMAT_SPEC>`"]
pub type D_MEAN_FORMAT = crate::Reg<d_mean_format::D_MEAN_FORMAT_SPEC>;
#[doc = "Whether mean registers are used or not"]
pub mod d_mean_format;
#[doc = "D_MEAN_GLOBAL_0 register accessor: an alias for `Reg<D_MEAN_GLOBAL_0_SPEC>`"]
pub type D_MEAN_GLOBAL_0 = crate::Reg<d_mean_global_0::D_MEAN_GLOBAL_0_SPEC>;
#[doc = "Global mean value for red in RGB or Y in YUV Global mean value for green in RGB or U in YUV"]
pub mod d_mean_global_0;
#[doc = "D_MEAN_GLOBAL_1 register accessor: an alias for `Reg<D_MEAN_GLOBAL_1_SPEC>`"]
pub type D_MEAN_GLOBAL_1 = crate::Reg<d_mean_global_1::D_MEAN_GLOBAL_1_SPEC>;
#[doc = "Global mean value for blue in RGB or V in YUV Global mean value for alpha in ARGB/AYUV or X in XRGB"]
pub mod d_mean_global_1;
#[doc = "D_CVT_CFG register accessor: an alias for `Reg<D_CVT_CFG_SPEC>`"]
pub type D_CVT_CFG = crate::Reg<d_cvt_cfg::D_CVT_CFG_SPEC>;
#[doc = "Enable/disable input data converter in CDMA and number of bits to be truncated in the input data converter"]
pub mod d_cvt_cfg;
#[doc = "D_CVT_OFFSET register accessor: an alias for `Reg<D_CVT_OFFSET_SPEC>`"]
pub type D_CVT_OFFSET = crate::Reg<d_cvt_offset::D_CVT_OFFSET_SPEC>;
#[doc = "Offset of input data convertor"]
pub mod d_cvt_offset;
#[doc = "D_CVT_SCALE register accessor: an alias for `Reg<D_CVT_SCALE_SPEC>`"]
pub type D_CVT_SCALE = crate::Reg<d_cvt_scale::D_CVT_SCALE_SPEC>;
#[doc = "Scale of input data convertor"]
pub mod d_cvt_scale;
#[doc = "D_CONV_STRIDE register accessor: an alias for `Reg<D_CONV_STRIDE_SPEC>`"]
pub type D_CONV_STRIDE = crate::Reg<d_conv_stride::D_CONV_STRIDE_SPEC>;
#[doc = "Convolution x stride and convolution y stride"]
pub mod d_conv_stride;
#[doc = "D_ZERO_PADDING register accessor: an alias for `Reg<D_ZERO_PADDING_SPEC>`"]
pub type D_ZERO_PADDING = crate::Reg<d_zero_padding::D_ZERO_PADDING_SPEC>;
#[doc = "Left/right/top/bottom padding size"]
pub mod d_zero_padding;
#[doc = "D_ZERO_PADDING_VALUE register accessor: an alias for `Reg<D_ZERO_PADDING_VALUE_SPEC>`"]
pub type D_ZERO_PADDING_VALUE = crate::Reg<d_zero_padding_value::D_ZERO_PADDING_VALUE_SPEC>;
#[doc = "Padding value"]
pub mod d_zero_padding_value;
#[doc = "D_BANK register accessor: an alias for `Reg<D_BANK_SPEC>`"]
pub type D_BANK = crate::Reg<d_bank::D_BANK_SPEC>;
#[doc = "Number of data banks and weight banks in CBUF"]
pub mod d_bank;
#[doc = "D_NAN_FLUSH_TO_ZERO register accessor: an alias for `Reg<D_NAN_FLUSH_TO_ZERO_SPEC>`"]
pub type D_NAN_FLUSH_TO_ZERO = crate::Reg<d_nan_flush_to_zero::D_NAN_FLUSH_TO_ZERO_SPEC>;
#[doc = "Enable/Disable flushing input NaN to zero"]
pub mod d_nan_flush_to_zero;
#[doc = "D_NAN_INPUT_DATA_NUM register accessor: an alias for `Reg<D_NAN_INPUT_DATA_NUM_SPEC>`"]
pub type D_NAN_INPUT_DATA_NUM = crate::Reg<d_nan_input_data_num::D_NAN_INPUT_DATA_NUM_SPEC>;
#[doc = "Count NaN number in input data cube, update per layer"]
pub mod d_nan_input_data_num;
#[doc = "D_NAN_INPUT_WEIGHT_NUM register accessor: an alias for `Reg<D_NAN_INPUT_WEIGHT_NUM_SPEC>`"]
pub type D_NAN_INPUT_WEIGHT_NUM = crate::Reg<d_nan_input_weight_num::D_NAN_INPUT_WEIGHT_NUM_SPEC>;
#[doc = "Count NaN number in weight kernels, update per layer"]
pub mod d_nan_input_weight_num;
#[doc = "D_INF_INPUT_DATA_NUM register accessor: an alias for `Reg<D_INF_INPUT_DATA_NUM_SPEC>`"]
pub type D_INF_INPUT_DATA_NUM = crate::Reg<d_inf_input_data_num::D_INF_INPUT_DATA_NUM_SPEC>;
#[doc = "Count infinity number in input data cube, update per layer"]
pub mod d_inf_input_data_num;
#[doc = "D_INF_INPUT_WEIGHT_NUM register accessor: an alias for `Reg<D_INF_INPUT_WEIGHT_NUM_SPEC>`"]
pub type D_INF_INPUT_WEIGHT_NUM = crate::Reg<d_inf_input_weight_num::D_INF_INPUT_WEIGHT_NUM_SPEC>;
#[doc = "Count infinity number in weight kernels, update per layer"]
pub mod d_inf_input_weight_num;
#[doc = "D_PERF_ENABLE register accessor: an alias for `Reg<D_PERF_ENABLE_SPEC>`"]
pub type D_PERF_ENABLE = crate::Reg<d_perf_enable::D_PERF_ENABLE_SPEC>;
#[doc = "Enable/disable performance counter"]
pub mod d_perf_enable;
#[doc = "D_PERF_DAT_READ_STALL register accessor: an alias for `Reg<D_PERF_DAT_READ_STALL_SPEC>`"]
pub type D_PERF_DAT_READ_STALL = crate::Reg<d_perf_dat_read_stall::D_PERF_DAT_READ_STALL_SPEC>;
#[doc = "Count blocking cycles of read request of input data, update per layer"]
pub mod d_perf_dat_read_stall;
#[doc = "D_PERF_WT_READ_STALL register accessor: an alias for `Reg<D_PERF_WT_READ_STALL_SPEC>`"]
pub type D_PERF_WT_READ_STALL = crate::Reg<d_perf_wt_read_stall::D_PERF_WT_READ_STALL_SPEC>;
#[doc = "Count blocking cycles of read request of weight data, update per layer"]
pub mod d_perf_wt_read_stall;
#[doc = "D_PERF_DAT_READ_LATENCY register accessor: an alias for `Reg<D_PERF_DAT_READ_LATENCY_SPEC>`"]
pub type D_PERF_DAT_READ_LATENCY =
    crate::Reg<d_perf_dat_read_latency::D_PERF_DAT_READ_LATENCY_SPEC>;
#[doc = "Count total latency cycles of read response of input data, update per layer"]
pub mod d_perf_dat_read_latency;
#[doc = "D_PERF_WT_READ_LATENCY register accessor: an alias for `Reg<D_PERF_WT_READ_LATENCY_SPEC>`"]
pub type D_PERF_WT_READ_LATENCY = crate::Reg<d_perf_wt_read_latency::D_PERF_WT_READ_LATENCY_SPEC>;
#[doc = "Count total latency cycles of read request of weight data, update per layer"]
pub mod d_perf_wt_read_latency;
#[doc = "D_CYA register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
