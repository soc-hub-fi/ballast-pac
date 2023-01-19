#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0020_0000],
    #[doc = "0x200000..0x20c000 - CLINT"]
    pub clint: CLINT,
    _reserved1: [u8; 0x4000],
    #[doc = "0x210000..0x21001c - Timer"]
    pub timer: TIMER,
    _reserved2: [u8; 0xffe4],
    #[doc = "0x220000..0x220028 - l2_config"]
    pub l2_config: L2_CONFIG,
    _reserved3: [u8; 0xffd8],
    #[doc = "0x230000..0x230228 - cluster_config"]
    pub cluster_config: CLUSTER_CONFIG,
    _reserved4: [u8; 0x00dc_fdd8],
    #[doc = "0x1000000..0x1200020 - PLIC"]
    pub plic: PLIC,
}
#[doc = "CLINT"]
pub use self::clint::CLINT;
#[doc = r"Cluster"]
#[doc = "CLINT"]
pub mod clint;
#[doc = "Timer"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Timer"]
pub mod timer;
#[doc = "l2_config"]
pub use self::l2_config::L2_CONFIG;
#[doc = r"Cluster"]
#[doc = "l2_config"]
pub mod l2_config;
#[doc = "cluster_config"]
pub use self::cluster_config::CLUSTER_CONFIG;
#[doc = r"Cluster"]
#[doc = "cluster_config"]
pub mod cluster_config;
#[doc = "PLIC"]
pub use self::plic::PLIC;
#[doc = r"Cluster"]
#[doc = "PLIC"]
pub mod plic;
