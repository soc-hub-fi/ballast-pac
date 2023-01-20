#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0a - Apb_software_irq"]
    pub apb_software_irq: APB_SOFTWARE_IRQ,
    _reserved1: [u8; 0x01f6],
    #[doc = "0x200..0x24d - Global_intr_router"]
    pub global_intr_router: GLOBAL_INTR_ROUTER,
    _reserved2: [u8; 0x01b3],
    #[doc = "0x400..0x410 - C2C_remap"]
    pub c2c_remap: C2C_REMAP,
    _reserved3: [u8; 0x01f0],
    #[doc = "0x600..0x641 - Pad_config"]
    pub pad_config: PAD_CONFIG,
    _reserved4: [u8; 0x01bf],
    #[doc = "0x800..0x81a - EMA"]
    pub ema: EMA,
}
#[doc = "Apb_software_irq"]
pub use self::apb_software_irq::APB_SOFTWARE_IRQ;
#[doc = r"Cluster"]
#[doc = "Apb_software_irq"]
pub mod apb_software_irq;
#[doc = "Global_intr_router"]
pub use self::global_intr_router::GLOBAL_INTR_ROUTER;
#[doc = r"Cluster"]
#[doc = "Global_intr_router"]
pub mod global_intr_router;
#[doc = "C2C_remap"]
pub use self::c2c_remap::C2C_REMAP;
#[doc = r"Cluster"]
#[doc = "C2C_remap"]
pub mod c2c_remap;
#[doc = "Pad_config"]
pub use self::pad_config::PAD_CONFIG;
#[doc = r"Cluster"]
#[doc = "Pad_config"]
pub mod pad_config;
#[doc = "EMA"]
pub use self::ema::EMA;
#[doc = r"Cluster"]
#[doc = "EMA"]
pub mod ema;
