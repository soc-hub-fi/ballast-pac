#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0010_1000],
    #[doc = "0x101000..0x101038 - GPIO"]
    pub gpio: GPIO,
    _reserved1: [u8; 0x0fc8],
    #[doc = "0x102000..0x102464 - UDMA"]
    pub udma: UDMA,
    _reserved2: [u8; 0x1b9c],
    #[doc = "0x104000..0x1040d0 - SocControl"]
    pub soc_control: SOC_CONTROL,
    #[doc = "0x1040d0..0x1040f0 - ETH_PLL"]
    pub eth_pll: ETH_PLL,
    #[doc = "0x1040f0..0x104110 - AI_PLL"]
    pub ai_pll: AI_PLL,
    #[doc = "0x104110..0x104130 - HPC_PLL"]
    pub hpc_pll: HPC_PLL,
    #[doc = "0x104130..0x104150 - PULP_PLL"]
    pub pulp_pll: PULP_PLL,
    #[doc = "0x104150..0x104170 - INTER_PLL"]
    pub inter_pll: INTER_PLL,
    #[doc = "0x104170..0x104190 - C2C_PLL"]
    pub c2c_pll: C2C_PLL,
    _reserved9: [u8; 0x20],
    #[doc = "0x1041b0..0x1041d0 - TOPPERIPH_PLL"]
    pub topperiph_pll: TOPPERIPH_PLL,
    #[doc = "0x1041d0..0x1041d8 - BootConfig"]
    pub boot_config: BOOT_CONFIG,
    _reserved11: [u8; 0x0e28],
    #[doc = "0x105000..0x105108 - AdvancedTimer"]
    pub advanced_timer: ADVANCED_TIMER,
    _reserved12: [u8; 0x0ef8],
    #[doc = "0x106000..0x10608c - SocEventGenerator"]
    pub soc_event_generator: SOC_EVENT_GENERATOR,
    _reserved13: [u8; 0x2f74],
    #[doc = "0x109000..0x109028 - EventInterruptUnit"]
    pub event_interrupt_unit: EVENT_INTERRUPT_UNIT,
    _reserved14: [u8; 0x1fd8],
    #[doc = "0x10b000..0x10b028 - Timer"]
    pub timer: TIMER,
    _reserved15: [u8; 0x0001_4fd8],
    #[doc = "0x120000..0x120060 - SDIO"]
    pub sdio: SDIO,
}
#[doc = "GPIO"]
pub use self::gpio::GPIO;
#[doc = r"Cluster"]
#[doc = "GPIO"]
pub mod gpio;
#[doc = "UDMA"]
pub use self::udma::UDMA;
#[doc = r"Cluster"]
#[doc = "UDMA"]
pub mod udma;
#[doc = "AdvancedTimer"]
pub use self::advanced_timer::ADVANCED_TIMER;
#[doc = r"Cluster"]
#[doc = "AdvancedTimer"]
pub mod advanced_timer;
#[doc = "SocEventGenerator"]
pub use self::soc_event_generator::SOC_EVENT_GENERATOR;
#[doc = r"Cluster"]
#[doc = "SocEventGenerator"]
pub mod soc_event_generator;
#[doc = "EventInterruptUnit"]
pub use self::event_interrupt_unit::EVENT_INTERRUPT_UNIT;
#[doc = r"Cluster"]
#[doc = "EventInterruptUnit"]
pub mod event_interrupt_unit;
#[doc = "Timer"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Timer"]
pub mod timer;
#[doc = "SDIO"]
pub use self::sdio::SDIO;
#[doc = r"Cluster"]
#[doc = "SDIO"]
pub mod sdio;
#[doc = "SocControl"]
pub use self::soc_control::SOC_CONTROL;
#[doc = r"Cluster"]
#[doc = "SocControl"]
pub mod soc_control;
#[doc = "ETH_PLL"]
pub use self::eth_pll::ETH_PLL;
#[doc = r"Cluster"]
#[doc = "ETH_PLL"]
pub mod eth_pll;
#[doc = "AI_PLL"]
pub use self::ai_pll::AI_PLL;
#[doc = r"Cluster"]
#[doc = "AI_PLL"]
pub mod ai_pll;
#[doc = "HPC_PLL"]
pub use self::hpc_pll::HPC_PLL;
#[doc = r"Cluster"]
#[doc = "HPC_PLL"]
pub mod hpc_pll;
#[doc = "PULP_PLL"]
pub use self::pulp_pll::PULP_PLL;
#[doc = r"Cluster"]
#[doc = "PULP_PLL"]
pub mod pulp_pll;
#[doc = "INTER_PLL"]
pub use self::inter_pll::INTER_PLL;
#[doc = r"Cluster"]
#[doc = "INTER_PLL"]
pub mod inter_pll;
#[doc = "C2C_PLL"]
pub use self::c2c_pll::C2C_PLL;
#[doc = r"Cluster"]
#[doc = "C2C_PLL"]
pub mod c2c_pll;
#[doc = "TOPPERIPH_PLL"]
pub use self::topperiph_pll::TOPPERIPH_PLL;
#[doc = r"Cluster"]
#[doc = "TOPPERIPH_PLL"]
pub mod topperiph_pll;
#[doc = "BootConfig"]
pub use self::boot_config::BOOT_CONFIG;
#[doc = r"Cluster"]
#[doc = "BootConfig"]
pub mod boot_config;
