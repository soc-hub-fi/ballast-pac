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
#[doc = "D_MISC_CFG register accessor: an alias for `Reg<D_MISC_CFG_SPEC>`"]
pub type D_MISC_CFG = crate::Reg<d_misc_cfg::D_MISC_CFG_SPEC>;
#[doc = "Configuration of operation: convolution mode, precision, weight reuse, data reuse."]
pub mod d_misc_cfg;
#[doc = "D_DATAIN_FORMAT register accessor: an alias for `Reg<D_DATAIN_FORMAT_SPEC>`"]
pub type D_DATAIN_FORMAT = crate::Reg<d_datain_format::D_DATAIN_FORMAT_SPEC>;
#[doc = "Input data format and pixel format"]
pub mod d_datain_format;
#[doc = "D_DATAIN_SIZE_EXT_0 register accessor: an alias for `Reg<D_DATAIN_SIZE_EXT_0_SPEC>`"]
pub type D_DATAIN_SIZE_EXT_0 = crate::Reg<d_datain_size_ext_0::D_DATAIN_SIZE_EXT_0_SPEC>;
#[doc = "Input cubes width and height after extension"]
pub mod d_datain_size_ext_0;
#[doc = "D_DATAIN_SIZE_EXT_1 register accessor: an alias for `Reg<D_DATAIN_SIZE_EXT_1_SPEC>`"]
pub type D_DATAIN_SIZE_EXT_1 = crate::Reg<d_datain_size_ext_1::D_DATAIN_SIZE_EXT_1_SPEC>;
#[doc = "Input cubes channel after extension"]
pub mod d_datain_size_ext_1;
#[doc = "D_BATCH_NUMBER register accessor: an alias for `Reg<D_BATCH_NUMBER_SPEC>`"]
pub type D_BATCH_NUMBER = crate::Reg<d_batch_number::D_BATCH_NUMBER_SPEC>;
#[doc = "Number of batches"]
pub mod d_batch_number;
#[doc = "D_POST_Y_EXTENSION register accessor: an alias for `Reg<D_POST_Y_EXTENSION_SPEC>`"]
pub type D_POST_Y_EXTENSION = crate::Reg<d_post_y_extension::D_POST_Y_EXTENSION_SPEC>;
#[doc = "Post extension parameter for image-in"]
pub mod d_post_y_extension;
#[doc = "D_ENTRY_PER_SLICE register accessor: an alias for `Reg<D_ENTRY_PER_SLICE_SPEC>`"]
pub type D_ENTRY_PER_SLICE = crate::Reg<d_entry_per_slice::D_ENTRY_PER_SLICE_SPEC>;
#[doc = "Number of CBUF entries used for one input slice"]
pub mod d_entry_per_slice;
#[doc = "D_WEIGHT_FORMAT register accessor: an alias for `Reg<D_WEIGHT_FORMAT_SPEC>`"]
pub type D_WEIGHT_FORMAT = crate::Reg<d_weight_format::D_WEIGHT_FORMAT_SPEC>;
#[doc = "Whether weight is compressed or not"]
pub mod d_weight_format;
#[doc = "D_WEIGHT_SIZE_EXT_0 register accessor: an alias for `Reg<D_WEIGHT_SIZE_EXT_0_SPEC>`"]
pub type D_WEIGHT_SIZE_EXT_0 = crate::Reg<d_weight_size_ext_0::D_WEIGHT_SIZE_EXT_0_SPEC>;
#[doc = "Weights width and height after extension"]
pub mod d_weight_size_ext_0;
#[doc = "D_WEIGHT_SIZE_EXT_1 register accessor: an alias for `Reg<D_WEIGHT_SIZE_EXT_1_SPEC>`"]
pub type D_WEIGHT_SIZE_EXT_1 = crate::Reg<d_weight_size_ext_1::D_WEIGHT_SIZE_EXT_1_SPEC>;
#[doc = "Weights channel after extension and number of weight kernels"]
pub mod d_weight_size_ext_1;
#[doc = "D_WEIGHT_BYTES register accessor: an alias for `Reg<D_WEIGHT_BYTES_SPEC>`"]
pub type D_WEIGHT_BYTES = crate::Reg<d_weight_bytes::D_WEIGHT_BYTES_SPEC>;
#[doc = "Total bytes of Weight"]
pub mod d_weight_bytes;
#[doc = "D_WMB_BYTES register accessor: an alias for `Reg<D_WMB_BYTES_SPEC>`"]
pub type D_WMB_BYTES = crate::Reg<d_wmb_bytes::D_WMB_BYTES_SPEC>;
#[doc = "Total bytes of WMB"]
pub mod d_wmb_bytes;
#[doc = "D_DATAOUT_SIZE_0 register accessor: an alias for `Reg<D_DATAOUT_SIZE_0_SPEC>`"]
pub type D_DATAOUT_SIZE_0 = crate::Reg<d_dataout_size_0::D_DATAOUT_SIZE_0_SPEC>;
#[doc = "Output cubes width and height"]
pub mod d_dataout_size_0;
#[doc = "D_DATAOUT_SIZE_1 register accessor: an alias for `Reg<D_DATAOUT_SIZE_1_SPEC>`"]
pub type D_DATAOUT_SIZE_1 = crate::Reg<d_dataout_size_1::D_DATAOUT_SIZE_1_SPEC>;
#[doc = "Output cubes channel"]
pub mod d_dataout_size_1;
#[doc = "D_ATOMICS register accessor: an alias for `Reg<D_ATOMICS_SPEC>`"]
pub type D_ATOMICS = crate::Reg<d_atomics::D_ATOMICS_SPEC>;
#[doc = "Equals to output_data_cube_width * output_data_cube_height - 1"]
pub mod d_atomics;
#[doc = "D_RELEASE register accessor: an alias for `Reg<D_RELEASE_SPEC>`"]
pub type D_RELEASE = crate::Reg<d_release::D_RELEASE_SPEC>;
#[doc = "Slices of CBUF to be released at the end of current layer"]
pub mod d_release;
#[doc = "D_CONV_STRIDE_EXT register accessor: an alias for `Reg<D_CONV_STRIDE_EXT_SPEC>`"]
pub type D_CONV_STRIDE_EXT = crate::Reg<d_conv_stride_ext::D_CONV_STRIDE_EXT_SPEC>;
#[doc = "Convolution x stride and convolution y stride after extension"]
pub mod d_conv_stride_ext;
#[doc = "D_DILATION_EXT register accessor: an alias for `Reg<D_DILATION_EXT_SPEC>`"]
pub type D_DILATION_EXT = crate::Reg<d_dilation_ext::D_DILATION_EXT_SPEC>;
#[doc = "Dilation parameter"]
pub mod d_dilation_ext;
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
#[doc = "D_PRA_CFG register accessor: an alias for `Reg<D_PRA_CFG_SPEC>`"]
pub type D_PRA_CFG = crate::Reg<d_pra_cfg::D_PRA_CFG_SPEC>;
#[doc = "PRA truncate in Winograd mode, range: 0~2"]
pub mod d_pra_cfg;
#[doc = "D_CYA register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
