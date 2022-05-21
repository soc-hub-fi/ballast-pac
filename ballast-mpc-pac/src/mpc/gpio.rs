#[doc = "PADDIR register accessor: an alias for `Reg<PADDIR_SPEC>`"]
pub type PADDIR = crate::Reg<paddir::PADDIR_SPEC>;
#[doc = "Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
pub mod paddir;
#[doc = "PADIN register accessor: an alias for `Reg<PADIN_SPEC>`"]
pub type PADIN = crate::Reg<padin::PADIN_SPEC>;
#[doc = "Input Values"]
pub mod padin;
#[doc = "PADOUT register accessor: an alias for `Reg<PADOUT_SPEC>`"]
pub type PADOUT = crate::Reg<padout::PADOUT_SPEC>;
#[doc = "Output values."]
pub mod padout;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior. There are four triggers available - INTTYPE0 = 0, INTTYPE1 = 0: Level 1 - INTTYPE0 = 1, INTTYPE1 = 0: Level 0 - INTTYPE0 = 0, INTTYPE1 = 1: Rise - INTTYPE0 = 1, INTTYPE1 = 1: Fall"]
pub mod inten;
#[doc = "INTTYPE0 register accessor: an alias for `Reg<INTTYPE0_SPEC>`"]
pub type INTTYPE0 = crate::Reg<inttype0::INTTYPE0_SPEC>;
#[doc = "Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
pub mod inttype0;
#[doc = "INTTYPE1 register accessor: an alias for `Reg<INTTYPE1_SPEC>`"]
pub type INTTYPE1 = crate::Reg<inttype1::INTTYPE1_SPEC>;
#[doc = "Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
pub mod inttype1;
#[doc = "INTSTATUS register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
pub mod intstatus;
#[doc = "GPIOEN register accessor: an alias for `Reg<GPIOEN_SPEC>`"]
pub type GPIOEN = crate::Reg<gpioen::GPIOEN_SPEC>;
#[doc = "Contains the enable bit per GPIO line."]
pub mod gpioen;
#[doc = "PADCFG0 register accessor: an alias for `Reg<PADCFG0_SPEC>`"]
pub type PADCFG0 = crate::Reg<padcfg0::PADCFG0_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg0;
#[doc = "PADCFG1 register accessor: an alias for `Reg<PADCFG1_SPEC>`"]
pub type PADCFG1 = crate::Reg<padcfg1::PADCFG1_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg1;
#[doc = "PADCFG2 register accessor: an alias for `Reg<PADCFG2_SPEC>`"]
pub type PADCFG2 = crate::Reg<padcfg2::PADCFG2_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg2;
#[doc = "PADCFG3 register accessor: an alias for `Reg<PADCFG3_SPEC>`"]
pub type PADCFG3 = crate::Reg<padcfg3::PADCFG3_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg3;
#[doc = "PADCFG4 register accessor: an alias for `Reg<PADCFG4_SPEC>`"]
pub type PADCFG4 = crate::Reg<padcfg4::PADCFG4_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg4;
#[doc = "PADCFG5 register accessor: an alias for `Reg<PADCFG5_SPEC>`"]
pub type PADCFG5 = crate::Reg<padcfg5::PADCFG5_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg5;
#[doc = "PADCFG6 register accessor: an alias for `Reg<PADCFG6_SPEC>`"]
pub type PADCFG6 = crate::Reg<padcfg6::PADCFG6_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg6;
#[doc = "PADCFG7 register accessor: an alias for `Reg<PADCFG7_SPEC>`"]
pub type PADCFG7 = crate::Reg<padcfg7::PADCFG7_SPEC>;
#[doc = "The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
pub mod padcfg7;
