#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x1a0 - CFGROM"]
    pub cfgrom: CFGROM,
    _reserved1: [u8; 0x0e60],
    #[doc = "0x1000..0x1010 - GLB"]
    pub glb: GLB,
    _reserved2: [u8; 0x0ff0],
    #[doc = "0x2000..0x201c - MCIF"]
    pub mcif: MCIF,
    _reserved3: [u8; 0x0fe4],
    #[doc = "0x3000..0x30ec - CDMA"]
    pub cdma: CDMA,
    _reserved4: [u8; 0x0f14],
    #[doc = "0x4000..0x4068 - CSC"]
    pub csc: CSC,
    _reserved5: [u8; 0x0f98],
    #[doc = "0x5000..0x5010 - CMAC_A"]
    pub cmac_a: CMAC_A,
    _reserved6: [u8; 0x0ff0],
    #[doc = "0x6000..0x6010 - CMAC_B"]
    pub cmac_b: CMAC_B,
    _reserved7: [u8; 0x0ff0],
    #[doc = "0x7000..0x7038 - CACC"]
    pub cacc: CACC,
    _reserved8: [u8; 0x0fc8],
    #[doc = "0x8000..0x8094 - SDP_RDMA"]
    pub sdp_rdma: SDP_RDMA,
    _reserved9: [u8; 0x0f6c],
    #[doc = "0x9000..0x90fc - SDP"]
    pub sdp: SDP,
    _reserved10: [u8; 0x0f04],
    #[doc = "0xa000..0xa050 - PDP_RDMA"]
    pub pdp_rdma: PDP_RDMA,
    _reserved11: [u8; 0x0fb0],
    #[doc = "0xb000..0xb0a0 - PDP"]
    pub pdp: PDP,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GLB {
    #[doc = "0x00 - HW version of NVDLA"]
    pub hw_version: crate::Reg<self::glb::hw_version::HW_VERSION_SPEC>,
    #[doc = "0x04 - Interrupt mask control"]
    pub intr_mask: crate::Reg<self::glb::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x08 - Interrupt set control"]
    pub intr_set: crate::Reg<self::glb::intr_set::INTR_SET_SPEC>,
    #[doc = "0x0c - Interrupt status"]
    pub intr_status: crate::Reg<self::glb::intr_status::INTR_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "GLB"]
pub mod glb;
#[doc = r"Register block"]
#[repr(C)]
pub struct MCIF {
    #[doc = "0x00 - Register0 to control the read weight of clients in MCIF"]
    pub cfg_rd_weight_0: crate::Reg<self::mcif::cfg_rd_weight_0::CFG_RD_WEIGHT_0_SPEC>,
    #[doc = "0x04 - Register1 to control the read weight of clients in MCIF"]
    pub cfg_rd_weight_1: crate::Reg<self::mcif::cfg_rd_weight_1::CFG_RD_WEIGHT_1_SPEC>,
    #[doc = "0x08 - Register2 to control the read weight of clients in MCIF"]
    pub cfg_rd_weight_2: crate::Reg<self::mcif::cfg_rd_weight_2::CFG_RD_WEIGHT_2_SPEC>,
    #[doc = "0x0c - Register0 to control the write weight of clients in MCIF"]
    pub cfg_wr_weight_0: crate::Reg<self::mcif::cfg_wr_weight_0::CFG_WR_WEIGHT_0_SPEC>,
    #[doc = "0x10 - Register1 to control the write weight of clients in MCIF"]
    pub cfg_wr_weight_1: crate::Reg<self::mcif::cfg_wr_weight_1::CFG_WR_WEIGHT_1_SPEC>,
    #[doc = "0x14 - Outstanding AXI transactions in unit of 64Byte"]
    pub cfg_outstanding_cnt: crate::Reg<self::mcif::cfg_outstanding_cnt::CFG_OUTSTANDING_CNT_SPEC>,
    #[doc = "0x18 - Idle status of MCIF"]
    pub status: crate::Reg<self::mcif::status::STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "MCIF"]
pub mod mcif;
#[doc = r"Register block"]
#[repr(C)]
pub struct CDMA {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::cdma::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::cdma::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - WMB and Weight share same port to access external memory. This register controls the weight factor in the arbiter."]
    pub s_arbiter: crate::Reg<self::cdma::s_arbiter::S_ARBITER_SPEC>,
    #[doc = "0x0c - Indicates whether CBUF flush is finished after reset."]
    pub s_cbuf_flush_status: crate::Reg<self::cdma::s_cbuf_flush_status::S_CBUF_FLUSH_STATUS_SPEC>,
    #[doc = "0x10 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::cdma::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x14 - Configuration of operation: convolution mode, precision, weight reuse, data reuse."]
    pub d_misc_cfg: crate::Reg<self::cdma::d_misc_cfg::D_MISC_CFG_SPEC>,
    #[doc = "0x18 - Input data format and pixel format"]
    pub d_datain_format: crate::Reg<self::cdma::d_datain_format::D_DATAIN_FORMAT_SPEC>,
    #[doc = "0x1c - Input cube’s width and height"]
    pub d_datain_size_0: crate::Reg<self::cdma::d_datain_size_0::D_DATAIN_SIZE_0_SPEC>,
    #[doc = "0x20 - Input cube’s channel"]
    pub d_datain_size_1: crate::Reg<self::cdma::d_datain_size_1::D_DATAIN_SIZE_1_SPEC>,
    #[doc = "0x24 - Input cube’s width and height after extension"]
    pub d_datain_size_ext_0: crate::Reg<self::cdma::d_datain_size_ext_0::D_DATAIN_SIZE_EXT_0_SPEC>,
    #[doc = "0x28 - For image-in mode, horizontal offset and vertical offset of the 1 st pixel."]
    pub d_pixel_offset: crate::Reg<self::cdma::d_pixel_offset::D_PIXEL_OFFSET_SPEC>,
    #[doc = "0x2c - Ram type of input RAM"]
    pub d_dain_ram_type: crate::Reg<self::cdma::d_dain_ram_type::D_DAIN_RAM_TYPE_SPEC>,
    #[doc = "0x30 - Higher 32bits of input data address when axi araddr is 64bits"]
    pub d_dain_addr_high_0: crate::Reg<self::cdma::d_dain_addr_high_0::D_DAIN_ADDR_HIGH_0_SPEC>,
    #[doc = "0x34 - Lower 32bits of input data address"]
    pub d_dain_addr_low_0: crate::Reg<self::cdma::d_dain_addr_low_0::D_DAIN_ADDR_LOW_0_SPEC>,
    #[doc = "0x38 - Higher 32bits of input data address of UV plane when axi araddr is 64bits"]
    pub d_dain_addr_high_1: crate::Reg<self::cdma::d_dain_addr_high_1::D_DAIN_ADDR_HIGH_1_SPEC>,
    #[doc = "0x3c - Lower 32bits of input data address of UV plane"]
    pub d_dain_addr_low_1: crate::Reg<self::cdma::d_dain_addr_low_1::D_DAIN_ADDR_LOW_1_SPEC>,
    #[doc = "0x40 - Line stride of input cube"]
    pub d_line_stride: crate::Reg<self::cdma::d_line_stride::D_LINE_STRIDE_SPEC>,
    #[doc = "0x44 - Line stride of input cube’s UV plane"]
    pub d_line_uv_stride: crate::Reg<self::cdma::d_line_uv_stride::D_LINE_UV_STRIDE_SPEC>,
    #[doc = "0x48 - Surface stride of input cube"]
    pub d_surf_stride: crate::Reg<self::cdma::d_surf_stride::D_SURF_STRIDE_SPEC>,
    #[doc = "0x4c - Whether input cube is line packed or surface packed"]
    pub d_dain_map: crate::Reg<self::cdma::d_dain_map::D_DAIN_MAP_SPEC>,
    _reserved20: [u8; 0x08],
    #[doc = "0x58 - Number of batches"]
    pub d_batch_number: crate::Reg<self::cdma::d_batch_number::D_BATCH_NUMBER_SPEC>,
    #[doc = "0x5c - The stride of input data cubes when batches > 1"]
    pub d_batch_stride: crate::Reg<self::cdma::d_batch_stride::D_BATCH_STRIDE_SPEC>,
    #[doc = "0x60 - Number of CBUF entries used for one input slice"]
    pub d_entry_per_slice: crate::Reg<self::cdma::d_entry_per_slice::D_ENTRY_PER_SLICE_SPEC>,
    #[doc = "0x64 - Number of slices to be fetched before sending update information to CSC"]
    pub d_fetch_grain: crate::Reg<self::cdma::d_fetch_grain::D_FETCH_GRAIN_SPEC>,
    #[doc = "0x68 - Whether weight is compressed or not"]
    pub d_weight_format: crate::Reg<self::cdma::d_weight_format::D_WEIGHT_FORMAT_SPEC>,
    #[doc = "0x6c - The size of one kernel in bytes"]
    pub d_weight_size_0: crate::Reg<self::cdma::d_weight_size_0::D_WEIGHT_SIZE_0_SPEC>,
    #[doc = "0x70 - Number of kernels"]
    pub d_weight_size_1: crate::Reg<self::cdma::d_weight_size_1::D_WEIGHT_SIZE_1_SPEC>,
    #[doc = "0x74 - Ram type of weight"]
    pub d_weight_ram_type: crate::Reg<self::cdma::d_weight_ram_type::D_WEIGHT_RAM_TYPE_SPEC>,
    #[doc = "0x78 - Higher 32bits of weight address when axi araddr is 64bits"]
    pub d_weight_addr_high: crate::Reg<self::cdma::d_weight_addr_high::D_WEIGHT_ADDR_HIGH_SPEC>,
    #[doc = "0x7c - Lower 32bits of weight address"]
    pub d_weight_addr_low: crate::Reg<self::cdma::d_weight_addr_low::D_WEIGHT_ADDR_LOW_SPEC>,
    #[doc = "0x80 - Total bytes of Weight"]
    pub d_weight_bytes: crate::Reg<self::cdma::d_weight_bytes::D_WEIGHT_BYTES_SPEC>,
    #[doc = "0x84 - Higher 32bits of wgs address when axi araddr is 64bits"]
    pub d_wgs_addr_high: crate::Reg<self::cdma::d_wgs_addr_high::D_WGS_ADDR_HIGH_SPEC>,
    #[doc = "0x88 - Lower 32bits of wgs address"]
    pub d_wgs_addr_low: crate::Reg<self::cdma::d_wgs_addr_low::D_WGS_ADDR_LOW_SPEC>,
    #[doc = "0x8c - Higher 32bits of wmb address when axi araddr is 64bits"]
    pub d_wmb_addr_high: crate::Reg<self::cdma::d_wmb_addr_high::D_WMB_ADDR_HIGH_SPEC>,
    #[doc = "0x90 - Lower 32bits of wmb address"]
    pub d_wmb_addr_low: crate::Reg<self::cdma::d_wmb_addr_low::D_WMB_ADDR_LOW_SPEC>,
    #[doc = "0x94 - Total bytes of WMB"]
    pub d_wmb_bytes: crate::Reg<self::cdma::d_wmb_bytes::D_WMB_BYTES_SPEC>,
    #[doc = "0x98 - Whether mean registers are used or not"]
    pub d_mean_format: crate::Reg<self::cdma::d_mean_format::D_MEAN_FORMAT_SPEC>,
    #[doc = "0x9c - Global mean value for red in RGB or Y in YUV Global mean value for green in RGB or U in YUV"]
    pub d_mean_global_0: crate::Reg<self::cdma::d_mean_global_0::D_MEAN_GLOBAL_0_SPEC>,
    #[doc = "0xa0 - Global mean value for blue in RGB or V in YUV Global mean value for alpha in ARGB/AYUV or X in XRGB"]
    pub d_mean_global_1: crate::Reg<self::cdma::d_mean_global_1::D_MEAN_GLOBAL_1_SPEC>,
    #[doc = "0xa4 - Enable/disable input data converter in CDMA and number of bits to be truncated in the input data converter"]
    pub d_cvt_cfg: crate::Reg<self::cdma::d_cvt_cfg::D_CVT_CFG_SPEC>,
    #[doc = "0xa8 - Offset of input data convertor"]
    pub d_cvt_offset: crate::Reg<self::cdma::d_cvt_offset::D_CVT_OFFSET_SPEC>,
    #[doc = "0xac - Scale of input data convertor"]
    pub d_cvt_scale: crate::Reg<self::cdma::d_cvt_scale::D_CVT_SCALE_SPEC>,
    #[doc = "0xb0 - Convolution x stride and convolution y stride"]
    pub d_conv_stride: crate::Reg<self::cdma::d_conv_stride::D_CONV_STRIDE_SPEC>,
    #[doc = "0xb4 - Left/right/top/bottom padding size"]
    pub d_zero_padding: crate::Reg<self::cdma::d_zero_padding::D_ZERO_PADDING_SPEC>,
    #[doc = "0xb8 - Padding value"]
    pub d_zero_padding_value:
        crate::Reg<self::cdma::d_zero_padding_value::D_ZERO_PADDING_VALUE_SPEC>,
    #[doc = "0xbc - Number of data banks and weight banks in CBUF"]
    pub d_bank: crate::Reg<self::cdma::d_bank::D_BANK_SPEC>,
    #[doc = "0xc0 - Enable/Disable flushing input NaN to zero"]
    pub d_nan_flush_to_zero: crate::Reg<self::cdma::d_nan_flush_to_zero::D_NAN_FLUSH_TO_ZERO_SPEC>,
    #[doc = "0xc4 - Count NaN number in input data cube, update per layer"]
    pub d_nan_input_data_num:
        crate::Reg<self::cdma::d_nan_input_data_num::D_NAN_INPUT_DATA_NUM_SPEC>,
    #[doc = "0xc8 - Count NaN number in weight kernels, update per layer"]
    pub d_nan_input_weight_num:
        crate::Reg<self::cdma::d_nan_input_weight_num::D_NAN_INPUT_WEIGHT_NUM_SPEC>,
    #[doc = "0xcc - Count infinity number in input data cube, update per layer"]
    pub d_inf_input_data_num:
        crate::Reg<self::cdma::d_inf_input_data_num::D_INF_INPUT_DATA_NUM_SPEC>,
    #[doc = "0xd0 - Count infinity number in weight kernels, update per layer"]
    pub d_inf_input_weight_num:
        crate::Reg<self::cdma::d_inf_input_weight_num::D_INF_INPUT_WEIGHT_NUM_SPEC>,
    #[doc = "0xd4 - Enable/disable performance counter"]
    pub d_perf_enable: crate::Reg<self::cdma::d_perf_enable::D_PERF_ENABLE_SPEC>,
    #[doc = "0xd8 - Count blocking cycles of read request of input data, update per layer"]
    pub d_perf_dat_read_stall:
        crate::Reg<self::cdma::d_perf_dat_read_stall::D_PERF_DAT_READ_STALL_SPEC>,
    #[doc = "0xdc - Count blocking cycles of read request of weight data, update per layer"]
    pub d_perf_wt_read_stall:
        crate::Reg<self::cdma::d_perf_wt_read_stall::D_PERF_WT_READ_STALL_SPEC>,
    #[doc = "0xe0 - Count total latency cycles of read response of input data, update per layer"]
    pub d_perf_dat_read_latency:
        crate::Reg<self::cdma::d_perf_dat_read_latency::D_PERF_DAT_READ_LATENCY_SPEC>,
    #[doc = "0xe4 - Count total latency cycles of read request of weight data, update per layer"]
    pub d_perf_wt_read_latency:
        crate::Reg<self::cdma::d_perf_wt_read_latency::D_PERF_WT_READ_LATENCY_SPEC>,
    #[doc = "0xe8 - "]
    pub d_cya: crate::Reg<self::cdma::d_cya::D_CYA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CDMA"]
pub mod cdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct CSC {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::csc::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::csc::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::csc::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, weight reuse, data reuse."]
    pub d_misc_cfg: crate::Reg<self::csc::d_misc_cfg::D_MISC_CFG_SPEC>,
    #[doc = "0x10 - Input data format and pixel format"]
    pub d_datain_format: crate::Reg<self::csc::d_datain_format::D_DATAIN_FORMAT_SPEC>,
    #[doc = "0x14 - Input cube’s width and height after extension"]
    pub d_datain_size_ext_0: crate::Reg<self::csc::d_datain_size_ext_0::D_DATAIN_SIZE_EXT_0_SPEC>,
    #[doc = "0x18 - Input cube’s channel after extension"]
    pub d_datain_size_ext_1: crate::Reg<self::csc::d_datain_size_ext_1::D_DATAIN_SIZE_EXT_1_SPEC>,
    #[doc = "0x1c - Number of batches"]
    pub d_batch_number: crate::Reg<self::csc::d_batch_number::D_BATCH_NUMBER_SPEC>,
    #[doc = "0x20 - Post extension parameter for image-in"]
    pub d_post_y_extension: crate::Reg<self::csc::d_post_y_extension::D_POST_Y_EXTENSION_SPEC>,
    #[doc = "0x24 - Number of CBUF entries used for one input slice"]
    pub d_entry_per_slice: crate::Reg<self::csc::d_entry_per_slice::D_ENTRY_PER_SLICE_SPEC>,
    #[doc = "0x28 - Whether weight is compressed or not"]
    pub d_weight_format: crate::Reg<self::csc::d_weight_format::D_WEIGHT_FORMAT_SPEC>,
    #[doc = "0x2c - Weight’s width and height after extension"]
    pub d_weight_size_ext_0: crate::Reg<self::csc::d_weight_size_ext_0::D_WEIGHT_SIZE_EXT_0_SPEC>,
    #[doc = "0x30 - Weight’s channel after extension and number of weight kernels"]
    pub d_weight_size_ext_1: crate::Reg<self::csc::d_weight_size_ext_1::D_WEIGHT_SIZE_EXT_1_SPEC>,
    #[doc = "0x34 - Total bytes of Weight"]
    pub d_weight_bytes: crate::Reg<self::csc::d_weight_bytes::D_WEIGHT_BYTES_SPEC>,
    #[doc = "0x38 - Total bytes of WMB"]
    pub d_wmb_bytes: crate::Reg<self::csc::d_wmb_bytes::D_WMB_BYTES_SPEC>,
    #[doc = "0x3c - Output cube’s width and height"]
    pub d_dataout_size_0: crate::Reg<self::csc::d_dataout_size_0::D_DATAOUT_SIZE_0_SPEC>,
    #[doc = "0x40 - Output cube’s channel"]
    pub d_dataout_size_1: crate::Reg<self::csc::d_dataout_size_1::D_DATAOUT_SIZE_1_SPEC>,
    #[doc = "0x44 - Equals to output_data_cube_width * output_data_cube_height - 1"]
    pub d_atomics: crate::Reg<self::csc::d_atomics::D_ATOMICS_SPEC>,
    #[doc = "0x48 - Slices of CBUF to be released at the end of current layer"]
    pub d_release: crate::Reg<self::csc::d_release::D_RELEASE_SPEC>,
    #[doc = "0x4c - Convolution x stride and convolution y stride after extension"]
    pub d_conv_stride_ext: crate::Reg<self::csc::d_conv_stride_ext::D_CONV_STRIDE_EXT_SPEC>,
    #[doc = "0x50 - Dilation parameter"]
    pub d_dilation_ext: crate::Reg<self::csc::d_dilation_ext::D_DILATION_EXT_SPEC>,
    #[doc = "0x54 - Left/right/top/bottom padding size"]
    pub d_zero_padding: crate::Reg<self::csc::d_zero_padding::D_ZERO_PADDING_SPEC>,
    #[doc = "0x58 - Padding value"]
    pub d_zero_padding_value:
        crate::Reg<self::csc::d_zero_padding_value::D_ZERO_PADDING_VALUE_SPEC>,
    #[doc = "0x5c - Number of data banks and weight banks in CBUF"]
    pub d_bank: crate::Reg<self::csc::d_bank::D_BANK_SPEC>,
    #[doc = "0x60 - PRA truncate in Winograd mode, range: 0~2"]
    pub d_pra_cfg: crate::Reg<self::csc::d_pra_cfg::D_PRA_CFG_SPEC>,
    #[doc = "0x64 - "]
    pub d_cya: crate::Reg<self::csc::d_cya::D_CYA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CSC"]
pub mod csc;
#[doc = r"Register block"]
#[repr(C)]
pub struct CMAC_A {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::cmac_a::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::cmac_a::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::cmac_a::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, etc."]
    pub d_misc_cfg: crate::Reg<self::cmac_a::d_misc_cfg::D_MISC_CFG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CMAC_A"]
pub mod cmac_a;
#[doc = r"Register block"]
#[repr(C)]
pub struct CMAC_B {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::cmac_b::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::cmac_b::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::cmac_b::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, etc."]
    pub d_misc_cfg: crate::Reg<self::cmac_b::d_misc_cfg::D_MISC_CFG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CMAC_B"]
pub mod cmac_b;
#[doc = r"Register block"]
#[repr(C)]
pub struct CACC {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::cacc::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::cacc::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::cacc::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - Configuration of operation: convolution mode, precision, etc."]
    pub d_misc_cfg: crate::Reg<self::cacc::d_misc_cfg::D_MISC_CFG_SPEC>,
    #[doc = "0x10 - Input cube’s width and height after extension"]
    pub d_dataout_size_0: crate::Reg<self::cacc::d_dataout_size_0::D_DATAOUT_SIZE_0_SPEC>,
    #[doc = "0x14 - Input cube’s channel after extension"]
    pub d_dataout_size_1: crate::Reg<self::cacc::d_dataout_size_1::D_DATAOUT_SIZE_1_SPEC>,
    #[doc = "0x18 - Address of output cube"]
    pub d_dataout_addr: crate::Reg<self::cacc::d_dataout_addr::D_DATAOUT_ADDR_SPEC>,
    #[doc = "0x1c - Number of batches"]
    pub d_batch_number: crate::Reg<self::cacc::d_batch_number::D_BATCH_NUMBER_SPEC>,
    #[doc = "0x20 - Line stride of output cube"]
    pub d_line_stride: crate::Reg<self::cacc::d_line_stride::D_LINE_STRIDE_SPEC>,
    #[doc = "0x24 - Line stride of surface cube"]
    pub d_surf_stride: crate::Reg<self::cacc::d_surf_stride::D_SURF_STRIDE_SPEC>,
    #[doc = "0x28 - Whether output cube is line packed or surface packed"]
    pub d_dataout_map: crate::Reg<self::cacc::d_dataout_map::D_DATAOUT_MAP_SPEC>,
    #[doc = "0x2c - Number of bits to be truncated before sending to SDP"]
    pub d_clip_cfg: crate::Reg<self::cacc::d_clip_cfg::D_CLIP_CFG_SPEC>,
    #[doc = "0x30 - Output saturation count"]
    pub d_out_saturation: crate::Reg<self::cacc::d_out_saturation::D_OUT_SATURATION_SPEC>,
    #[doc = "0x34 - "]
    pub d_cya: crate::Reg<self::cacc::d_cya::D_CYA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CACC"]
pub mod cacc;
#[doc = r"Register block"]
#[repr(C)]
pub struct SDP_RDMA {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::sdp_rdma::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::sdp_rdma::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::sdp_rdma::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - "]
    pub d_data_cube_width: crate::Reg<self::sdp_rdma::d_data_cube_width::D_DATA_CUBE_WIDTH_SPEC>,
    #[doc = "0x10 - "]
    pub d_data_cube_height: crate::Reg<self::sdp_rdma::d_data_cube_height::D_DATA_CUBE_HEIGHT_SPEC>,
    #[doc = "0x14 - "]
    pub d_data_cube_channel:
        crate::Reg<self::sdp_rdma::d_data_cube_channel::D_DATA_CUBE_CHANNEL_SPEC>,
    #[doc = "0x18 - "]
    pub d_src_base_addr_low:
        crate::Reg<self::sdp_rdma::d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x1c - "]
    pub d_src_base_addr_high:
        crate::Reg<self::sdp_rdma::d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x20 - "]
    pub d_src_line_stride: crate::Reg<self::sdp_rdma::d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>,
    #[doc = "0x24 - "]
    pub d_src_surface_stride:
        crate::Reg<self::sdp_rdma::d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>,
    #[doc = "0x28 - "]
    pub d_brdma_cfg: crate::Reg<self::sdp_rdma::d_brdma_cfg::D_BRDMA_CFG_SPEC>,
    #[doc = "0x2c - "]
    pub d_bs_base_addr_low: crate::Reg<self::sdp_rdma::d_bs_base_addr_low::D_BS_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x30 - "]
    pub d_bs_base_addr_high:
        crate::Reg<self::sdp_rdma::d_bs_base_addr_high::D_BS_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x34 - "]
    pub d_bs_line_stride: crate::Reg<self::sdp_rdma::d_bs_line_stride::D_BS_LINE_STRIDE_SPEC>,
    #[doc = "0x38 - "]
    pub d_bs_surface_stride:
        crate::Reg<self::sdp_rdma::d_bs_surface_stride::D_BS_SURFACE_STRIDE_SPEC>,
    #[doc = "0x3c - "]
    pub d_bs_batch_stride: crate::Reg<self::sdp_rdma::d_bs_batch_stride::D_BS_BATCH_STRIDE_SPEC>,
    #[doc = "0x40 - "]
    pub d_nrdma_cfg: crate::Reg<self::sdp_rdma::d_nrdma_cfg::D_NRDMA_CFG_SPEC>,
    #[doc = "0x44 - "]
    pub d_bn_base_addr_low: crate::Reg<self::sdp_rdma::d_bn_base_addr_low::D_BN_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x48 - "]
    pub d_bn_base_addr_high:
        crate::Reg<self::sdp_rdma::d_bn_base_addr_high::D_BN_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x4c - "]
    pub d_bn_line_stride: crate::Reg<self::sdp_rdma::d_bn_line_stride::D_BN_LINE_STRIDE_SPEC>,
    #[doc = "0x50 - "]
    pub d_bn_surface_stride:
        crate::Reg<self::sdp_rdma::d_bn_surface_stride::D_BN_SURFACE_STRIDE_SPEC>,
    #[doc = "0x54 - "]
    pub d_bn_batch_stride: crate::Reg<self::sdp_rdma::d_bn_batch_stride::D_BN_BATCH_STRIDE_SPEC>,
    #[doc = "0x58 - "]
    pub d_erdma_cfg: crate::Reg<self::sdp_rdma::d_erdma_cfg::D_ERDMA_CFG_SPEC>,
    #[doc = "0x5c - "]
    pub d_ew_base_addr_low: crate::Reg<self::sdp_rdma::d_ew_base_addr_low::D_EW_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x60 - "]
    pub d_ew_base_addr_high:
        crate::Reg<self::sdp_rdma::d_ew_base_addr_high::D_EW_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x64 - "]
    pub d_ew_line_stride: crate::Reg<self::sdp_rdma::d_ew_line_stride::D_EW_LINE_STRIDE_SPEC>,
    #[doc = "0x68 - "]
    pub d_ew_surface_stride:
        crate::Reg<self::sdp_rdma::d_ew_surface_stride::D_EW_SURFACE_STRIDE_SPEC>,
    #[doc = "0x6c - "]
    pub d_ew_batch_stride: crate::Reg<self::sdp_rdma::d_ew_batch_stride::D_EW_BATCH_STRIDE_SPEC>,
    #[doc = "0x70 - "]
    pub d_feature_mode_cfg: crate::Reg<self::sdp_rdma::d_feature_mode_cfg::D_FEATURE_MODE_CFG_SPEC>,
    #[doc = "0x74 - "]
    pub d_src_dma_cfg: crate::Reg<self::sdp_rdma::d_src_dma_cfg::D_SRC_DMA_CFG_SPEC>,
    #[doc = "0x78 - "]
    pub d_status_nan_input_num:
        crate::Reg<self::sdp_rdma::d_status_nan_input_num::D_STATUS_NAN_INPUT_NUM_SPEC>,
    #[doc = "0x7c - "]
    pub d_status_inf_input_num:
        crate::Reg<self::sdp_rdma::d_status_inf_input_num::D_STATUS_INF_INPUT_NUM_SPEC>,
    #[doc = "0x80 - "]
    pub d_perf_enable: crate::Reg<self::sdp_rdma::d_perf_enable::D_PERF_ENABLE_SPEC>,
    #[doc = "0x84 - "]
    pub d_perf_mrdma_read_stall:
        crate::Reg<self::sdp_rdma::d_perf_mrdma_read_stall::D_PERF_MRDMA_READ_STALL_SPEC>,
    #[doc = "0x88 - "]
    pub d_perf_brdma_read_stall:
        crate::Reg<self::sdp_rdma::d_perf_brdma_read_stall::D_PERF_BRDMA_READ_STALL_SPEC>,
    #[doc = "0x8c - "]
    pub d_perf_nrdma_read_stall:
        crate::Reg<self::sdp_rdma::d_perf_nrdma_read_stall::D_PERF_NRDMA_READ_STALL_SPEC>,
    #[doc = "0x90 - "]
    pub d_perf_erdma_read_stall:
        crate::Reg<self::sdp_rdma::d_perf_erdma_read_stall::D_PERF_ERDMA_READ_STALL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SDP_RDMA"]
pub mod sdp_rdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct SDP {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::sdp::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::sdp::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - LUT access address and type"]
    pub s_lut_access_cfg: crate::Reg<self::sdp::s_lut_access_cfg::S_LUT_ACCESS_CFG_SPEC>,
    #[doc = "0x0c - Data register of read or write LUT"]
    pub s_lut_access_data: crate::Reg<self::sdp::s_lut_access_data::S_LUT_ACCESS_DATA_SPEC>,
    #[doc = "0x10 - LUT’s type: exponent or linear. And the selection between LE and LO tables."]
    pub s_lut_cfg: crate::Reg<self::sdp::s_lut_cfg::S_LUT_CFG_SPEC>,
    #[doc = "0x14 - LE and LO LUT index offset and selection"]
    pub s_lut_info: crate::Reg<self::sdp::s_lut_info::S_LUT_INFO_SPEC>,
    #[doc = "0x18 - Start of LE LUT’s range"]
    pub s_lut_le_start: crate::Reg<self::sdp::s_lut_le_start::S_LUT_LE_START_SPEC>,
    #[doc = "0x1c - End of LE LUT’s range"]
    pub s_lut_le_end: crate::Reg<self::sdp::s_lut_le_end::S_LUT_LE_END_SPEC>,
    #[doc = "0x20 - Start of LO LUT’s range"]
    pub s_lut_lo_start: crate::Reg<self::sdp::s_lut_lo_start::S_LUT_LO_START_SPEC>,
    #[doc = "0x24 - End of LO LUT’s range"]
    pub s_lut_lo_end: crate::Reg<self::sdp::s_lut_lo_end::S_LUT_LO_END_SPEC>,
    #[doc = "0x28 - Slope scale parameter for LE LUT underflow and overflow, signed value"]
    pub s_lut_le_slope_scale:
        crate::Reg<self::sdp::s_lut_le_slope_scale::S_LUT_LE_SLOPE_SCALE_SPEC>,
    #[doc = "0x2c - Slope shift parameter for LE_LUT underflow and overflow, signed value"]
    pub s_lut_le_slope_shift:
        crate::Reg<self::sdp::s_lut_le_slope_shift::S_LUT_LE_SLOPE_SHIFT_SPEC>,
    #[doc = "0x30 - Slope scale parameter for LO LUT underflow and overflow, signed value"]
    pub s_lut_lo_slope_scale:
        crate::Reg<self::sdp::s_lut_lo_slope_scale::S_LUT_LO_SLOPE_SCALE_SPEC>,
    #[doc = "0x34 - Slope shift parameter for LO_LUT underflow and overflow, signed value"]
    pub s_lut_lo_slope_shift:
        crate::Reg<self::sdp::s_lut_lo_slope_shift::S_LUT_LO_SLOPE_SHIFT_SPEC>,
    #[doc = "0x38 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::sdp::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x3c - Input cube’s width"]
    pub d_data_cube_width: crate::Reg<self::sdp::d_data_cube_width::D_DATA_CUBE_WIDTH_SPEC>,
    #[doc = "0x40 - Input cube’s height"]
    pub d_data_cube_height: crate::Reg<self::sdp::d_data_cube_height::D_DATA_CUBE_HEIGHT_SPEC>,
    #[doc = "0x44 - Input cube’s channel"]
    pub d_data_cube_channel: crate::Reg<self::sdp::d_data_cube_channel::D_DATA_CUBE_CHANNEL_SPEC>,
    #[doc = "0x48 - Lower 32bits of output data address"]
    pub d_dst_base_addr_low: crate::Reg<self::sdp::d_dst_base_addr_low::D_DST_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x4c - Higher 32bits of output data address when axi awaddr is 64bits"]
    pub d_dst_base_addr_high:
        crate::Reg<self::sdp::d_dst_base_addr_high::D_DST_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x50 - Line stride of output data cube"]
    pub d_dst_line_stride: crate::Reg<self::sdp::d_dst_line_stride::D_DST_LINE_STRIDE_SPEC>,
    #[doc = "0x54 - Surface stride of output data cube"]
    pub d_dst_surface_stride:
        crate::Reg<self::sdp::d_dst_surface_stride::D_DST_SURFACE_STRIDE_SPEC>,
    #[doc = "0x58 - Configurations of BS module: bypass, algorithm, etc."]
    pub d_dp_bs_cfg: crate::Reg<self::sdp::d_dp_bs_cfg::D_DP_BS_CFG_SPEC>,
    #[doc = "0x5c - Source type and shifter value of BS ALU"]
    pub d_dp_bs_alu_cfg: crate::Reg<self::sdp::d_dp_bs_alu_cfg::D_DP_BS_ALU_CFG_SPEC>,
    #[doc = "0x60 - Operand value of BS ALU"]
    pub d_dp_bs_alu_src_value:
        crate::Reg<self::sdp::d_dp_bs_alu_src_value::D_DP_BS_ALU_SRC_VALUE_SPEC>,
    #[doc = "0x64 - Source type and shifter value of BS MUL"]
    pub d_dp_bs_mul_cfg: crate::Reg<self::sdp::d_dp_bs_mul_cfg::D_DP_BS_MUL_CFG_SPEC>,
    #[doc = "0x68 - Operand value of BS MUL"]
    pub d_dp_bs_mul_src_value:
        crate::Reg<self::sdp::d_dp_bs_mul_src_value::D_DP_BS_MUL_SRC_VALUE_SPEC>,
    #[doc = "0x6c - Configurations of BN module: bypass, algorithm, etc."]
    pub d_dp_bn_cfg: crate::Reg<self::sdp::d_dp_bn_cfg::D_DP_BN_CFG_SPEC>,
    #[doc = "0x70 - Source type and shifter value of BN ALU"]
    pub d_dp_bn_alu_cfg: crate::Reg<self::sdp::d_dp_bn_alu_cfg::D_DP_BN_ALU_CFG_SPEC>,
    #[doc = "0x74 - Operand value of BN ALU"]
    pub d_dp_bn_alu_src_value:
        crate::Reg<self::sdp::d_dp_bn_alu_src_value::D_DP_BN_ALU_SRC_VALUE_SPEC>,
    #[doc = "0x78 - Source type and shifter value of BN MUL"]
    pub d_dp_bn_mul_cfg: crate::Reg<self::sdp::d_dp_bn_mul_cfg::D_DP_BN_MUL_CFG_SPEC>,
    #[doc = "0x7c - Operand value of BN MUL"]
    pub d_dp_bn_mul_src_value:
        crate::Reg<self::sdp::d_dp_bn_mul_src_value::D_DP_BN_MUL_SRC_VALUE_SPEC>,
    #[doc = "0x80 - Configurations of EW module: bypass, algorithm, etc."]
    pub d_dp_ew_cfg: crate::Reg<self::sdp::d_dp_ew_cfg::D_DP_EW_CFG_SPEC>,
    #[doc = "0x84 - Source type and bypass control of EW ALU"]
    pub d_dp_ew_alu_cfg: crate::Reg<self::sdp::d_dp_ew_alu_cfg::D_DP_EW_ALU_CFG_SPEC>,
    #[doc = "0x88 - Operand value of EW ALU"]
    pub d_dp_ew_alu_src_value:
        crate::Reg<self::sdp::d_dp_ew_alu_src_value::D_DP_EW_ALU_SRC_VALUE_SPEC>,
    #[doc = "0x8c - Converter offset of EW ALU"]
    pub d_dp_ew_alu_cvt_offset_value:
        crate::Reg<self::sdp::d_dp_ew_alu_cvt_offset_value::D_DP_EW_ALU_CVT_OFFSET_VALUE_SPEC>,
    #[doc = "0x90 - Converter scale of EW ALU"]
    pub d_dp_ew_alu_cvt_scale_value:
        crate::Reg<self::sdp::d_dp_ew_alu_cvt_scale_value::D_DP_EW_ALU_CVT_SCALE_VALUE_SPEC>,
    #[doc = "0x94 - Converter truncate of EW ALU"]
    pub d_dp_ew_alu_cvt_truncate_value:
        crate::Reg<self::sdp::d_dp_ew_alu_cvt_truncate_value::D_DP_EW_ALU_CVT_TRUNCATE_VALUE_SPEC>,
    #[doc = "0x98 - Source type and bypass control of EW MUL"]
    pub d_dp_ew_mul_cfg: crate::Reg<self::sdp::d_dp_ew_mul_cfg::D_DP_EW_MUL_CFG_SPEC>,
    #[doc = "0x9c - Operand value of EW MUL"]
    pub d_dp_ew_mul_src_value:
        crate::Reg<self::sdp::d_dp_ew_mul_src_value::D_DP_EW_MUL_SRC_VALUE_SPEC>,
    #[doc = "0xa0 - Converter offset of EW MUL"]
    pub d_dp_ew_mul_cvt_offset_value:
        crate::Reg<self::sdp::d_dp_ew_mul_cvt_offset_value::D_DP_EW_MUL_CVT_OFFSET_VALUE_SPEC>,
    #[doc = "0xa4 - Converter scale of EW MUL"]
    pub d_dp_ew_mul_cvt_scale_value:
        crate::Reg<self::sdp::d_dp_ew_mul_cvt_scale_value::D_DP_EW_MUL_CVT_SCALE_VALUE_SPEC>,
    #[doc = "0xa8 - Converter truncate of EW MUL"]
    pub d_dp_ew_mul_cvt_truncate_value:
        crate::Reg<self::sdp::d_dp_ew_mul_cvt_truncate_value::D_DP_EW_MUL_CVT_TRUNCATE_VALUE_SPEC>,
    #[doc = "0xac - Truncate of EW"]
    pub d_dp_ew_truncate_value:
        crate::Reg<self::sdp::d_dp_ew_truncate_value::D_DP_EW_TRUNCATE_VALUE_SPEC>,
    #[doc = "0xb0 - Operation configuration: flying mode, output destination, Direct or Winograd mode, flush NaN to zero, batch number."]
    pub d_feature_mode_cfg: crate::Reg<self::sdp::d_feature_mode_cfg::D_FEATURE_MODE_CFG_SPEC>,
    #[doc = "0xb4 - Destination RAM type"]
    pub d_dst_dma_cfg: crate::Reg<self::sdp::d_dst_dma_cfg::D_DST_DMA_CFG_SPEC>,
    #[doc = "0xb8 - Stride of output cubes in batch mode"]
    pub d_dst_batch_stride: crate::Reg<self::sdp::d_dst_batch_stride::D_DST_BATCH_STRIDE_SPEC>,
    #[doc = "0xbc - Data precision"]
    pub d_data_format: crate::Reg<self::sdp::d_data_format::D_DATA_FORMAT_SPEC>,
    #[doc = "0xc0 - Output converter offset"]
    pub d_cvt_offset: crate::Reg<self::sdp::d_cvt_offset::D_CVT_OFFSET_SPEC>,
    #[doc = "0xc4 - Output converter scale"]
    pub d_cvt_scale: crate::Reg<self::sdp::d_cvt_scale::D_CVT_SCALE_SPEC>,
    #[doc = "0xc8 - Output converter shifter value"]
    pub d_cvt_shift: crate::Reg<self::sdp::d_cvt_shift::D_CVT_SHIFT_SPEC>,
    #[doc = "0xcc - Output of equal mode"]
    pub d_status: crate::Reg<self::sdp::d_status::D_STATUS_SPEC>,
    #[doc = "0xd0 - Input NaN element number"]
    pub d_status_nan_input_num:
        crate::Reg<self::sdp::d_status_nan_input_num::D_STATUS_NAN_INPUT_NUM_SPEC>,
    #[doc = "0xd4 - Input Infinity element number"]
    pub d_status_inf_input_num:
        crate::Reg<self::sdp::d_status_inf_input_num::D_STATUS_INF_INPUT_NUM_SPEC>,
    #[doc = "0xd8 - Output NaN element number"]
    pub d_status_nan_output_num:
        crate::Reg<self::sdp::d_status_nan_output_num::D_STATUS_NAN_OUTPUT_NUM_SPEC>,
    #[doc = "0xdc - Enable/Disable performance counting"]
    pub d_perf_enable: crate::Reg<self::sdp::d_perf_enable::D_PERF_ENABLE_SPEC>,
    #[doc = "0xe0 - Count stall cycles of write DMA for one layer"]
    pub d_perf_wdma_write_stall:
        crate::Reg<self::sdp::d_perf_wdma_write_stall::D_PERF_WDMA_WRITE_STALL_SPEC>,
    #[doc = "0xe4 - Element number of both table underflow"]
    pub d_perf_lut_uflow: crate::Reg<self::sdp::d_perf_lut_uflow::D_PERF_LUT_UFLOW_SPEC>,
    #[doc = "0xe8 - Element number of both table overflow"]
    pub d_perf_lut_oflow: crate::Reg<self::sdp::d_perf_lut_oflow::D_PERF_LUT_OFLOW_SPEC>,
    #[doc = "0xec - Element number of both table saturation"]
    pub d_perf_out_saturation:
        crate::Reg<self::sdp::d_perf_out_saturation::D_PERF_OUT_SATURATION_SPEC>,
    #[doc = "0xf0 - Element number of both hit, or both miss situation that element underflow one table and at the same time overflow the other."]
    pub d_perf_lut_hybrid: crate::Reg<self::sdp::d_perf_lut_hybrid::D_PERF_LUT_HYBRID_SPEC>,
    #[doc = "0xf4 - Element number of only LE table hit"]
    pub d_perf_lut_le_hit: crate::Reg<self::sdp::d_perf_lut_le_hit::D_PERF_LUT_LE_HIT_SPEC>,
    #[doc = "0xf8 - Element number of only LO table hit"]
    pub d_perf_lut_lo_hit: crate::Reg<self::sdp::d_perf_lut_lo_hit::D_PERF_LUT_LO_HIT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SDP"]
pub mod sdp;
#[doc = r"Register block"]
#[repr(C)]
pub struct PDP_RDMA {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::pdp_rdma::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::pdp_rdma::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::pdp_rdma::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - "]
    pub d_data_cube_in_width:
        crate::Reg<self::pdp_rdma::d_data_cube_in_width::D_DATA_CUBE_IN_WIDTH_SPEC>,
    #[doc = "0x10 - "]
    pub d_data_cube_in_height:
        crate::Reg<self::pdp_rdma::d_data_cube_in_height::D_DATA_CUBE_IN_HEIGHT_SPEC>,
    #[doc = "0x14 - "]
    pub d_data_cube_in_channel:
        crate::Reg<self::pdp_rdma::d_data_cube_in_channel::D_DATA_CUBE_IN_CHANNEL_SPEC>,
    #[doc = "0x18 - "]
    pub d_flying_mode: crate::Reg<self::pdp_rdma::d_flying_mode::D_FLYING_MODE_SPEC>,
    #[doc = "0x1c - "]
    pub d_src_base_addr_low:
        crate::Reg<self::pdp_rdma::d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x20 - "]
    pub d_src_base_addr_high:
        crate::Reg<self::pdp_rdma::d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x24 - "]
    pub d_src_line_stride: crate::Reg<self::pdp_rdma::d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>,
    #[doc = "0x28 - "]
    pub d_src_surface_stride:
        crate::Reg<self::pdp_rdma::d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>,
    #[doc = "0x2c - "]
    pub d_src_ram_cfg: crate::Reg<self::pdp_rdma::d_src_ram_cfg::D_SRC_RAM_CFG_SPEC>,
    #[doc = "0x30 - "]
    pub d_data_format: crate::Reg<self::pdp_rdma::d_data_format::D_DATA_FORMAT_SPEC>,
    #[doc = "0x34 - "]
    pub d_operation_mode_cfg:
        crate::Reg<self::pdp_rdma::d_operation_mode_cfg::D_OPERATION_MODE_CFG_SPEC>,
    #[doc = "0x38 - "]
    pub d_pooling_kernel_cfg:
        crate::Reg<self::pdp_rdma::d_pooling_kernel_cfg::D_POOLING_KERNEL_CFG_SPEC>,
    #[doc = "0x3c - "]
    pub d_pooling_padding_cfg:
        crate::Reg<self::pdp_rdma::d_pooling_padding_cfg::D_POOLING_PADDING_CFG_SPEC>,
    #[doc = "0x40 - "]
    pub d_partial_width_in: crate::Reg<self::pdp_rdma::d_partial_width_in::D_PARTIAL_WIDTH_IN_SPEC>,
    #[doc = "0x44 - "]
    pub d_perf_enable: crate::Reg<self::pdp_rdma::d_perf_enable::D_PERF_ENABLE_SPEC>,
    #[doc = "0x48 - "]
    pub d_perf_read_stall: crate::Reg<self::pdp_rdma::d_perf_read_stall::D_PERF_READ_STALL_SPEC>,
    #[doc = "0x4c - "]
    pub d_cya: crate::Reg<self::pdp_rdma::d_cya::D_CYA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PDP_RDMA"]
pub mod pdp_rdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct PDP {
    #[doc = "0x00 - Idle status of two register groups"]
    pub s_status: crate::Reg<self::pdp::s_status::S_STATUS_SPEC>,
    #[doc = "0x04 - Pointer for CSB master and data path to access groups"]
    pub s_pointer: crate::Reg<self::pdp::s_pointer::S_POINTER_SPEC>,
    #[doc = "0x08 - Set it to 1 to kick off operation for current register group"]
    pub d_op_enable: crate::Reg<self::pdp::d_op_enable::D_OP_ENABLE_SPEC>,
    #[doc = "0x0c - Input data cube’s width"]
    pub d_data_cube_in_width:
        crate::Reg<self::pdp::d_data_cube_in_width::D_DATA_CUBE_IN_WIDTH_SPEC>,
    #[doc = "0x10 - Input data cube’s height"]
    pub d_data_cube_in_height:
        crate::Reg<self::pdp::d_data_cube_in_height::D_DATA_CUBE_IN_HEIGHT_SPEC>,
    #[doc = "0x14 - Input data cube’s channel"]
    pub d_data_cube_in_channel:
        crate::Reg<self::pdp::d_data_cube_in_channel::D_DATA_CUBE_IN_CHANNEL_SPEC>,
    #[doc = "0x18 - Output data cube’s width"]
    pub d_data_cube_out_width:
        crate::Reg<self::pdp::d_data_cube_out_width::D_DATA_CUBE_OUT_WIDTH_SPEC>,
    #[doc = "0x1c - Output data cube’s height"]
    pub d_data_cube_out_height:
        crate::Reg<self::pdp::d_data_cube_out_height::D_DATA_CUBE_OUT_HEIGHT_SPEC>,
    #[doc = "0x20 - Output data cube’s channel"]
    pub d_data_cube_out_channel:
        crate::Reg<self::pdp::d_data_cube_out_channel::D_DATA_CUBE_OUT_CHANNEL_SPEC>,
    #[doc = "0x24 - Split number"]
    pub d_operation_mode_cfg:
        crate::Reg<self::pdp::d_operation_mode_cfg::D_OPERATION_MODE_CFG_SPEC>,
    #[doc = "0x28 - Option to flush input NaN to zero"]
    pub d_nan_flush_to_zero: crate::Reg<self::pdp::d_nan_flush_to_zero::D_NAN_FLUSH_TO_ZERO_SPEC>,
    #[doc = "0x2c - Partial width for first, last and middle partitions of input cube"]
    pub d_partial_width_in: crate::Reg<self::pdp::d_partial_width_in::D_PARTIAL_WIDTH_IN_SPEC>,
    #[doc = "0x30 - Partial width for first, last and middle partitions of output cube"]
    pub d_partial_width_out: crate::Reg<self::pdp::d_partial_width_out::D_PARTIAL_WIDTH_OUT_SPEC>,
    #[doc = "0x34 - Kernel width and kernel stride"]
    pub d_pooling_kernel_cfg:
        crate::Reg<self::pdp::d_pooling_kernel_cfg::D_POOLING_KERNEL_CFG_SPEC>,
    #[doc = "0x38 - Reciprocal of pooling kernel width, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
    pub d_recip_kernel_width:
        crate::Reg<self::pdp::d_recip_kernel_width::D_RECIP_KERNEL_WIDTH_SPEC>,
    #[doc = "0x3c - Reciprocal of pooling kernel height, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format."]
    pub d_recip_kernel_height:
        crate::Reg<self::pdp::d_recip_kernel_height::D_RECIP_KERNEL_HEIGHT_SPEC>,
    #[doc = "0x40 - Left/right/top/bottom padding size"]
    pub d_pooling_padding_cfg:
        crate::Reg<self::pdp::d_pooling_padding_cfg::D_POOLING_PADDING_CFG_SPEC>,
    #[doc = "0x44 - Padding_value*1"]
    pub d_pooling_padding_value_1_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_1_cfg::D_POOLING_PADDING_VALUE_1_CFG_SPEC>,
    #[doc = "0x48 - Padding_value*2"]
    pub d_pooling_padding_value_2_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_2_cfg::D_POOLING_PADDING_VALUE_2_CFG_SPEC>,
    #[doc = "0x4c - Padding_value*3"]
    pub d_pooling_padding_value_3_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_3_cfg::D_POOLING_PADDING_VALUE_3_CFG_SPEC>,
    #[doc = "0x50 - Padding_value*4"]
    pub d_pooling_padding_value_4_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_4_cfg::D_POOLING_PADDING_VALUE_4_CFG_SPEC>,
    #[doc = "0x54 - Padding_value*5"]
    pub d_pooling_padding_value_5_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_5_cfg::D_POOLING_PADDING_VALUE_5_CFG_SPEC>,
    #[doc = "0x58 - Padding_value*6"]
    pub d_pooling_padding_value_6_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_6_cfg::D_POOLING_PADDING_VALUE_6_CFG_SPEC>,
    #[doc = "0x5c - Padding_value*7"]
    pub d_pooling_padding_value_7_cfg:
        crate::Reg<self::pdp::d_pooling_padding_value_7_cfg::D_POOLING_PADDING_VALUE_7_CFG_SPEC>,
    #[doc = "0x60 - Lower 32bits of input data address"]
    pub d_src_base_addr_low: crate::Reg<self::pdp::d_src_base_addr_low::D_SRC_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x64 - Higher 32bits of input data address when axi araddr is 64bits"]
    pub d_src_base_addr_high:
        crate::Reg<self::pdp::d_src_base_addr_high::D_SRC_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x68 - Line stride of input cube"]
    pub d_src_line_stride: crate::Reg<self::pdp::d_src_line_stride::D_SRC_LINE_STRIDE_SPEC>,
    #[doc = "0x6c - Surface stride of input cube"]
    pub d_src_surface_stride:
        crate::Reg<self::pdp::d_src_surface_stride::D_SRC_SURFACE_STRIDE_SPEC>,
    #[doc = "0x70 - Lower 32bits of output data address"]
    pub d_dst_base_addr_low: crate::Reg<self::pdp::d_dst_base_addr_low::D_DST_BASE_ADDR_LOW_SPEC>,
    #[doc = "0x74 - Higher 32bits of output data address when axi awaddr is 64bits"]
    pub d_dst_base_addr_high:
        crate::Reg<self::pdp::d_dst_base_addr_high::D_DST_BASE_ADDR_HIGH_SPEC>,
    #[doc = "0x78 - Line stride of output cube"]
    pub d_dst_line_stride: crate::Reg<self::pdp::d_dst_line_stride::D_DST_LINE_STRIDE_SPEC>,
    #[doc = "0x7c - Surface stride of output cube"]
    pub d_dst_surface_stride:
        crate::Reg<self::pdp::d_dst_surface_stride::D_DST_SURFACE_STRIDE_SPEC>,
    #[doc = "0x80 - RAM type of destination cube"]
    pub d_dst_ram_cfg: crate::Reg<self::pdp::d_dst_ram_cfg::D_DST_RAM_CFG_SPEC>,
    #[doc = "0x84 - Precision of input data"]
    pub d_data_format: crate::Reg<self::pdp::d_data_format::D_DATA_FORMAT_SPEC>,
    #[doc = "0x88 - Input infinity element number"]
    pub d_inf_input_num: crate::Reg<self::pdp::d_inf_input_num::D_INF_INPUT_NUM_SPEC>,
    #[doc = "0x8c - Input NaN element number"]
    pub d_nan_input_num: crate::Reg<self::pdp::d_nan_input_num::D_NAN_INPUT_NUM_SPEC>,
    #[doc = "0x90 - Output NaN element number"]
    pub d_nan_output_num: crate::Reg<self::pdp::d_nan_output_num::D_NAN_OUTPUT_NUM_SPEC>,
    #[doc = "0x94 - Enable/disable performance counting"]
    pub d_perf_enable: crate::Reg<self::pdp::d_perf_enable::D_PERF_ENABLE_SPEC>,
    #[doc = "0x98 - Counting stalls of write requests"]
    pub d_perf_write_stall: crate::Reg<self::pdp::d_perf_write_stall::D_PERF_WRITE_STALL_SPEC>,
    #[doc = "0x9c - "]
    pub d_cya: crate::Reg<self::pdp::d_cya::D_CYA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PDP"]
pub mod pdp;
#[doc = r"Register block"]
#[repr(C)]
pub struct CFGROM {
    #[doc = "0x00 - "]
    pub hw_version_0: crate::Reg<self::cfgrom::hw_version_0::HW_VERSION_0_SPEC>,
    #[doc = "0x04 - "]
    pub glb_desc_0: crate::Reg<self::cfgrom::glb_desc_0::GLB_DESC_0_SPEC>,
    #[doc = "0x08 - "]
    pub cif_desc_0: crate::Reg<self::cfgrom::cif_desc_0::CIF_DESC_0_SPEC>,
    #[doc = "0x0c - "]
    pub cif_cap_incompat_0: crate::Reg<self::cfgrom::cif_cap_incompat_0::CIF_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x10 - "]
    pub cif_cap_compat_0: crate::Reg<self::cfgrom::cif_cap_compat_0::CIF_CAP_COMPAT_0_SPEC>,
    #[doc = "0x14 - "]
    pub cif_base_width_0: crate::Reg<self::cfgrom::cif_base_width_0::CIF_BASE_WIDTH_0_SPEC>,
    #[doc = "0x18 - "]
    pub cif_base_latency_0: crate::Reg<self::cfgrom::cif_base_latency_0::CIF_BASE_LATENCY_0_SPEC>,
    #[doc = "0x1c - "]
    pub cif_base_burst_length_max_0:
        crate::Reg<self::cfgrom::cif_base_burst_length_max_0::CIF_BASE_BURST_LENGTH_MAX_0_SPEC>,
    #[doc = "0x20 - "]
    pub cif_base_mem_addr_width_0:
        crate::Reg<self::cfgrom::cif_base_mem_addr_width_0::CIF_BASE_MEM_ADDR_WIDTH_0_SPEC>,
    #[doc = "0x24 - "]
    pub cdma_desc_0: crate::Reg<self::cfgrom::cdma_desc_0::CDMA_DESC_0_SPEC>,
    #[doc = "0x28 - "]
    pub cdma_cap_incompat_0:
        crate::Reg<self::cfgrom::cdma_cap_incompat_0::CDMA_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x2c - "]
    pub cdma_cap_compat_0: crate::Reg<self::cfgrom::cdma_cap_compat_0::CDMA_CAP_COMPAT_0_SPEC>,
    #[doc = "0x30 - "]
    pub cdma_base_feature_types_0:
        crate::Reg<self::cfgrom::cdma_base_feature_types_0::CDMA_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0x34 - "]
    pub cdma_base_weight_types_0:
        crate::Reg<self::cfgrom::cdma_base_weight_types_0::CDMA_BASE_WEIGHT_TYPES_0_SPEC>,
    #[doc = "0x38 - "]
    pub cdma_base_atomic_c_0:
        crate::Reg<self::cfgrom::cdma_base_atomic_c_0::CDMA_BASE_ATOMIC_C_0_SPEC>,
    #[doc = "0x3c - "]
    pub cdma_base_atomic_k_0:
        crate::Reg<self::cfgrom::cdma_base_atomic_k_0::CDMA_BASE_ATOMIC_K_0_SPEC>,
    #[doc = "0x40 - "]
    pub cdma_base_atomic_m_0:
        crate::Reg<self::cfgrom::cdma_base_atomic_m_0::CDMA_BASE_ATOMIC_M_0_SPEC>,
    #[doc = "0x44 - "]
    pub cdma_base_cbuf_bank_num_0:
        crate::Reg<self::cfgrom::cdma_base_cbuf_bank_num_0::CDMA_BASE_CBUF_BANK_NUM_0_SPEC>,
    #[doc = "0x48 - "]
    pub cdma_base_cbuf_bank_width_0:
        crate::Reg<self::cfgrom::cdma_base_cbuf_bank_width_0::CDMA_BASE_CBUF_BANK_WIDTH_0_SPEC>,
    #[doc = "0x4c - "]
    pub cdma_base_cbuf_bank_depth_0:
        crate::Reg<self::cfgrom::cdma_base_cbuf_bank_depth_0::CDMA_BASE_CBUF_BANK_DEPTH_0_SPEC>,
    #[doc = "0x50 - "]
    pub cdma_multi_batch_max_0:
        crate::Reg<self::cfgrom::cdma_multi_batch_max_0::CDMA_MULTI_BATCH_MAX_0_SPEC>,
    #[doc = "0x54 - "]
    pub cdma_image_in_formats_packed_0: crate::Reg<
        self::cfgrom::cdma_image_in_formats_packed_0::CDMA_IMAGE_IN_FORMATS_PACKED_0_SPEC,
    >,
    #[doc = "0x58 - "]
    pub cdma_image_in_formats_semi_0:
        crate::Reg<self::cfgrom::cdma_image_in_formats_semi_0::CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>,
    #[doc = "0x5c - "]
    pub cbuf_desc_0: crate::Reg<self::cfgrom::cbuf_desc_0::CBUF_DESC_0_SPEC>,
    #[doc = "0x60 - "]
    pub cbuf_cap_incompat_0:
        crate::Reg<self::cfgrom::cbuf_cap_incompat_0::CBUF_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x64 - "]
    pub cbuf_cap_compat_0: crate::Reg<self::cfgrom::cbuf_cap_compat_0::CBUF_CAP_COMPAT_0_SPEC>,
    #[doc = "0x68 - "]
    pub cbuf_base_cbuf_bank_num_0:
        crate::Reg<self::cfgrom::cbuf_base_cbuf_bank_num_0::CBUF_BASE_CBUF_BANK_NUM_0_SPEC>,
    #[doc = "0x6c - "]
    pub cbuf_base_cbuf_bank_width_0:
        crate::Reg<self::cfgrom::cbuf_base_cbuf_bank_width_0::CBUF_BASE_CBUF_BANK_WIDTH_0_SPEC>,
    #[doc = "0x70 - "]
    pub cbuf_base_cbuf_bank_depth_0:
        crate::Reg<self::cfgrom::cbuf_base_cbuf_bank_depth_0::CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>,
    #[doc = "0x74 - "]
    pub cbuf_base_cdma_id_0:
        crate::Reg<self::cfgrom::cbuf_base_cdma_id_0::CBUF_BASE_CDMA_ID_0_SPEC>,
    #[doc = "0x78 - "]
    pub csc_desc_0: crate::Reg<self::cfgrom::csc_desc_0::CSC_DESC_0_SPEC>,
    #[doc = "0x7c - "]
    pub csc_cap_incompat_0: crate::Reg<self::cfgrom::csc_cap_incompat_0::CSC_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x80 - "]
    pub csc_cap_compat_0: crate::Reg<self::cfgrom::csc_cap_compat_0::CSC_CAP_COMPAT_0_SPEC>,
    #[doc = "0x84 - "]
    pub csc_base_feature_types_0:
        crate::Reg<self::cfgrom::csc_base_feature_types_0::CSC_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0x88 - "]
    pub csc_base_weight_types_0:
        crate::Reg<self::cfgrom::csc_base_weight_types_0::CSC_BASE_WEIGHT_TYPES_0_SPEC>,
    #[doc = "0x8c - "]
    pub csc_base_atomic_c_0:
        crate::Reg<self::cfgrom::csc_base_atomic_c_0::CSC_BASE_ATOMIC_C_0_SPEC>,
    #[doc = "0x90 - "]
    pub csc_base_atomic_k_0:
        crate::Reg<self::cfgrom::csc_base_atomic_k_0::CSC_BASE_ATOMIC_K_0_SPEC>,
    #[doc = "0x94 - "]
    pub csc_base_atomic_m_0:
        crate::Reg<self::cfgrom::csc_base_atomic_m_0::CSC_BASE_ATOMIC_M_0_SPEC>,
    #[doc = "0x98 - "]
    pub csc_base_cbuf_bank_num_0:
        crate::Reg<self::cfgrom::csc_base_cbuf_bank_num_0::CSC_BASE_CBUF_BANK_NUM_0_SPEC>,
    #[doc = "0x9c - "]
    pub csc_base_cbuf_bank_width_0:
        crate::Reg<self::cfgrom::csc_base_cbuf_bank_width_0::CSC_BASE_CBUF_BANK_WIDTH_0_SPEC>,
    #[doc = "0xa0 - "]
    pub csc_base_cbuf_bank_depth_0:
        crate::Reg<self::cfgrom::csc_base_cbuf_bank_depth_0::CSC_BASE_CBUF_BANK_DEPTH_0_SPEC>,
    #[doc = "0xa4 - "]
    pub csc_base_cdma_id_0: crate::Reg<self::cfgrom::csc_base_cdma_id_0::CSC_BASE_CDMA_ID_0_SPEC>,
    #[doc = "0xa8 - "]
    pub csc_multi_batch_max_0:
        crate::Reg<self::cfgrom::csc_multi_batch_max_0::CSC_MULTI_BATCH_MAX_0_SPEC>,
    #[doc = "0xac - "]
    pub cmac_a_desc_0: crate::Reg<self::cfgrom::cmac_a_desc_0::CMAC_A_DESC_0_SPEC>,
    #[doc = "0xb0 - "]
    pub cmac_a_cap_incompat_0:
        crate::Reg<self::cfgrom::cmac_a_cap_incompat_0::CMAC_A_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0xb4 - "]
    pub cmac_a_cap_compat_0:
        crate::Reg<self::cfgrom::cmac_a_cap_compat_0::CMAC_A_CAP_COMPAT_0_SPEC>,
    #[doc = "0xb8 - "]
    pub cmac_a_base_feature_types_0:
        crate::Reg<self::cfgrom::cmac_a_base_feature_types_0::CMAC_A_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0xbc - "]
    pub cmac_a_base_weight_types_0:
        crate::Reg<self::cfgrom::cmac_a_base_weight_types_0::CMAC_A_BASE_WEIGHT_TYPES_0_SPEC>,
    #[doc = "0xc0 - "]
    pub cmac_a_base_atomic_c_0:
        crate::Reg<self::cfgrom::cmac_a_base_atomic_c_0::CMAC_A_BASE_ATOMIC_C_0_SPEC>,
    #[doc = "0xc4 - "]
    pub cmac_a_base_atomic_k_0:
        crate::Reg<self::cfgrom::cmac_a_base_atomic_k_0::CMAC_A_BASE_ATOMIC_K_0_SPEC>,
    #[doc = "0xc8 - "]
    pub cmac_a_base_cdma_id_0:
        crate::Reg<self::cfgrom::cmac_a_base_cdma_id_0::CMAC_A_BASE_CDMA_ID_0_SPEC>,
    #[doc = "0xcc - "]
    pub cmac_b_desc_0: crate::Reg<self::cfgrom::cmac_b_desc_0::CMAC_B_DESC_0_SPEC>,
    #[doc = "0xd0 - "]
    pub cmac_b_cap_incompat_0:
        crate::Reg<self::cfgrom::cmac_b_cap_incompat_0::CMAC_B_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0xd4 - "]
    pub cmac_b_cap_compat_0:
        crate::Reg<self::cfgrom::cmac_b_cap_compat_0::CMAC_B_CAP_COMPAT_0_SPEC>,
    #[doc = "0xd8 - "]
    pub cmac_b_base_feature_types_0:
        crate::Reg<self::cfgrom::cmac_b_base_feature_types_0::CMAC_B_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0xdc - "]
    pub cmac_b_base_weight_types_0:
        crate::Reg<self::cfgrom::cmac_b_base_weight_types_0::CMAC_B_BASE_WEIGHT_TYPES_0_SPEC>,
    #[doc = "0xe0 - "]
    pub cmac_b_base_atomic_c_0:
        crate::Reg<self::cfgrom::cmac_b_base_atomic_c_0::CMAC_B_BASE_ATOMIC_C_0_SPEC>,
    #[doc = "0xe4 - "]
    pub cmac_b_base_atomic_k_0:
        crate::Reg<self::cfgrom::cmac_b_base_atomic_k_0::CMAC_B_BASE_ATOMIC_K_0_SPEC>,
    #[doc = "0xe8 - "]
    pub cmac_b_base_cdma_id_0:
        crate::Reg<self::cfgrom::cmac_b_base_cdma_id_0::CMAC_B_BASE_CDMA_ID_0_SPEC>,
    #[doc = "0xec - "]
    pub cacc_desc_0: crate::Reg<self::cfgrom::cacc_desc_0::CACC_DESC_0_SPEC>,
    #[doc = "0xf0 - "]
    pub cacc_cap_incompat_0:
        crate::Reg<self::cfgrom::cacc_cap_incompat_0::CACC_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0xf4 - "]
    pub cacc_cap_compat_0: crate::Reg<self::cfgrom::cacc_cap_compat_0::CACC_CAP_COMPAT_0_SPEC>,
    #[doc = "0xf8 - "]
    pub cacc_base_feature_types_0:
        crate::Reg<self::cfgrom::cacc_base_feature_types_0::CACC_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0xfc - "]
    pub cacc_base_weight_types_0:
        crate::Reg<self::cfgrom::cacc_base_weight_types_0::CACC_BASE_WEIGHT_TYPES_0_SPEC>,
    #[doc = "0x100 - "]
    pub cacc_base_atomic_c_0:
        crate::Reg<self::cfgrom::cacc_base_atomic_c_0::CACC_BASE_ATOMIC_C_0_SPEC>,
    #[doc = "0x104 - "]
    pub cacc_base_atomic_k_0:
        crate::Reg<self::cfgrom::cacc_base_atomic_k_0::CACC_BASE_ATOMIC_K_0_SPEC>,
    #[doc = "0x108 - "]
    pub cacc_base_cdma_id_0:
        crate::Reg<self::cfgrom::cacc_base_cdma_id_0::CACC_BASE_CDMA_ID_0_SPEC>,
    #[doc = "0x10c - "]
    pub cacc_multi_batch_max_0:
        crate::Reg<self::cfgrom::cacc_multi_batch_max_0::CACC_MULTI_BATCH_MAX_0_SPEC>,
    #[doc = "0x110 - "]
    pub sdp_rdma_desc_0: crate::Reg<self::cfgrom::sdp_rdma_desc_0::SDP_RDMA_DESC_0_SPEC>,
    #[doc = "0x114 - "]
    pub sdp_rdma_cap_incompat_0:
        crate::Reg<self::cfgrom::sdp_rdma_cap_incompat_0::SDP_RDMA_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x118 - "]
    pub sdp_rdma_cap_compat_0:
        crate::Reg<self::cfgrom::sdp_rdma_cap_compat_0::SDP_RDMA_CAP_COMPAT_0_SPEC>,
    #[doc = "0x11c - "]
    pub sdp_rdma_base_atomic_m_0:
        crate::Reg<self::cfgrom::sdp_rdma_base_atomic_m_0::SDP_RDMA_BASE_ATOMIC_M_0_SPEC>,
    #[doc = "0x120 - "]
    pub sdp_rdma_base_sdp_id_0:
        crate::Reg<self::cfgrom::sdp_rdma_base_sdp_id_0::SDP_RDMA_BASE_SDP_ID_0_SPEC>,
    #[doc = "0x124 - "]
    pub sdp_desc_0: crate::Reg<self::cfgrom::sdp_desc_0::SDP_DESC_0_SPEC>,
    #[doc = "0x128 - "]
    pub sdp_cap_incompat_0: crate::Reg<self::cfgrom::sdp_cap_incompat_0::SDP_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x12c - "]
    pub sdp_cap_compat_0: crate::Reg<self::cfgrom::sdp_cap_compat_0::SDP_CAP_COMPAT_0_SPEC>,
    #[doc = "0x130 - "]
    pub sdp_base_feature_types_0:
        crate::Reg<self::cfgrom::sdp_base_feature_types_0::SDP_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0x134 - "]
    pub sdp_base_weight_types_0:
        crate::Reg<self::cfgrom::sdp_base_weight_types_0::SDP_BASE_WEIGHT_TYPES_0_SPEC>,
    #[doc = "0x138 - "]
    pub sdp_base_cdma_id_0: crate::Reg<self::cfgrom::sdp_base_cdma_id_0::SDP_BASE_CDMA_ID_0_SPEC>,
    #[doc = "0x13c - "]
    pub sdp_multi_batch_max_0:
        crate::Reg<self::cfgrom::sdp_multi_batch_max_0::SDP_MULTI_BATCH_MAX_0_SPEC>,
    #[doc = "0x140 - "]
    pub sdp_bs_throughput_0:
        crate::Reg<self::cfgrom::sdp_bs_throughput_0::SDP_BS_THROUGHPUT_0_SPEC>,
    #[doc = "0x144 - "]
    pub sdp_bn_throughput_0:
        crate::Reg<self::cfgrom::sdp_bn_throughput_0::SDP_BN_THROUGHPUT_0_SPEC>,
    #[doc = "0x148 - "]
    pub sdp_ew_throughput_0:
        crate::Reg<self::cfgrom::sdp_ew_throughput_0::SDP_EW_THROUGHPUT_0_SPEC>,
    #[doc = "0x14c - "]
    pub pdp_rdma_desc_0: crate::Reg<self::cfgrom::pdp_rdma_desc_0::PDP_RDMA_DESC_0_SPEC>,
    #[doc = "0x150 - "]
    pub pdp_rdma_cap_incompat_0:
        crate::Reg<self::cfgrom::pdp_rdma_cap_incompat_0::PDP_RDMA_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x154 - "]
    pub pdp_rdma_cap_compat_0:
        crate::Reg<self::cfgrom::pdp_rdma_cap_compat_0::PDP_RDMA_CAP_COMPAT_0_SPEC>,
    #[doc = "0x158 - "]
    pub pdp_rdma_base_atomic_m_0:
        crate::Reg<self::cfgrom::pdp_rdma_base_atomic_m_0::PDP_RDMA_BASE_ATOMIC_M_0_SPEC>,
    #[doc = "0x15c - "]
    pub pdp_rdma_base_pdp_id_0:
        crate::Reg<self::cfgrom::pdp_rdma_base_pdp_id_0::PDP_RDMA_BASE_PDP_ID_0_SPEC>,
    #[doc = "0x160 - "]
    pub pdp_desc_0: crate::Reg<self::cfgrom::pdp_desc_0::PDP_DESC_0_SPEC>,
    #[doc = "0x164 - "]
    pub pdp_cap_incompat_0: crate::Reg<self::cfgrom::pdp_cap_incompat_0::PDP_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x168 - "]
    pub pdp_cap_compat_0: crate::Reg<self::cfgrom::pdp_cap_compat_0::PDP_CAP_COMPAT_0_SPEC>,
    #[doc = "0x16c - "]
    pub pdp_base_feature_types_0:
        crate::Reg<self::cfgrom::pdp_base_feature_types_0::PDP_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0x170 - "]
    pub pdp_base_throughput_0:
        crate::Reg<self::cfgrom::pdp_base_throughput_0::PDP_BASE_THROUGHPUT_0_SPEC>,
    #[doc = "0x174 - "]
    pub cdp_rdma_desc_0: crate::Reg<self::cfgrom::cdp_rdma_desc_0::CDP_RDMA_DESC_0_SPEC>,
    #[doc = "0x178 - "]
    pub cdp_rdma_cap_incompat_0:
        crate::Reg<self::cfgrom::cdp_rdma_cap_incompat_0::CDP_RDMA_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x17c - "]
    pub cdp_rdma_cap_compat_0:
        crate::Reg<self::cfgrom::cdp_rdma_cap_compat_0::CDP_RDMA_CAP_COMPAT_0_SPEC>,
    #[doc = "0x180 - "]
    pub cdp_rdma_base_atomic_m_0:
        crate::Reg<self::cfgrom::cdp_rdma_base_atomic_m_0::CDP_RDMA_BASE_ATOMIC_M_0_SPEC>,
    #[doc = "0x184 - "]
    pub cdp_rdma_base_cdp_id_0:
        crate::Reg<self::cfgrom::cdp_rdma_base_cdp_id_0::CDP_RDMA_BASE_CDP_ID_0_SPEC>,
    #[doc = "0x188 - "]
    pub cdp_desc_0: crate::Reg<self::cfgrom::cdp_desc_0::CDP_DESC_0_SPEC>,
    #[doc = "0x18c - "]
    pub cdp_cap_incompat_0: crate::Reg<self::cfgrom::cdp_cap_incompat_0::CDP_CAP_INCOMPAT_0_SPEC>,
    #[doc = "0x190 - "]
    pub cdp_cap_compat_0: crate::Reg<self::cfgrom::cdp_cap_compat_0::CDP_CAP_COMPAT_0_SPEC>,
    #[doc = "0x194 - "]
    pub cdp_base_feature_types_0:
        crate::Reg<self::cfgrom::cdp_base_feature_types_0::CDP_BASE_FEATURE_TYPES_0_SPEC>,
    #[doc = "0x198 - "]
    pub cdp_base_throughput_0:
        crate::Reg<self::cfgrom::cdp_base_throughput_0::CDP_BASE_THROUGHPUT_0_SPEC>,
    #[doc = "0x19c - "]
    pub end_of_list_0: crate::Reg<self::cfgrom::end_of_list_0::END_OF_LIST_0_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CFGROM"]
pub mod cfgrom;
