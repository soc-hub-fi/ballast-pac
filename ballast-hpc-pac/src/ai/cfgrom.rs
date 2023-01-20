#[doc = r"Register block"]
#[repr(C)]
pub struct CFGROM {
    #[doc = "0x00 - "]
    pub hw_version_0: HW_VERSION_0,
    #[doc = "0x04 - "]
    pub glb_desc_0: GLB_DESC_0,
    #[doc = "0x08 - "]
    pub cif_desc_0: CIF_DESC_0,
    #[doc = "0x0c - "]
    pub cif_cap_incompat_0: CIF_CAP_INCOMPAT_0,
    #[doc = "0x10 - "]
    pub cif_cap_compat_0: CIF_CAP_COMPAT_0,
    #[doc = "0x14 - "]
    pub cif_base_width_0: CIF_BASE_WIDTH_0,
    #[doc = "0x18 - "]
    pub cif_base_latency_0: CIF_BASE_LATENCY_0,
    #[doc = "0x1c - "]
    pub cif_base_burst_length_max_0: CIF_BASE_BURST_LENGTH_MAX_0,
    #[doc = "0x20 - "]
    pub cif_base_mem_addr_width_0: CIF_BASE_MEM_ADDR_WIDTH_0,
    #[doc = "0x24 - "]
    pub cdma_desc_0: CDMA_DESC_0,
    #[doc = "0x28 - "]
    pub cdma_cap_incompat_0: CDMA_CAP_INCOMPAT_0,
    #[doc = "0x2c - "]
    pub cdma_cap_compat_0: CDMA_CAP_COMPAT_0,
    #[doc = "0x30 - "]
    pub cdma_base_feature_types_0: CDMA_BASE_FEATURE_TYPES_0,
    #[doc = "0x34 - "]
    pub cdma_base_weight_types_0: CDMA_BASE_WEIGHT_TYPES_0,
    #[doc = "0x38 - "]
    pub cdma_base_atomic_c_0: CDMA_BASE_ATOMIC_C_0,
    #[doc = "0x3c - "]
    pub cdma_base_atomic_k_0: CDMA_BASE_ATOMIC_K_0,
    #[doc = "0x40 - "]
    pub cdma_base_atomic_m_0: CDMA_BASE_ATOMIC_M_0,
    #[doc = "0x44 - "]
    pub cdma_base_cbuf_bank_num_0: CDMA_BASE_CBUF_BANK_NUM_0,
    #[doc = "0x48 - "]
    pub cdma_base_cbuf_bank_width_0: CDMA_BASE_CBUF_BANK_WIDTH_0,
    #[doc = "0x4c - "]
    pub cdma_base_cbuf_bank_depth_0: CDMA_BASE_CBUF_BANK_DEPTH_0,
    #[doc = "0x50 - "]
    pub cdma_multi_batch_max_0: CDMA_MULTI_BATCH_MAX_0,
    #[doc = "0x54 - "]
    pub cdma_image_in_formats_packed_0: CDMA_IMAGE_IN_FORMATS_PACKED_0,
    #[doc = "0x58 - "]
    pub cdma_image_in_formats_semi_0: CDMA_IMAGE_IN_FORMATS_SEMI_0,
    #[doc = "0x5c - "]
    pub cbuf_desc_0: CBUF_DESC_0,
    #[doc = "0x60 - "]
    pub cbuf_cap_incompat_0: CBUF_CAP_INCOMPAT_0,
    #[doc = "0x64 - "]
    pub cbuf_cap_compat_0: CBUF_CAP_COMPAT_0,
    #[doc = "0x68 - "]
    pub cbuf_base_cbuf_bank_num_0: CBUF_BASE_CBUF_BANK_NUM_0,
    #[doc = "0x6c - "]
    pub cbuf_base_cbuf_bank_width_0: CBUF_BASE_CBUF_BANK_WIDTH_0,
    #[doc = "0x70 - "]
    pub cbuf_base_cbuf_bank_depth_0: CBUF_BASE_CBUF_BANK_DEPTH_0,
    #[doc = "0x74 - "]
    pub cbuf_base_cdma_id_0: CBUF_BASE_CDMA_ID_0,
    #[doc = "0x78 - "]
    pub csc_desc_0: CSC_DESC_0,
    #[doc = "0x7c - "]
    pub csc_cap_incompat_0: CSC_CAP_INCOMPAT_0,
    #[doc = "0x80 - "]
    pub csc_cap_compat_0: CSC_CAP_COMPAT_0,
    #[doc = "0x84 - "]
    pub csc_base_feature_types_0: CSC_BASE_FEATURE_TYPES_0,
    #[doc = "0x88 - "]
    pub csc_base_weight_types_0: CSC_BASE_WEIGHT_TYPES_0,
    #[doc = "0x8c - "]
    pub csc_base_atomic_c_0: CSC_BASE_ATOMIC_C_0,
    #[doc = "0x90 - "]
    pub csc_base_atomic_k_0: CSC_BASE_ATOMIC_K_0,
    #[doc = "0x94 - "]
    pub csc_base_atomic_m_0: CSC_BASE_ATOMIC_M_0,
    #[doc = "0x98 - "]
    pub csc_base_cbuf_bank_num_0: CSC_BASE_CBUF_BANK_NUM_0,
    #[doc = "0x9c - "]
    pub csc_base_cbuf_bank_width_0: CSC_BASE_CBUF_BANK_WIDTH_0,
    #[doc = "0xa0 - "]
    pub csc_base_cbuf_bank_depth_0: CSC_BASE_CBUF_BANK_DEPTH_0,
    #[doc = "0xa4 - "]
    pub csc_base_cdma_id_0: CSC_BASE_CDMA_ID_0,
    #[doc = "0xa8 - "]
    pub csc_multi_batch_max_0: CSC_MULTI_BATCH_MAX_0,
    #[doc = "0xac - "]
    pub cmac_a_desc_0: CMAC_A_DESC_0,
    #[doc = "0xb0 - "]
    pub cmac_a_cap_incompat_0: CMAC_A_CAP_INCOMPAT_0,
    #[doc = "0xb4 - "]
    pub cmac_a_cap_compat_0: CMAC_A_CAP_COMPAT_0,
    #[doc = "0xb8 - "]
    pub cmac_a_base_feature_types_0: CMAC_A_BASE_FEATURE_TYPES_0,
    #[doc = "0xbc - "]
    pub cmac_a_base_weight_types_0: CMAC_A_BASE_WEIGHT_TYPES_0,
    #[doc = "0xc0 - "]
    pub cmac_a_base_atomic_c_0: CMAC_A_BASE_ATOMIC_C_0,
    #[doc = "0xc4 - "]
    pub cmac_a_base_atomic_k_0: CMAC_A_BASE_ATOMIC_K_0,
    #[doc = "0xc8 - "]
    pub cmac_a_base_cdma_id_0: CMAC_A_BASE_CDMA_ID_0,
    #[doc = "0xcc - "]
    pub cmac_b_desc_0: CMAC_B_DESC_0,
    #[doc = "0xd0 - "]
    pub cmac_b_cap_incompat_0: CMAC_B_CAP_INCOMPAT_0,
    #[doc = "0xd4 - "]
    pub cmac_b_cap_compat_0: CMAC_B_CAP_COMPAT_0,
    #[doc = "0xd8 - "]
    pub cmac_b_base_feature_types_0: CMAC_B_BASE_FEATURE_TYPES_0,
    #[doc = "0xdc - "]
    pub cmac_b_base_weight_types_0: CMAC_B_BASE_WEIGHT_TYPES_0,
    #[doc = "0xe0 - "]
    pub cmac_b_base_atomic_c_0: CMAC_B_BASE_ATOMIC_C_0,
    #[doc = "0xe4 - "]
    pub cmac_b_base_atomic_k_0: CMAC_B_BASE_ATOMIC_K_0,
    #[doc = "0xe8 - "]
    pub cmac_b_base_cdma_id_0: CMAC_B_BASE_CDMA_ID_0,
    #[doc = "0xec - "]
    pub cacc_desc_0: CACC_DESC_0,
    #[doc = "0xf0 - "]
    pub cacc_cap_incompat_0: CACC_CAP_INCOMPAT_0,
    #[doc = "0xf4 - "]
    pub cacc_cap_compat_0: CACC_CAP_COMPAT_0,
    #[doc = "0xf8 - "]
    pub cacc_base_feature_types_0: CACC_BASE_FEATURE_TYPES_0,
    #[doc = "0xfc - "]
    pub cacc_base_weight_types_0: CACC_BASE_WEIGHT_TYPES_0,
    #[doc = "0x100 - "]
    pub cacc_base_atomic_c_0: CACC_BASE_ATOMIC_C_0,
    #[doc = "0x104 - "]
    pub cacc_base_atomic_k_0: CACC_BASE_ATOMIC_K_0,
    #[doc = "0x108 - "]
    pub cacc_base_cdma_id_0: CACC_BASE_CDMA_ID_0,
    #[doc = "0x10c - "]
    pub cacc_multi_batch_max_0: CACC_MULTI_BATCH_MAX_0,
    #[doc = "0x110 - "]
    pub sdp_rdma_desc_0: SDP_RDMA_DESC_0,
    #[doc = "0x114 - "]
    pub sdp_rdma_cap_incompat_0: SDP_RDMA_CAP_INCOMPAT_0,
    #[doc = "0x118 - "]
    pub sdp_rdma_cap_compat_0: SDP_RDMA_CAP_COMPAT_0,
    #[doc = "0x11c - "]
    pub sdp_rdma_base_atomic_m_0: SDP_RDMA_BASE_ATOMIC_M_0,
    #[doc = "0x120 - "]
    pub sdp_rdma_base_sdp_id_0: SDP_RDMA_BASE_SDP_ID_0,
    #[doc = "0x124 - "]
    pub sdp_desc_0: SDP_DESC_0,
    #[doc = "0x128 - "]
    pub sdp_cap_incompat_0: SDP_CAP_INCOMPAT_0,
    #[doc = "0x12c - "]
    pub sdp_cap_compat_0: SDP_CAP_COMPAT_0,
    #[doc = "0x130 - "]
    pub sdp_base_feature_types_0: SDP_BASE_FEATURE_TYPES_0,
    #[doc = "0x134 - "]
    pub sdp_base_weight_types_0: SDP_BASE_WEIGHT_TYPES_0,
    #[doc = "0x138 - "]
    pub sdp_base_cdma_id_0: SDP_BASE_CDMA_ID_0,
    #[doc = "0x13c - "]
    pub sdp_multi_batch_max_0: SDP_MULTI_BATCH_MAX_0,
    #[doc = "0x140 - "]
    pub sdp_bs_throughput_0: SDP_BS_THROUGHPUT_0,
    #[doc = "0x144 - "]
    pub sdp_bn_throughput_0: SDP_BN_THROUGHPUT_0,
    #[doc = "0x148 - "]
    pub sdp_ew_throughput_0: SDP_EW_THROUGHPUT_0,
    #[doc = "0x14c - "]
    pub pdp_rdma_desc_0: PDP_RDMA_DESC_0,
    #[doc = "0x150 - "]
    pub pdp_rdma_cap_incompat_0: PDP_RDMA_CAP_INCOMPAT_0,
    #[doc = "0x154 - "]
    pub pdp_rdma_cap_compat_0: PDP_RDMA_CAP_COMPAT_0,
    #[doc = "0x158 - "]
    pub pdp_rdma_base_atomic_m_0: PDP_RDMA_BASE_ATOMIC_M_0,
    #[doc = "0x15c - "]
    pub pdp_rdma_base_pdp_id_0: PDP_RDMA_BASE_PDP_ID_0,
    #[doc = "0x160 - "]
    pub pdp_desc_0: PDP_DESC_0,
    #[doc = "0x164 - "]
    pub pdp_cap_incompat_0: PDP_CAP_INCOMPAT_0,
    #[doc = "0x168 - "]
    pub pdp_cap_compat_0: PDP_CAP_COMPAT_0,
    #[doc = "0x16c - "]
    pub pdp_base_feature_types_0: PDP_BASE_FEATURE_TYPES_0,
    #[doc = "0x170 - "]
    pub pdp_base_throughput_0: PDP_BASE_THROUGHPUT_0,
    #[doc = "0x174 - "]
    pub cdp_rdma_desc_0: CDP_RDMA_DESC_0,
    #[doc = "0x178 - "]
    pub cdp_rdma_cap_incompat_0: CDP_RDMA_CAP_INCOMPAT_0,
    #[doc = "0x17c - "]
    pub cdp_rdma_cap_compat_0: CDP_RDMA_CAP_COMPAT_0,
    #[doc = "0x180 - "]
    pub cdp_rdma_base_atomic_m_0: CDP_RDMA_BASE_ATOMIC_M_0,
    #[doc = "0x184 - "]
    pub cdp_rdma_base_cdp_id_0: CDP_RDMA_BASE_CDP_ID_0,
    #[doc = "0x188 - "]
    pub cdp_desc_0: CDP_DESC_0,
    #[doc = "0x18c - "]
    pub cdp_cap_incompat_0: CDP_CAP_INCOMPAT_0,
    #[doc = "0x190 - "]
    pub cdp_cap_compat_0: CDP_CAP_COMPAT_0,
    #[doc = "0x194 - "]
    pub cdp_base_feature_types_0: CDP_BASE_FEATURE_TYPES_0,
    #[doc = "0x198 - "]
    pub cdp_base_throughput_0: CDP_BASE_THROUGHPUT_0,
    #[doc = "0x19c - "]
    pub end_of_list_0: END_OF_LIST_0,
}
#[doc = "HW_VERSION_0 (r) register accessor: an alias for `Reg<HW_VERSION_0_SPEC>`"]
pub type HW_VERSION_0 = crate::Reg<hw_version_0::HW_VERSION_0_SPEC>;
#[doc = ""]
pub mod hw_version_0;
#[doc = "GLB_DESC_0 (r) register accessor: an alias for `Reg<GLB_DESC_0_SPEC>`"]
pub type GLB_DESC_0 = crate::Reg<glb_desc_0::GLB_DESC_0_SPEC>;
#[doc = ""]
pub mod glb_desc_0;
#[doc = "CIF_DESC_0 (r) register accessor: an alias for `Reg<CIF_DESC_0_SPEC>`"]
pub type CIF_DESC_0 = crate::Reg<cif_desc_0::CIF_DESC_0_SPEC>;
#[doc = ""]
pub mod cif_desc_0;
#[doc = "CIF_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CIF_CAP_INCOMPAT_0_SPEC>`"]
pub type CIF_CAP_INCOMPAT_0 = crate::Reg<cif_cap_incompat_0::CIF_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cif_cap_incompat_0;
#[doc = "CIF_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CIF_CAP_COMPAT_0_SPEC>`"]
pub type CIF_CAP_COMPAT_0 = crate::Reg<cif_cap_compat_0::CIF_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cif_cap_compat_0;
#[doc = "CIF_BASE_WIDTH_0 (r) register accessor: an alias for `Reg<CIF_BASE_WIDTH_0_SPEC>`"]
pub type CIF_BASE_WIDTH_0 = crate::Reg<cif_base_width_0::CIF_BASE_WIDTH_0_SPEC>;
#[doc = ""]
pub mod cif_base_width_0;
#[doc = "CIF_BASE_LATENCY_0 (r) register accessor: an alias for `Reg<CIF_BASE_LATENCY_0_SPEC>`"]
pub type CIF_BASE_LATENCY_0 = crate::Reg<cif_base_latency_0::CIF_BASE_LATENCY_0_SPEC>;
#[doc = ""]
pub mod cif_base_latency_0;
#[doc = "CIF_BASE_BURST_LENGTH_MAX_0 (r) register accessor: an alias for `Reg<CIF_BASE_BURST_LENGTH_MAX_0_SPEC>`"]
pub type CIF_BASE_BURST_LENGTH_MAX_0 =
    crate::Reg<cif_base_burst_length_max_0::CIF_BASE_BURST_LENGTH_MAX_0_SPEC>;
#[doc = ""]
pub mod cif_base_burst_length_max_0;
#[doc = "CIF_BASE_MEM_ADDR_WIDTH_0 (r) register accessor: an alias for `Reg<CIF_BASE_MEM_ADDR_WIDTH_0_SPEC>`"]
pub type CIF_BASE_MEM_ADDR_WIDTH_0 =
    crate::Reg<cif_base_mem_addr_width_0::CIF_BASE_MEM_ADDR_WIDTH_0_SPEC>;
#[doc = ""]
pub mod cif_base_mem_addr_width_0;
#[doc = "CDMA_DESC_0 (r) register accessor: an alias for `Reg<CDMA_DESC_0_SPEC>`"]
pub type CDMA_DESC_0 = crate::Reg<cdma_desc_0::CDMA_DESC_0_SPEC>;
#[doc = ""]
pub mod cdma_desc_0;
#[doc = "CDMA_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CDMA_CAP_INCOMPAT_0_SPEC>`"]
pub type CDMA_CAP_INCOMPAT_0 = crate::Reg<cdma_cap_incompat_0::CDMA_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cdma_cap_incompat_0;
#[doc = "CDMA_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CDMA_CAP_COMPAT_0_SPEC>`"]
pub type CDMA_CAP_COMPAT_0 = crate::Reg<cdma_cap_compat_0::CDMA_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cdma_cap_compat_0;
#[doc = "CDMA_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<CDMA_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type CDMA_BASE_FEATURE_TYPES_0 =
    crate::Reg<cdma_base_feature_types_0::CDMA_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod cdma_base_feature_types_0;
#[doc = "CDMA_BASE_WEIGHT_TYPES_0 (r) register accessor: an alias for `Reg<CDMA_BASE_WEIGHT_TYPES_0_SPEC>`"]
pub type CDMA_BASE_WEIGHT_TYPES_0 =
    crate::Reg<cdma_base_weight_types_0::CDMA_BASE_WEIGHT_TYPES_0_SPEC>;
#[doc = ""]
pub mod cdma_base_weight_types_0;
#[doc = "CDMA_BASE_ATOMIC_C_0 (r) register accessor: an alias for `Reg<CDMA_BASE_ATOMIC_C_0_SPEC>`"]
pub type CDMA_BASE_ATOMIC_C_0 = crate::Reg<cdma_base_atomic_c_0::CDMA_BASE_ATOMIC_C_0_SPEC>;
#[doc = ""]
pub mod cdma_base_atomic_c_0;
#[doc = "CDMA_BASE_ATOMIC_K_0 (r) register accessor: an alias for `Reg<CDMA_BASE_ATOMIC_K_0_SPEC>`"]
pub type CDMA_BASE_ATOMIC_K_0 = crate::Reg<cdma_base_atomic_k_0::CDMA_BASE_ATOMIC_K_0_SPEC>;
#[doc = ""]
pub mod cdma_base_atomic_k_0;
#[doc = "CDMA_BASE_ATOMIC_M_0 (r) register accessor: an alias for `Reg<CDMA_BASE_ATOMIC_M_0_SPEC>`"]
pub type CDMA_BASE_ATOMIC_M_0 = crate::Reg<cdma_base_atomic_m_0::CDMA_BASE_ATOMIC_M_0_SPEC>;
#[doc = ""]
pub mod cdma_base_atomic_m_0;
#[doc = "CDMA_BASE_CBUF_BANK_NUM_0 (r) register accessor: an alias for `Reg<CDMA_BASE_CBUF_BANK_NUM_0_SPEC>`"]
pub type CDMA_BASE_CBUF_BANK_NUM_0 =
    crate::Reg<cdma_base_cbuf_bank_num_0::CDMA_BASE_CBUF_BANK_NUM_0_SPEC>;
#[doc = ""]
pub mod cdma_base_cbuf_bank_num_0;
#[doc = "CDMA_BASE_CBUF_BANK_WIDTH_0 (r) register accessor: an alias for `Reg<CDMA_BASE_CBUF_BANK_WIDTH_0_SPEC>`"]
pub type CDMA_BASE_CBUF_BANK_WIDTH_0 =
    crate::Reg<cdma_base_cbuf_bank_width_0::CDMA_BASE_CBUF_BANK_WIDTH_0_SPEC>;
#[doc = ""]
pub mod cdma_base_cbuf_bank_width_0;
#[doc = "CDMA_BASE_CBUF_BANK_DEPTH_0 (r) register accessor: an alias for `Reg<CDMA_BASE_CBUF_BANK_DEPTH_0_SPEC>`"]
pub type CDMA_BASE_CBUF_BANK_DEPTH_0 =
    crate::Reg<cdma_base_cbuf_bank_depth_0::CDMA_BASE_CBUF_BANK_DEPTH_0_SPEC>;
#[doc = ""]
pub mod cdma_base_cbuf_bank_depth_0;
#[doc = "CDMA_MULTI_BATCH_MAX_0 (r) register accessor: an alias for `Reg<CDMA_MULTI_BATCH_MAX_0_SPEC>`"]
pub type CDMA_MULTI_BATCH_MAX_0 = crate::Reg<cdma_multi_batch_max_0::CDMA_MULTI_BATCH_MAX_0_SPEC>;
#[doc = ""]
pub mod cdma_multi_batch_max_0;
#[doc = "CDMA_IMAGE_IN_FORMATS_PACKED_0 (r) register accessor: an alias for `Reg<CDMA_IMAGE_IN_FORMATS_PACKED_0_SPEC>`"]
pub type CDMA_IMAGE_IN_FORMATS_PACKED_0 =
    crate::Reg<cdma_image_in_formats_packed_0::CDMA_IMAGE_IN_FORMATS_PACKED_0_SPEC>;
#[doc = ""]
pub mod cdma_image_in_formats_packed_0;
#[doc = "CDMA_IMAGE_IN_FORMATS_SEMI_0 (r) register accessor: an alias for `Reg<CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>`"]
pub type CDMA_IMAGE_IN_FORMATS_SEMI_0 =
    crate::Reg<cdma_image_in_formats_semi_0::CDMA_IMAGE_IN_FORMATS_SEMI_0_SPEC>;
#[doc = ""]
pub mod cdma_image_in_formats_semi_0;
#[doc = "CBUF_DESC_0 (r) register accessor: an alias for `Reg<CBUF_DESC_0_SPEC>`"]
pub type CBUF_DESC_0 = crate::Reg<cbuf_desc_0::CBUF_DESC_0_SPEC>;
#[doc = ""]
pub mod cbuf_desc_0;
#[doc = "CBUF_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CBUF_CAP_INCOMPAT_0_SPEC>`"]
pub type CBUF_CAP_INCOMPAT_0 = crate::Reg<cbuf_cap_incompat_0::CBUF_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cbuf_cap_incompat_0;
#[doc = "CBUF_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CBUF_CAP_COMPAT_0_SPEC>`"]
pub type CBUF_CAP_COMPAT_0 = crate::Reg<cbuf_cap_compat_0::CBUF_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cbuf_cap_compat_0;
#[doc = "CBUF_BASE_CBUF_BANK_NUM_0 (r) register accessor: an alias for `Reg<CBUF_BASE_CBUF_BANK_NUM_0_SPEC>`"]
pub type CBUF_BASE_CBUF_BANK_NUM_0 =
    crate::Reg<cbuf_base_cbuf_bank_num_0::CBUF_BASE_CBUF_BANK_NUM_0_SPEC>;
#[doc = ""]
pub mod cbuf_base_cbuf_bank_num_0;
#[doc = "CBUF_BASE_CBUF_BANK_WIDTH_0 (r) register accessor: an alias for `Reg<CBUF_BASE_CBUF_BANK_WIDTH_0_SPEC>`"]
pub type CBUF_BASE_CBUF_BANK_WIDTH_0 =
    crate::Reg<cbuf_base_cbuf_bank_width_0::CBUF_BASE_CBUF_BANK_WIDTH_0_SPEC>;
#[doc = ""]
pub mod cbuf_base_cbuf_bank_width_0;
#[doc = "CBUF_BASE_CBUF_BANK_DEPTH_0 (r) register accessor: an alias for `Reg<CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>`"]
pub type CBUF_BASE_CBUF_BANK_DEPTH_0 =
    crate::Reg<cbuf_base_cbuf_bank_depth_0::CBUF_BASE_CBUF_BANK_DEPTH_0_SPEC>;
#[doc = ""]
pub mod cbuf_base_cbuf_bank_depth_0;
#[doc = "CBUF_BASE_CDMA_ID_0 (r) register accessor: an alias for `Reg<CBUF_BASE_CDMA_ID_0_SPEC>`"]
pub type CBUF_BASE_CDMA_ID_0 = crate::Reg<cbuf_base_cdma_id_0::CBUF_BASE_CDMA_ID_0_SPEC>;
#[doc = ""]
pub mod cbuf_base_cdma_id_0;
#[doc = "CSC_DESC_0 (r) register accessor: an alias for `Reg<CSC_DESC_0_SPEC>`"]
pub type CSC_DESC_0 = crate::Reg<csc_desc_0::CSC_DESC_0_SPEC>;
#[doc = ""]
pub mod csc_desc_0;
#[doc = "CSC_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CSC_CAP_INCOMPAT_0_SPEC>`"]
pub type CSC_CAP_INCOMPAT_0 = crate::Reg<csc_cap_incompat_0::CSC_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod csc_cap_incompat_0;
#[doc = "CSC_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CSC_CAP_COMPAT_0_SPEC>`"]
pub type CSC_CAP_COMPAT_0 = crate::Reg<csc_cap_compat_0::CSC_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod csc_cap_compat_0;
#[doc = "CSC_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<CSC_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type CSC_BASE_FEATURE_TYPES_0 =
    crate::Reg<csc_base_feature_types_0::CSC_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod csc_base_feature_types_0;
#[doc = "CSC_BASE_WEIGHT_TYPES_0 (r) register accessor: an alias for `Reg<CSC_BASE_WEIGHT_TYPES_0_SPEC>`"]
pub type CSC_BASE_WEIGHT_TYPES_0 =
    crate::Reg<csc_base_weight_types_0::CSC_BASE_WEIGHT_TYPES_0_SPEC>;
#[doc = ""]
pub mod csc_base_weight_types_0;
#[doc = "CSC_BASE_ATOMIC_C_0 (r) register accessor: an alias for `Reg<CSC_BASE_ATOMIC_C_0_SPEC>`"]
pub type CSC_BASE_ATOMIC_C_0 = crate::Reg<csc_base_atomic_c_0::CSC_BASE_ATOMIC_C_0_SPEC>;
#[doc = ""]
pub mod csc_base_atomic_c_0;
#[doc = "CSC_BASE_ATOMIC_K_0 (r) register accessor: an alias for `Reg<CSC_BASE_ATOMIC_K_0_SPEC>`"]
pub type CSC_BASE_ATOMIC_K_0 = crate::Reg<csc_base_atomic_k_0::CSC_BASE_ATOMIC_K_0_SPEC>;
#[doc = ""]
pub mod csc_base_atomic_k_0;
#[doc = "CSC_BASE_ATOMIC_M_0 (r) register accessor: an alias for `Reg<CSC_BASE_ATOMIC_M_0_SPEC>`"]
pub type CSC_BASE_ATOMIC_M_0 = crate::Reg<csc_base_atomic_m_0::CSC_BASE_ATOMIC_M_0_SPEC>;
#[doc = ""]
pub mod csc_base_atomic_m_0;
#[doc = "CSC_BASE_CBUF_BANK_NUM_0 (r) register accessor: an alias for `Reg<CSC_BASE_CBUF_BANK_NUM_0_SPEC>`"]
pub type CSC_BASE_CBUF_BANK_NUM_0 =
    crate::Reg<csc_base_cbuf_bank_num_0::CSC_BASE_CBUF_BANK_NUM_0_SPEC>;
#[doc = ""]
pub mod csc_base_cbuf_bank_num_0;
#[doc = "CSC_BASE_CBUF_BANK_WIDTH_0 (r) register accessor: an alias for `Reg<CSC_BASE_CBUF_BANK_WIDTH_0_SPEC>`"]
pub type CSC_BASE_CBUF_BANK_WIDTH_0 =
    crate::Reg<csc_base_cbuf_bank_width_0::CSC_BASE_CBUF_BANK_WIDTH_0_SPEC>;
#[doc = ""]
pub mod csc_base_cbuf_bank_width_0;
#[doc = "CSC_BASE_CBUF_BANK_DEPTH_0 (r) register accessor: an alias for `Reg<CSC_BASE_CBUF_BANK_DEPTH_0_SPEC>`"]
pub type CSC_BASE_CBUF_BANK_DEPTH_0 =
    crate::Reg<csc_base_cbuf_bank_depth_0::CSC_BASE_CBUF_BANK_DEPTH_0_SPEC>;
#[doc = ""]
pub mod csc_base_cbuf_bank_depth_0;
#[doc = "CSC_BASE_CDMA_ID_0 (r) register accessor: an alias for `Reg<CSC_BASE_CDMA_ID_0_SPEC>`"]
pub type CSC_BASE_CDMA_ID_0 = crate::Reg<csc_base_cdma_id_0::CSC_BASE_CDMA_ID_0_SPEC>;
#[doc = ""]
pub mod csc_base_cdma_id_0;
#[doc = "CSC_MULTI_BATCH_MAX_0 (r) register accessor: an alias for `Reg<CSC_MULTI_BATCH_MAX_0_SPEC>`"]
pub type CSC_MULTI_BATCH_MAX_0 = crate::Reg<csc_multi_batch_max_0::CSC_MULTI_BATCH_MAX_0_SPEC>;
#[doc = ""]
pub mod csc_multi_batch_max_0;
#[doc = "CMAC_A_DESC_0 (r) register accessor: an alias for `Reg<CMAC_A_DESC_0_SPEC>`"]
pub type CMAC_A_DESC_0 = crate::Reg<cmac_a_desc_0::CMAC_A_DESC_0_SPEC>;
#[doc = ""]
pub mod cmac_a_desc_0;
#[doc = "CMAC_A_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CMAC_A_CAP_INCOMPAT_0_SPEC>`"]
pub type CMAC_A_CAP_INCOMPAT_0 = crate::Reg<cmac_a_cap_incompat_0::CMAC_A_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cmac_a_cap_incompat_0;
#[doc = "CMAC_A_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CMAC_A_CAP_COMPAT_0_SPEC>`"]
pub type CMAC_A_CAP_COMPAT_0 = crate::Reg<cmac_a_cap_compat_0::CMAC_A_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cmac_a_cap_compat_0;
#[doc = "CMAC_A_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<CMAC_A_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type CMAC_A_BASE_FEATURE_TYPES_0 =
    crate::Reg<cmac_a_base_feature_types_0::CMAC_A_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod cmac_a_base_feature_types_0;
#[doc = "CMAC_A_BASE_WEIGHT_TYPES_0 (r) register accessor: an alias for `Reg<CMAC_A_BASE_WEIGHT_TYPES_0_SPEC>`"]
pub type CMAC_A_BASE_WEIGHT_TYPES_0 =
    crate::Reg<cmac_a_base_weight_types_0::CMAC_A_BASE_WEIGHT_TYPES_0_SPEC>;
#[doc = ""]
pub mod cmac_a_base_weight_types_0;
#[doc = "CMAC_A_BASE_ATOMIC_C_0 (r) register accessor: an alias for `Reg<CMAC_A_BASE_ATOMIC_C_0_SPEC>`"]
pub type CMAC_A_BASE_ATOMIC_C_0 = crate::Reg<cmac_a_base_atomic_c_0::CMAC_A_BASE_ATOMIC_C_0_SPEC>;
#[doc = ""]
pub mod cmac_a_base_atomic_c_0;
#[doc = "CMAC_A_BASE_ATOMIC_K_0 (r) register accessor: an alias for `Reg<CMAC_A_BASE_ATOMIC_K_0_SPEC>`"]
pub type CMAC_A_BASE_ATOMIC_K_0 = crate::Reg<cmac_a_base_atomic_k_0::CMAC_A_BASE_ATOMIC_K_0_SPEC>;
#[doc = ""]
pub mod cmac_a_base_atomic_k_0;
#[doc = "CMAC_A_BASE_CDMA_ID_0 (r) register accessor: an alias for `Reg<CMAC_A_BASE_CDMA_ID_0_SPEC>`"]
pub type CMAC_A_BASE_CDMA_ID_0 = crate::Reg<cmac_a_base_cdma_id_0::CMAC_A_BASE_CDMA_ID_0_SPEC>;
#[doc = ""]
pub mod cmac_a_base_cdma_id_0;
#[doc = "CMAC_B_DESC_0 (r) register accessor: an alias for `Reg<CMAC_B_DESC_0_SPEC>`"]
pub type CMAC_B_DESC_0 = crate::Reg<cmac_b_desc_0::CMAC_B_DESC_0_SPEC>;
#[doc = ""]
pub mod cmac_b_desc_0;
#[doc = "CMAC_B_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CMAC_B_CAP_INCOMPAT_0_SPEC>`"]
pub type CMAC_B_CAP_INCOMPAT_0 = crate::Reg<cmac_b_cap_incompat_0::CMAC_B_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cmac_b_cap_incompat_0;
#[doc = "CMAC_B_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CMAC_B_CAP_COMPAT_0_SPEC>`"]
pub type CMAC_B_CAP_COMPAT_0 = crate::Reg<cmac_b_cap_compat_0::CMAC_B_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cmac_b_cap_compat_0;
#[doc = "CMAC_B_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<CMAC_B_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type CMAC_B_BASE_FEATURE_TYPES_0 =
    crate::Reg<cmac_b_base_feature_types_0::CMAC_B_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod cmac_b_base_feature_types_0;
#[doc = "CMAC_B_BASE_WEIGHT_TYPES_0 (r) register accessor: an alias for `Reg<CMAC_B_BASE_WEIGHT_TYPES_0_SPEC>`"]
pub type CMAC_B_BASE_WEIGHT_TYPES_0 =
    crate::Reg<cmac_b_base_weight_types_0::CMAC_B_BASE_WEIGHT_TYPES_0_SPEC>;
#[doc = ""]
pub mod cmac_b_base_weight_types_0;
#[doc = "CMAC_B_BASE_ATOMIC_C_0 (r) register accessor: an alias for `Reg<CMAC_B_BASE_ATOMIC_C_0_SPEC>`"]
pub type CMAC_B_BASE_ATOMIC_C_0 = crate::Reg<cmac_b_base_atomic_c_0::CMAC_B_BASE_ATOMIC_C_0_SPEC>;
#[doc = ""]
pub mod cmac_b_base_atomic_c_0;
#[doc = "CMAC_B_BASE_ATOMIC_K_0 (r) register accessor: an alias for `Reg<CMAC_B_BASE_ATOMIC_K_0_SPEC>`"]
pub type CMAC_B_BASE_ATOMIC_K_0 = crate::Reg<cmac_b_base_atomic_k_0::CMAC_B_BASE_ATOMIC_K_0_SPEC>;
#[doc = ""]
pub mod cmac_b_base_atomic_k_0;
#[doc = "CMAC_B_BASE_CDMA_ID_0 (r) register accessor: an alias for `Reg<CMAC_B_BASE_CDMA_ID_0_SPEC>`"]
pub type CMAC_B_BASE_CDMA_ID_0 = crate::Reg<cmac_b_base_cdma_id_0::CMAC_B_BASE_CDMA_ID_0_SPEC>;
#[doc = ""]
pub mod cmac_b_base_cdma_id_0;
#[doc = "CACC_DESC_0 (r) register accessor: an alias for `Reg<CACC_DESC_0_SPEC>`"]
pub type CACC_DESC_0 = crate::Reg<cacc_desc_0::CACC_DESC_0_SPEC>;
#[doc = ""]
pub mod cacc_desc_0;
#[doc = "CACC_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CACC_CAP_INCOMPAT_0_SPEC>`"]
pub type CACC_CAP_INCOMPAT_0 = crate::Reg<cacc_cap_incompat_0::CACC_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cacc_cap_incompat_0;
#[doc = "CACC_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CACC_CAP_COMPAT_0_SPEC>`"]
pub type CACC_CAP_COMPAT_0 = crate::Reg<cacc_cap_compat_0::CACC_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cacc_cap_compat_0;
#[doc = "CACC_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<CACC_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type CACC_BASE_FEATURE_TYPES_0 =
    crate::Reg<cacc_base_feature_types_0::CACC_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod cacc_base_feature_types_0;
#[doc = "CACC_BASE_WEIGHT_TYPES_0 (r) register accessor: an alias for `Reg<CACC_BASE_WEIGHT_TYPES_0_SPEC>`"]
pub type CACC_BASE_WEIGHT_TYPES_0 =
    crate::Reg<cacc_base_weight_types_0::CACC_BASE_WEIGHT_TYPES_0_SPEC>;
#[doc = ""]
pub mod cacc_base_weight_types_0;
#[doc = "CACC_BASE_ATOMIC_C_0 (r) register accessor: an alias for `Reg<CACC_BASE_ATOMIC_C_0_SPEC>`"]
pub type CACC_BASE_ATOMIC_C_0 = crate::Reg<cacc_base_atomic_c_0::CACC_BASE_ATOMIC_C_0_SPEC>;
#[doc = ""]
pub mod cacc_base_atomic_c_0;
#[doc = "CACC_BASE_ATOMIC_K_0 (r) register accessor: an alias for `Reg<CACC_BASE_ATOMIC_K_0_SPEC>`"]
pub type CACC_BASE_ATOMIC_K_0 = crate::Reg<cacc_base_atomic_k_0::CACC_BASE_ATOMIC_K_0_SPEC>;
#[doc = ""]
pub mod cacc_base_atomic_k_0;
#[doc = "CACC_BASE_CDMA_ID_0 (r) register accessor: an alias for `Reg<CACC_BASE_CDMA_ID_0_SPEC>`"]
pub type CACC_BASE_CDMA_ID_0 = crate::Reg<cacc_base_cdma_id_0::CACC_BASE_CDMA_ID_0_SPEC>;
#[doc = ""]
pub mod cacc_base_cdma_id_0;
#[doc = "CACC_MULTI_BATCH_MAX_0 (r) register accessor: an alias for `Reg<CACC_MULTI_BATCH_MAX_0_SPEC>`"]
pub type CACC_MULTI_BATCH_MAX_0 = crate::Reg<cacc_multi_batch_max_0::CACC_MULTI_BATCH_MAX_0_SPEC>;
#[doc = ""]
pub mod cacc_multi_batch_max_0;
#[doc = "SDP_RDMA_DESC_0 (r) register accessor: an alias for `Reg<SDP_RDMA_DESC_0_SPEC>`"]
pub type SDP_RDMA_DESC_0 = crate::Reg<sdp_rdma_desc_0::SDP_RDMA_DESC_0_SPEC>;
#[doc = ""]
pub mod sdp_rdma_desc_0;
#[doc = "SDP_RDMA_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<SDP_RDMA_CAP_INCOMPAT_0_SPEC>`"]
pub type SDP_RDMA_CAP_INCOMPAT_0 =
    crate::Reg<sdp_rdma_cap_incompat_0::SDP_RDMA_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod sdp_rdma_cap_incompat_0;
#[doc = "SDP_RDMA_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<SDP_RDMA_CAP_COMPAT_0_SPEC>`"]
pub type SDP_RDMA_CAP_COMPAT_0 = crate::Reg<sdp_rdma_cap_compat_0::SDP_RDMA_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod sdp_rdma_cap_compat_0;
#[doc = "SDP_RDMA_BASE_ATOMIC_M_0 (r) register accessor: an alias for `Reg<SDP_RDMA_BASE_ATOMIC_M_0_SPEC>`"]
pub type SDP_RDMA_BASE_ATOMIC_M_0 =
    crate::Reg<sdp_rdma_base_atomic_m_0::SDP_RDMA_BASE_ATOMIC_M_0_SPEC>;
#[doc = ""]
pub mod sdp_rdma_base_atomic_m_0;
#[doc = "SDP_RDMA_BASE_SDP_ID_0 (r) register accessor: an alias for `Reg<SDP_RDMA_BASE_SDP_ID_0_SPEC>`"]
pub type SDP_RDMA_BASE_SDP_ID_0 = crate::Reg<sdp_rdma_base_sdp_id_0::SDP_RDMA_BASE_SDP_ID_0_SPEC>;
#[doc = ""]
pub mod sdp_rdma_base_sdp_id_0;
#[doc = "SDP_DESC_0 (r) register accessor: an alias for `Reg<SDP_DESC_0_SPEC>`"]
pub type SDP_DESC_0 = crate::Reg<sdp_desc_0::SDP_DESC_0_SPEC>;
#[doc = ""]
pub mod sdp_desc_0;
#[doc = "SDP_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<SDP_CAP_INCOMPAT_0_SPEC>`"]
pub type SDP_CAP_INCOMPAT_0 = crate::Reg<sdp_cap_incompat_0::SDP_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod sdp_cap_incompat_0;
#[doc = "SDP_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<SDP_CAP_COMPAT_0_SPEC>`"]
pub type SDP_CAP_COMPAT_0 = crate::Reg<sdp_cap_compat_0::SDP_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod sdp_cap_compat_0;
#[doc = "SDP_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<SDP_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type SDP_BASE_FEATURE_TYPES_0 =
    crate::Reg<sdp_base_feature_types_0::SDP_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod sdp_base_feature_types_0;
#[doc = "SDP_BASE_WEIGHT_TYPES_0 (r) register accessor: an alias for `Reg<SDP_BASE_WEIGHT_TYPES_0_SPEC>`"]
pub type SDP_BASE_WEIGHT_TYPES_0 =
    crate::Reg<sdp_base_weight_types_0::SDP_BASE_WEIGHT_TYPES_0_SPEC>;
#[doc = ""]
pub mod sdp_base_weight_types_0;
#[doc = "SDP_BASE_CDMA_ID_0 (r) register accessor: an alias for `Reg<SDP_BASE_CDMA_ID_0_SPEC>`"]
pub type SDP_BASE_CDMA_ID_0 = crate::Reg<sdp_base_cdma_id_0::SDP_BASE_CDMA_ID_0_SPEC>;
#[doc = ""]
pub mod sdp_base_cdma_id_0;
#[doc = "SDP_MULTI_BATCH_MAX_0 (r) register accessor: an alias for `Reg<SDP_MULTI_BATCH_MAX_0_SPEC>`"]
pub type SDP_MULTI_BATCH_MAX_0 = crate::Reg<sdp_multi_batch_max_0::SDP_MULTI_BATCH_MAX_0_SPEC>;
#[doc = ""]
pub mod sdp_multi_batch_max_0;
#[doc = "SDP_BS_THROUGHPUT_0 (r) register accessor: an alias for `Reg<SDP_BS_THROUGHPUT_0_SPEC>`"]
pub type SDP_BS_THROUGHPUT_0 = crate::Reg<sdp_bs_throughput_0::SDP_BS_THROUGHPUT_0_SPEC>;
#[doc = ""]
pub mod sdp_bs_throughput_0;
#[doc = "SDP_BN_THROUGHPUT_0 (r) register accessor: an alias for `Reg<SDP_BN_THROUGHPUT_0_SPEC>`"]
pub type SDP_BN_THROUGHPUT_0 = crate::Reg<sdp_bn_throughput_0::SDP_BN_THROUGHPUT_0_SPEC>;
#[doc = ""]
pub mod sdp_bn_throughput_0;
#[doc = "SDP_EW_THROUGHPUT_0 (r) register accessor: an alias for `Reg<SDP_EW_THROUGHPUT_0_SPEC>`"]
pub type SDP_EW_THROUGHPUT_0 = crate::Reg<sdp_ew_throughput_0::SDP_EW_THROUGHPUT_0_SPEC>;
#[doc = ""]
pub mod sdp_ew_throughput_0;
#[doc = "PDP_RDMA_DESC_0 (r) register accessor: an alias for `Reg<PDP_RDMA_DESC_0_SPEC>`"]
pub type PDP_RDMA_DESC_0 = crate::Reg<pdp_rdma_desc_0::PDP_RDMA_DESC_0_SPEC>;
#[doc = ""]
pub mod pdp_rdma_desc_0;
#[doc = "PDP_RDMA_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<PDP_RDMA_CAP_INCOMPAT_0_SPEC>`"]
pub type PDP_RDMA_CAP_INCOMPAT_0 =
    crate::Reg<pdp_rdma_cap_incompat_0::PDP_RDMA_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod pdp_rdma_cap_incompat_0;
#[doc = "PDP_RDMA_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<PDP_RDMA_CAP_COMPAT_0_SPEC>`"]
pub type PDP_RDMA_CAP_COMPAT_0 = crate::Reg<pdp_rdma_cap_compat_0::PDP_RDMA_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod pdp_rdma_cap_compat_0;
#[doc = "PDP_RDMA_BASE_ATOMIC_M_0 (r) register accessor: an alias for `Reg<PDP_RDMA_BASE_ATOMIC_M_0_SPEC>`"]
pub type PDP_RDMA_BASE_ATOMIC_M_0 =
    crate::Reg<pdp_rdma_base_atomic_m_0::PDP_RDMA_BASE_ATOMIC_M_0_SPEC>;
#[doc = ""]
pub mod pdp_rdma_base_atomic_m_0;
#[doc = "PDP_RDMA_BASE_PDP_ID_0 (r) register accessor: an alias for `Reg<PDP_RDMA_BASE_PDP_ID_0_SPEC>`"]
pub type PDP_RDMA_BASE_PDP_ID_0 = crate::Reg<pdp_rdma_base_pdp_id_0::PDP_RDMA_BASE_PDP_ID_0_SPEC>;
#[doc = ""]
pub mod pdp_rdma_base_pdp_id_0;
#[doc = "PDP_DESC_0 (r) register accessor: an alias for `Reg<PDP_DESC_0_SPEC>`"]
pub type PDP_DESC_0 = crate::Reg<pdp_desc_0::PDP_DESC_0_SPEC>;
#[doc = ""]
pub mod pdp_desc_0;
#[doc = "PDP_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<PDP_CAP_INCOMPAT_0_SPEC>`"]
pub type PDP_CAP_INCOMPAT_0 = crate::Reg<pdp_cap_incompat_0::PDP_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod pdp_cap_incompat_0;
#[doc = "PDP_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<PDP_CAP_COMPAT_0_SPEC>`"]
pub type PDP_CAP_COMPAT_0 = crate::Reg<pdp_cap_compat_0::PDP_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod pdp_cap_compat_0;
#[doc = "PDP_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<PDP_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type PDP_BASE_FEATURE_TYPES_0 =
    crate::Reg<pdp_base_feature_types_0::PDP_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod pdp_base_feature_types_0;
#[doc = "PDP_BASE_THROUGHPUT_0 (r) register accessor: an alias for `Reg<PDP_BASE_THROUGHPUT_0_SPEC>`"]
pub type PDP_BASE_THROUGHPUT_0 = crate::Reg<pdp_base_throughput_0::PDP_BASE_THROUGHPUT_0_SPEC>;
#[doc = ""]
pub mod pdp_base_throughput_0;
#[doc = "CDP_RDMA_DESC_0 (r) register accessor: an alias for `Reg<CDP_RDMA_DESC_0_SPEC>`"]
pub type CDP_RDMA_DESC_0 = crate::Reg<cdp_rdma_desc_0::CDP_RDMA_DESC_0_SPEC>;
#[doc = ""]
pub mod cdp_rdma_desc_0;
#[doc = "CDP_RDMA_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CDP_RDMA_CAP_INCOMPAT_0_SPEC>`"]
pub type CDP_RDMA_CAP_INCOMPAT_0 =
    crate::Reg<cdp_rdma_cap_incompat_0::CDP_RDMA_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cdp_rdma_cap_incompat_0;
#[doc = "CDP_RDMA_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CDP_RDMA_CAP_COMPAT_0_SPEC>`"]
pub type CDP_RDMA_CAP_COMPAT_0 = crate::Reg<cdp_rdma_cap_compat_0::CDP_RDMA_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cdp_rdma_cap_compat_0;
#[doc = "CDP_RDMA_BASE_ATOMIC_M_0 (r) register accessor: an alias for `Reg<CDP_RDMA_BASE_ATOMIC_M_0_SPEC>`"]
pub type CDP_RDMA_BASE_ATOMIC_M_0 =
    crate::Reg<cdp_rdma_base_atomic_m_0::CDP_RDMA_BASE_ATOMIC_M_0_SPEC>;
#[doc = ""]
pub mod cdp_rdma_base_atomic_m_0;
#[doc = "CDP_RDMA_BASE_CDP_ID_0 (r) register accessor: an alias for `Reg<CDP_RDMA_BASE_CDP_ID_0_SPEC>`"]
pub type CDP_RDMA_BASE_CDP_ID_0 = crate::Reg<cdp_rdma_base_cdp_id_0::CDP_RDMA_BASE_CDP_ID_0_SPEC>;
#[doc = ""]
pub mod cdp_rdma_base_cdp_id_0;
#[doc = "CDP_DESC_0 (r) register accessor: an alias for `Reg<CDP_DESC_0_SPEC>`"]
pub type CDP_DESC_0 = crate::Reg<cdp_desc_0::CDP_DESC_0_SPEC>;
#[doc = ""]
pub mod cdp_desc_0;
#[doc = "CDP_CAP_INCOMPAT_0 (r) register accessor: an alias for `Reg<CDP_CAP_INCOMPAT_0_SPEC>`"]
pub type CDP_CAP_INCOMPAT_0 = crate::Reg<cdp_cap_incompat_0::CDP_CAP_INCOMPAT_0_SPEC>;
#[doc = ""]
pub mod cdp_cap_incompat_0;
#[doc = "CDP_CAP_COMPAT_0 (r) register accessor: an alias for `Reg<CDP_CAP_COMPAT_0_SPEC>`"]
pub type CDP_CAP_COMPAT_0 = crate::Reg<cdp_cap_compat_0::CDP_CAP_COMPAT_0_SPEC>;
#[doc = ""]
pub mod cdp_cap_compat_0;
#[doc = "CDP_BASE_FEATURE_TYPES_0 (r) register accessor: an alias for `Reg<CDP_BASE_FEATURE_TYPES_0_SPEC>`"]
pub type CDP_BASE_FEATURE_TYPES_0 =
    crate::Reg<cdp_base_feature_types_0::CDP_BASE_FEATURE_TYPES_0_SPEC>;
#[doc = ""]
pub mod cdp_base_feature_types_0;
#[doc = "CDP_BASE_THROUGHPUT_0 (r) register accessor: an alias for `Reg<CDP_BASE_THROUGHPUT_0_SPEC>`"]
pub type CDP_BASE_THROUGHPUT_0 = crate::Reg<cdp_base_throughput_0::CDP_BASE_THROUGHPUT_0_SPEC>;
#[doc = ""]
pub mod cdp_base_throughput_0;
#[doc = "END_OF_LIST_0 (r) register accessor: an alias for `Reg<END_OF_LIST_0_SPEC>`"]
pub type END_OF_LIST_0 = crate::Reg<end_of_list_0::END_OF_LIST_0_SPEC>;
#[doc = ""]
pub mod end_of_list_0;
