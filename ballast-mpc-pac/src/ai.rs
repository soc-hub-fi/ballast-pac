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
#[doc = "GLB"]
pub use self::glb::GLB;
#[doc = r"Cluster"]
#[doc = "GLB"]
pub mod glb;
#[doc = "MCIF"]
pub use self::mcif::MCIF;
#[doc = r"Cluster"]
#[doc = "MCIF"]
pub mod mcif;
#[doc = "CDMA"]
pub use self::cdma::CDMA;
#[doc = r"Cluster"]
#[doc = "CDMA"]
pub mod cdma;
#[doc = "CSC"]
pub use self::csc::CSC;
#[doc = r"Cluster"]
#[doc = "CSC"]
pub mod csc;
#[doc = "CMAC_A"]
pub use self::cmac_a::CMAC_A;
#[doc = r"Cluster"]
#[doc = "CMAC_A"]
pub mod cmac_a;
#[doc = "CMAC_B"]
pub use self::cmac_b::CMAC_B;
#[doc = r"Cluster"]
#[doc = "CMAC_B"]
pub mod cmac_b;
#[doc = "CACC"]
pub use self::cacc::CACC;
#[doc = r"Cluster"]
#[doc = "CACC"]
pub mod cacc;
#[doc = "SDP_RDMA"]
pub use self::sdp_rdma::SDP_RDMA;
#[doc = r"Cluster"]
#[doc = "SDP_RDMA"]
pub mod sdp_rdma;
#[doc = "SDP"]
pub use self::sdp::SDP;
#[doc = r"Cluster"]
#[doc = "SDP"]
pub mod sdp;
#[doc = "PDP_RDMA"]
pub use self::pdp_rdma::PDP_RDMA;
#[doc = r"Cluster"]
#[doc = "PDP_RDMA"]
pub mod pdp_rdma;
#[doc = "PDP"]
pub use self::pdp::PDP;
#[doc = r"Cluster"]
#[doc = "PDP"]
pub mod pdp;
#[doc = "CFGROM"]
pub use self::cfgrom::CFGROM;
#[doc = r"Cluster"]
#[doc = "CFGROM"]
pub mod cfgrom;
