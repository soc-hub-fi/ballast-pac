#[doc = r"Register block"]
#[repr(C)]
pub struct SDIO {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - "]
    pub cmd_op: CMD_OP,
    #[doc = "0x24 - "]
    pub cmd_arg: CMD_ARG,
    #[doc = "0x28 - "]
    pub data_setup: DATA_SETUP,
    #[doc = "0x2c - "]
    pub start: START,
    #[doc = "0x30 - "]
    pub rsp0: RSP0,
    #[doc = "0x34 - "]
    pub rsp1: RSP1,
    #[doc = "0x38 - "]
    pub rsp2: RSP2,
    #[doc = "0x3c - "]
    pub rsp3: RSP3,
    #[doc = "0x40 - "]
    pub clk_div_0: CLK_DIV_0,
    #[doc = "0x44 - "]
    pub status: STATUS,
    #[doc = "0x48 - Card Identification Word0"]
    pub cid0: CID0,
    #[doc = "0x4c - Card Identification Word 1"]
    pub cid1: CID1,
    #[doc = "0x50 - Card Identification Word 2"]
    pub cid2: CID2,
    #[doc = "0x54 - Card Identification Word 3"]
    pub cid3: CID3,
    #[doc = "0x58 - "]
    pub rca: RCA,
    #[doc = "0x5c - "]
    pub stop: STOP,
}
#[doc = "CMD_OP (w) register accessor: an alias for `Reg<CMD_OP_SPEC>`"]
pub type CMD_OP = crate::Reg<cmd_op::CMD_OP_SPEC>;
#[doc = ""]
pub mod cmd_op;
#[doc = "CMD_ARG (w) register accessor: an alias for `Reg<CMD_ARG_SPEC>`"]
pub type CMD_ARG = crate::Reg<cmd_arg::CMD_ARG_SPEC>;
#[doc = ""]
pub mod cmd_arg;
#[doc = "DATA_SETUP (w) register accessor: an alias for `Reg<DATA_SETUP_SPEC>`"]
pub type DATA_SETUP = crate::Reg<data_setup::DATA_SETUP_SPEC>;
#[doc = ""]
pub mod data_setup;
#[doc = "START (w) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "STOP (w) register accessor: an alias for `Reg<STOP_SPEC>`"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = ""]
pub mod stop;
#[doc = "RSP0 (r) register accessor: an alias for `Reg<RSP0_SPEC>`"]
pub type RSP0 = crate::Reg<rsp0::RSP0_SPEC>;
#[doc = ""]
pub mod rsp0;
#[doc = "CID3 (r) register accessor: an alias for `Reg<CID3_SPEC>`"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Card Identification Word 3"]
pub mod cid3;
#[doc = "CID1 (r) register accessor: an alias for `Reg<CID1_SPEC>`"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Card Identification Word 1"]
pub mod cid1;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "RSP3 (r) register accessor: an alias for `Reg<RSP3_SPEC>`"]
pub type RSP3 = crate::Reg<rsp3::RSP3_SPEC>;
#[doc = ""]
pub mod rsp3;
#[doc = "RSP1 (r) register accessor: an alias for `Reg<RSP1_SPEC>`"]
pub type RSP1 = crate::Reg<rsp1::RSP1_SPEC>;
#[doc = ""]
pub mod rsp1;
#[doc = "RCA (r) register accessor: an alias for `Reg<RCA_SPEC>`"]
pub type RCA = crate::Reg<rca::RCA_SPEC>;
#[doc = ""]
pub mod rca;
#[doc = "CID2 (r) register accessor: an alias for `Reg<CID2_SPEC>`"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Card Identification Word 2"]
pub mod cid2;
#[doc = "CID0 (r) register accessor: an alias for `Reg<CID0_SPEC>`"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Card Identification Word0"]
pub mod cid0;
#[doc = "CLK_DIV_0 (rw) register accessor: an alias for `Reg<CLK_DIV_0_SPEC>`"]
pub type CLK_DIV_0 = crate::Reg<clk_div_0::CLK_DIV_0_SPEC>;
#[doc = ""]
pub mod clk_div_0;
#[doc = "RSP2 (r) register accessor: an alias for `Reg<RSP2_SPEC>`"]
pub type RSP2 = crate::Reg<rsp2::RSP2_SPEC>;
#[doc = ""]
pub mod rsp2;
