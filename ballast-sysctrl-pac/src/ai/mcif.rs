#[doc = r"Register block"]
#[repr(C)]
pub struct MCIF {
    #[doc = "0x00 - Register0 to control the read weight of clients in MCIF"]
    pub cfg_rd_weight_0: CFG_RD_WEIGHT_0,
    #[doc = "0x04 - Register1 to control the read weight of clients in MCIF"]
    pub cfg_rd_weight_1: CFG_RD_WEIGHT_1,
    #[doc = "0x08 - Register2 to control the read weight of clients in MCIF"]
    pub cfg_rd_weight_2: CFG_RD_WEIGHT_2,
    #[doc = "0x0c - Register0 to control the write weight of clients in MCIF"]
    pub cfg_wr_weight_0: CFG_WR_WEIGHT_0,
    #[doc = "0x10 - Register1 to control the write weight of clients in MCIF"]
    pub cfg_wr_weight_1: CFG_WR_WEIGHT_1,
    #[doc = "0x14 - Outstanding AXI transactions in unit of 64Byte"]
    pub cfg_outstanding_cnt: CFG_OUTSTANDING_CNT,
    #[doc = "0x18 - Idle status of MCIF"]
    pub status: STATUS,
}
#[doc = "CFG_RD_WEIGHT_0 (rw) register accessor: an alias for `Reg<CFG_RD_WEIGHT_0_SPEC>`"]
pub type CFG_RD_WEIGHT_0 = crate::Reg<cfg_rd_weight_0::CFG_RD_WEIGHT_0_SPEC>;
#[doc = "Register0 to control the read weight of clients in MCIF"]
pub mod cfg_rd_weight_0;
#[doc = "CFG_RD_WEIGHT_1 (rw) register accessor: an alias for `Reg<CFG_RD_WEIGHT_1_SPEC>`"]
pub type CFG_RD_WEIGHT_1 = crate::Reg<cfg_rd_weight_1::CFG_RD_WEIGHT_1_SPEC>;
#[doc = "Register1 to control the read weight of clients in MCIF"]
pub mod cfg_rd_weight_1;
#[doc = "CFG_RD_WEIGHT_2 (rw) register accessor: an alias for `Reg<CFG_RD_WEIGHT_2_SPEC>`"]
pub type CFG_RD_WEIGHT_2 = crate::Reg<cfg_rd_weight_2::CFG_RD_WEIGHT_2_SPEC>;
#[doc = "Register2 to control the read weight of clients in MCIF"]
pub mod cfg_rd_weight_2;
#[doc = "CFG_WR_WEIGHT_0 (rw) register accessor: an alias for `Reg<CFG_WR_WEIGHT_0_SPEC>`"]
pub type CFG_WR_WEIGHT_0 = crate::Reg<cfg_wr_weight_0::CFG_WR_WEIGHT_0_SPEC>;
#[doc = "Register0 to control the write weight of clients in MCIF"]
pub mod cfg_wr_weight_0;
#[doc = "CFG_WR_WEIGHT_1 (rw) register accessor: an alias for `Reg<CFG_WR_WEIGHT_1_SPEC>`"]
pub type CFG_WR_WEIGHT_1 = crate::Reg<cfg_wr_weight_1::CFG_WR_WEIGHT_1_SPEC>;
#[doc = "Register1 to control the write weight of clients in MCIF"]
pub mod cfg_wr_weight_1;
#[doc = "CFG_OUTSTANDING_CNT (rw) register accessor: an alias for `Reg<CFG_OUTSTANDING_CNT_SPEC>`"]
pub type CFG_OUTSTANDING_CNT = crate::Reg<cfg_outstanding_cnt::CFG_OUTSTANDING_CNT_SPEC>;
#[doc = "Outstanding AXI transactions in unit of 64Byte"]
pub mod cfg_outstanding_cnt;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Idle status of MCIF"]
pub mod status;
