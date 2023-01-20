#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0010_1000],
    #[doc = "0x101000..0x101070 - GPIO"]
    pub gpio: GPIO,
    _reserved1: [u8; 0x0f90],
    #[doc = "0x102000..0x102464 - UDMA"]
    pub udma: UDMA,
    _reserved2: [u8; 0x1b9c],
    #[doc = "0x104000..0x1040cc - SocControl"]
    pub soc_control: SOC_CONTROL,
    _reserved3: [u8; 0x0f34],
    #[doc = "0x105000..0x105108 - AdvancedTimer"]
    pub advanced_timer: ADVANCED_TIMER,
    _reserved4: [u8; 0x0ef8],
    #[doc = "0x106000..0x10608c - SocEventGenerator"]
    pub soc_event_generator: SOC_EVENT_GENERATOR,
    _reserved5: [u8; 0x2f74],
    #[doc = "0x109000..0x109028 - EventInterruptUnit"]
    pub event_interrupt_unit: EVENT_INTERRUPT_UNIT,
    _reserved6: [u8; 0x1fd8],
    #[doc = "0x10b000..0x10b028 - Timer"]
    pub timer: TIMER,
}
#[doc = "GPIO"]
pub use self::gpio::GPIO;
#[doc = r"Cluster"]
#[doc = "GPIO"]
pub mod gpio;
#[doc = "SocControl"]
pub use self::soc_control::SOC_CONTROL;
#[doc = r"Cluster"]
#[doc = "SocControl"]
pub mod soc_control;
#[doc = "EventInterruptUnit"]
pub use self::event_interrupt_unit::EVENT_INTERRUPT_UNIT;
#[doc = r"Cluster"]
#[doc = "EventInterruptUnit"]
pub mod event_interrupt_unit;
#[doc = "SocEventGenerator"]
pub use self::soc_event_generator::SOC_EVENT_GENERATOR;
#[doc = r"Cluster"]
#[doc = "SocEventGenerator"]
pub mod soc_event_generator;
#[doc = "Timer"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "Timer"]
pub mod timer;
#[doc = "AdvancedTimer"]
pub use self::advanced_timer::ADVANCED_TIMER;
#[doc = r"Cluster"]
#[doc = "AdvancedTimer"]
pub mod advanced_timer;
#[doc = "UDMA"]
pub use self::udma::UDMA;
#[doc = r"Cluster"]
#[doc = "UDMA"]
pub mod udma;
