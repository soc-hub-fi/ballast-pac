#[doc = r"Register block"]
#[repr(C)]
pub struct CSC {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: S_STATUS,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: S_POINTER,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: D_OP_ENABLE,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, weight reuse, data reuse."]
    pub d_misc_cfg: D_MISC_CFG,
    #[doc = "0x10 - Input data format and pixel format"]
    pub d_datain_format: D_DATAIN_FORMAT,
    #[doc = "0x14 - Input cube’s width and height after extension"]
    pub d_datain_size_ext_0: D_DATAIN_SIZE_EXT_0,
    #[doc = "0x18 - Input cube’s channel after extension"]
    pub d_datain_size_ext_1: D_DATAIN_SIZE_EXT_1,
    #[doc = "0x1c - Number of batches"]
    pub d_batch_number: D_BATCH_NUMBER,
    #[doc = "0x20 - Post extension parameter for image-in"]
    pub d_post_y_extension: D_POST_Y_EXTENSION,
    #[doc = "0x24 - Number of CBUF entries used for one input slice"]
    pub d_entry_per_slice: D_ENTRY_PER_SLICE,
    #[doc = "0x28 - Whether weight is compressed or not"]
    pub d_weight_format: D_WEIGHT_FORMAT,
    #[doc = "0x2c - Weight’s width and height after extension"]
    pub d_weight_size_ext_0: D_WEIGHT_SIZE_EXT_0,
    #[doc = "0x30 - Weight’s channel after extension and number of weight kernels"]
    pub d_weight_size_ext_1: D_WEIGHT_SIZE_EXT_1,
    #[doc = "0x34 - Total bytes of Weight"]
    pub d_weight_bytes: D_WEIGHT_BYTES,
    #[doc = "0x38 - Total bytes of WMB"]
    pub d_wmb_bytes: D_WMB_BYTES,
    #[doc = "0x3c - Output cube’s width and height"]
    pub d_dataout_size_0: D_DATAOUT_SIZE_0,
    #[doc = "0x40 - Output cube’s channel"]
    pub d_dataout_size_1: D_DATAOUT_SIZE_1,
    #[doc = "0x44 - Equals to output_data_cube_width * output_data_cube_height - 1"]
    pub d_atomics: D_ATOMICS,
    #[doc = "0x48 - Slices of CBUF to be released at the end of current layer"]
    pub d_release: D_RELEASE,
    #[doc = "0x4c - Convolution x stride and convolution y stride after extension"]
    pub d_conv_stride_ext: D_CONV_STRIDE_EXT,
    #[doc = "0x50 - Dilation parameter"]
    pub d_dilation_ext: D_DILATION_EXT,
    #[doc = "0x54 - Left/right/top/bottom padding size"]
    pub d_zero_padding: D_ZERO_PADDING,
    #[doc = "0x58 - Padding value"]
    pub d_zero_padding_value: D_ZERO_PADDING_VALUE,
    #[doc = "0x5c - Number of data banks and weight banks in CBUF"]
    pub d_bank: D_BANK,
    #[doc = "0x60 - PRA truncate in Winograd mode, range: 0~2"]
    pub d_pra_cfg: D_PRA_CFG,
    #[doc = "0x64 - "]
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
#[doc = "Configuration of operation: convolution mode, precision, weight reuse, data reuse."]
pub mod d_misc_cfg;
#[doc = "D_DATAIN_FORMAT (rw) register accessor: an alias for `Reg<D_DATAIN_FORMAT_SPEC>`"]
pub type D_DATAIN_FORMAT = crate::Reg<d_datain_format::D_DATAIN_FORMAT_SPEC>;
#[doc = "Input data format and pixel format"]
pub mod d_datain_format;
#[doc = "D_DATAIN_SIZE_EXT_0 (rw) register accessor: an alias for `Reg<D_DATAIN_SIZE_EXT_0_SPEC>`"]
pub type D_DATAIN_SIZE_EXT_0 = crate::Reg<d_datain_size_ext_0::D_DATAIN_SIZE_EXT_0_SPEC>;
#[doc = "Input cube’s width and height after extension"]
pub mod d_datain_size_ext_0;
#[doc = "D_DATAIN_SIZE_EXT_1 (rw) register accessor: an alias for `Reg<D_DATAIN_SIZE_EXT_1_SPEC>`"]
pub type D_DATAIN_SIZE_EXT_1 = crate::Reg<d_datain_size_ext_1::D_DATAIN_SIZE_EXT_1_SPEC>;
#[doc = "Input cube’s channel after extension"]
pub mod d_datain_size_ext_1;
#[doc = "D_BATCH_NUMBER (rw) register accessor: an alias for `Reg<D_BATCH_NUMBER_SPEC>`"]
pub type D_BATCH_NUMBER = crate::Reg<d_batch_number::D_BATCH_NUMBER_SPEC>;
#[doc = "Number of batches"]
pub mod d_batch_number;
#[doc = "D_POST_Y_EXTENSION (rw) register accessor: an alias for `Reg<D_POST_Y_EXTENSION_SPEC>`"]
pub type D_POST_Y_EXTENSION = crate::Reg<d_post_y_extension::D_POST_Y_EXTENSION_SPEC>;
#[doc = "Post extension parameter for image-in"]
pub mod d_post_y_extension;
#[doc = "D_ENTRY_PER_SLICE (rw) register accessor: an alias for `Reg<D_ENTRY_PER_SLICE_SPEC>`"]
pub type D_ENTRY_PER_SLICE = crate::Reg<d_entry_per_slice::D_ENTRY_PER_SLICE_SPEC>;
#[doc = "Number of CBUF entries used for one input slice"]
pub mod d_entry_per_slice;
#[doc = "D_WEIGHT_FORMAT (rw) register accessor: an alias for `Reg<D_WEIGHT_FORMAT_SPEC>`"]
pub type D_WEIGHT_FORMAT = crate::Reg<d_weight_format::D_WEIGHT_FORMAT_SPEC>;
#[doc = "Whether weight is compressed or not"]
pub mod d_weight_format;
#[doc = "D_WEIGHT_SIZE_EXT_0 (rw) register accessor: an alias for `Reg<D_WEIGHT_SIZE_EXT_0_SPEC>`"]
pub type D_WEIGHT_SIZE_EXT_0 = crate::Reg<d_weight_size_ext_0::D_WEIGHT_SIZE_EXT_0_SPEC>;
#[doc = "Weight’s width and height after extension"]
pub mod d_weight_size_ext_0;
#[doc = "D_WEIGHT_SIZE_EXT_1 (rw) register accessor: an alias for `Reg<D_WEIGHT_SIZE_EXT_1_SPEC>`"]
pub type D_WEIGHT_SIZE_EXT_1 = crate::Reg<d_weight_size_ext_1::D_WEIGHT_SIZE_EXT_1_SPEC>;
#[doc = "Weight’s channel after extension and number of weight kernels"]
pub mod d_weight_size_ext_1;
#[doc = "D_WEIGHT_BYTES (rw) register accessor: an alias for `Reg<D_WEIGHT_BYTES_SPEC>`"]
pub type D_WEIGHT_BYTES = crate::Reg<d_weight_bytes::D_WEIGHT_BYTES_SPEC>;
#[doc = "Total bytes of Weight"]
pub mod d_weight_bytes;
#[doc = "D_WMB_BYTES (rw) register accessor: an alias for `Reg<D_WMB_BYTES_SPEC>`"]
pub type D_WMB_BYTES = crate::Reg<d_wmb_bytes::D_WMB_BYTES_SPEC>;
#[doc = "Total bytes of WMB"]
pub mod d_wmb_bytes;
#[doc = "D_DATAOUT_SIZE_0 (rw) register accessor: an alias for `Reg<D_DATAOUT_SIZE_0_SPEC>`"]
pub type D_DATAOUT_SIZE_0 = crate::Reg<d_dataout_size_0::D_DATAOUT_SIZE_0_SPEC>;
#[doc = "Output cube’s width and height"]
pub mod d_dataout_size_0;
#[doc = "D_DATAOUT_SIZE_1 (rw) register accessor: an alias for `Reg<D_DATAOUT_SIZE_1_SPEC>`"]
pub type D_DATAOUT_SIZE_1 = crate::Reg<d_dataout_size_1::D_DATAOUT_SIZE_1_SPEC>;
#[doc = "Output cube’s channel"]
pub mod d_dataout_size_1;
#[doc = "D_ATOMICS (rw) register accessor: an alias for `Reg<D_ATOMICS_SPEC>`"]
pub type D_ATOMICS = crate::Reg<d_atomics::D_ATOMICS_SPEC>;
#[doc = "Equals to output_data_cube_width * output_data_cube_height - 1"]
pub mod d_atomics;
#[doc = "D_RELEASE (rw) register accessor: an alias for `Reg<D_RELEASE_SPEC>`"]
pub type D_RELEASE = crate::Reg<d_release::D_RELEASE_SPEC>;
#[doc = "Slices of CBUF to be released at the end of current layer"]
pub mod d_release;
#[doc = "D_CONV_STRIDE_EXT (rw) register accessor: an alias for `Reg<D_CONV_STRIDE_EXT_SPEC>`"]
pub type D_CONV_STRIDE_EXT = crate::Reg<d_conv_stride_ext::D_CONV_STRIDE_EXT_SPEC>;
#[doc = "Convolution x stride and convolution y stride after extension"]
pub mod d_conv_stride_ext;
#[doc = "D_DILATION_EXT (rw) register accessor: an alias for `Reg<D_DILATION_EXT_SPEC>`"]
pub type D_DILATION_EXT = crate::Reg<d_dilation_ext::D_DILATION_EXT_SPEC>;
#[doc = "Dilation parameter"]
pub mod d_dilation_ext;
#[doc = "D_ZERO_PADDING (rw) register accessor: an alias for `Reg<D_ZERO_PADDING_SPEC>`"]
pub type D_ZERO_PADDING = crate::Reg<d_zero_padding::D_ZERO_PADDING_SPEC>;
#[doc = "Left/right/top/bottom padding size"]
pub mod d_zero_padding;
#[doc = "D_ZERO_PADDING_VALUE (rw) register accessor: an alias for `Reg<D_ZERO_PADDING_VALUE_SPEC>`"]
pub type D_ZERO_PADDING_VALUE = crate::Reg<d_zero_padding_value::D_ZERO_PADDING_VALUE_SPEC>;
#[doc = "Padding value"]
pub mod d_zero_padding_value;
#[doc = "D_BANK (rw) register accessor: an alias for `Reg<D_BANK_SPEC>`"]
pub type D_BANK = crate::Reg<d_bank::D_BANK_SPEC>;
#[doc = "Number of data banks and weight banks in CBUF"]
pub mod d_bank;
#[doc = "D_PRA_CFG (rw) register accessor: an alias for `Reg<D_PRA_CFG_SPEC>`"]
pub type D_PRA_CFG = crate::Reg<d_pra_cfg::D_PRA_CFG_SPEC>;
#[doc = "PRA truncate in Winograd mode, range: 0~2"]
pub mod d_pra_cfg;
#[doc = "D_CYA (rw) register accessor: an alias for `Reg<D_CYA_SPEC>`"]
pub type D_CYA = crate::Reg<d_cya::D_CYA_SPEC>;
#[doc = ""]
pub mod d_cya;
