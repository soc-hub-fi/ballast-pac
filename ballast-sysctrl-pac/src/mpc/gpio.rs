#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - Bit 31 - 0 DIR (R/W) GPIO\\[31:0\\]
direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
    pub paddir_00_31: PADDIR_00_31,
    #[doc = "0x04 - Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
    pub en_00_31: EN_00_31,
    #[doc = "0x08 - Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
    pub padin_00_31: PADIN_00_31,
    #[doc = "0x0c - Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
    pub padout_00_31: PADOUT_00_31,
    #[doc = "0x10 - GPIO pad output set register"]
    pub padoutset_00_31: PADOUTSET_00_31,
    #[doc = "0x14 - GPIO pad output clear register"]
    pub padoutclr_00_31: PADOUTCLR_00_31,
    #[doc = "0x18 - "]
    pub inten_00_31: INTEN_00_31,
    #[doc = "0x1c - Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[15:0\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
    pub inttype_00_15: INTTYPE_00_15,
    #[doc = "0x20 - Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[31:16\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
    pub inttype_16_31: INTTYPE_16_31,
    #[doc = "0x24 - Bit 31 - 0 INTSTATUS (R) GPIO\\[31:0\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red"]
    pub intstatus_00_31: INTSTATUS_00_31,
    #[doc = "0x28 - supports configuration for 4 PADS"]
    pub padcfg_00_07: PADCFG_00_07,
    #[doc = "0x2c - "]
    pub padcfg_08_15: PADCFG_08_15,
    #[doc = "0x30 - "]
    pub padcfg_16_23: PADCFG_16_23,
    #[doc = "0x34 - "]
    pub padcfg_24_31: PADCFG_24_31,
    #[doc = "0x38 - Bit 31 - 0 DIR (R/W) GPIO\\[63:32\\]
direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
    pub paddir_32_63: PADDIR_32_63,
    #[doc = "0x3c - GPIOEN (R/W) GPIO\\[63:32\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
    pub en_32_63: EN_32_63,
    #[doc = "0x40 - "]
    pub padin_32_63: PADIN_32_63,
    #[doc = "0x44 - Bit 31 - 0 DATA_OUT (R/W) GPIO\\[63:32\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]"]
    pub padout_32_63: PADOUT_32_63,
    #[doc = "0x48 - "]
    pub padoutset_32_63: PADOUTSET_32_63,
    #[doc = "0x4c - "]
    pub padoutclr_32_63: PADOUTCLR_32_63,
    #[doc = "0x50 - Bit 31 - 0 INTEN (R/W) GPIO\\[63:32\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]"]
    pub inten_32_63: INTEN_32_63,
    #[doc = "0x54 - Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[47:32\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
    pub inttype_32_47: INTTYPE_32_47,
    #[doc = "0x58 - Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[63:48\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
    pub inttype_48_63: INTTYPE_48_63,
    #[doc = "0x5c - Bit 31 - 0 INTSTATUS (R) GPIO\\[63:32\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red."]
    pub intstatus_32_63: INTSTATUS_32_63,
    #[doc = "0x60 - "]
    pub padcfg_32_39: PADCFG_32_39,
    #[doc = "0x64 - "]
    pub padcfg_40_47: PADCFG_40_47,
    #[doc = "0x68 - "]
    pub padcfg_48_55: PADCFG_48_55,
    #[doc = "0x6c - "]
    pub padcfg_56_63: PADCFG_56_63,
}
#[doc = "PADDIR_00_31 (rw) register accessor: an alias for `Reg<PADDIR_00_31_SPEC>`"]
pub type PADDIR_00_31 = crate::Reg<paddir_00_31::PADDIR_00_31_SPEC>;
#[doc = "Bit 31 - 0 DIR (R/W) GPIO\\[31:0\\]
direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
pub mod paddir_00_31;
#[doc = "EN_00_31 (rw) register accessor: an alias for `Reg<EN_00_31_SPEC>`"]
pub type EN_00_31 = crate::Reg<en_00_31::EN_00_31_SPEC>;
#[doc = "Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
pub mod en_00_31;
#[doc = "PADIN_00_31 (r) register accessor: an alias for `Reg<PADIN_00_31_SPEC>`"]
pub type PADIN_00_31 = crate::Reg<padin_00_31::PADIN_00_31_SPEC>;
#[doc = "Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
pub mod padin_00_31;
#[doc = "PADOUT_00_31 (rw) register accessor: an alias for `Reg<PADOUT_00_31_SPEC>`"]
pub type PADOUT_00_31 = crate::Reg<padout_00_31::PADOUT_00_31_SPEC>;
#[doc = "Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
pub mod padout_00_31;
#[doc = "PADOUTSET_00_31 (rw) register accessor: an alias for `Reg<PADOUTSET_00_31_SPEC>`"]
pub type PADOUTSET_00_31 = crate::Reg<padoutset_00_31::PADOUTSET_00_31_SPEC>;
#[doc = "GPIO pad output set register"]
pub mod padoutset_00_31;
#[doc = "PADOUTCLR_00_31 (rw) register accessor: an alias for `Reg<PADOUTCLR_00_31_SPEC>`"]
pub type PADOUTCLR_00_31 = crate::Reg<padoutclr_00_31::PADOUTCLR_00_31_SPEC>;
#[doc = "GPIO pad output clear register"]
pub mod padoutclr_00_31;
#[doc = "INTEN_00_31 (rw) register accessor: an alias for `Reg<INTEN_00_31_SPEC>`"]
pub type INTEN_00_31 = crate::Reg<inten_00_31::INTEN_00_31_SPEC>;
#[doc = ""]
pub mod inten_00_31;
#[doc = "INTTYPE_00_15 (rw) register accessor: an alias for `Reg<INTTYPE_00_15_SPEC>`"]
pub type INTTYPE_00_15 = crate::Reg<inttype_00_15::INTTYPE_00_15_SPEC>;
#[doc = "Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[15:0\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
pub mod inttype_00_15;
#[doc = "INTTYPE_16_31 (rw) register accessor: an alias for `Reg<INTTYPE_16_31_SPEC>`"]
pub type INTTYPE_16_31 = crate::Reg<inttype_16_31::INTTYPE_16_31_SPEC>;
#[doc = "Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[31:16\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
pub mod inttype_16_31;
#[doc = "INTSTATUS_00_31 (r) register accessor: an alias for `Reg<INTSTATUS_00_31_SPEC>`"]
pub type INTSTATUS_00_31 = crate::Reg<intstatus_00_31::INTSTATUS_00_31_SPEC>;
#[doc = "Bit 31 - 0 INTSTATUS (R) GPIO\\[31:0\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red"]
pub mod intstatus_00_31;
#[doc = "PADCFG_00_07 (rw) register accessor: an alias for `Reg<PADCFG_00_07_SPEC>`"]
pub type PADCFG_00_07 = crate::Reg<padcfg_00_07::PADCFG_00_07_SPEC>;
#[doc = "supports configuration for 4 PADS"]
pub mod padcfg_00_07;
#[doc = "PADCFG_56_63 (rw) register accessor: an alias for `Reg<PADCFG_56_63_SPEC>`"]
pub type PADCFG_56_63 = crate::Reg<padcfg_56_63::PADCFG_56_63_SPEC>;
#[doc = ""]
pub mod padcfg_56_63;
#[doc = "PADCFG_48_55 (rw) register accessor: an alias for `Reg<PADCFG_48_55_SPEC>`"]
pub type PADCFG_48_55 = crate::Reg<padcfg_48_55::PADCFG_48_55_SPEC>;
#[doc = ""]
pub mod padcfg_48_55;
#[doc = "PADCFG_40_47 (rw) register accessor: an alias for `Reg<PADCFG_40_47_SPEC>`"]
pub type PADCFG_40_47 = crate::Reg<padcfg_40_47::PADCFG_40_47_SPEC>;
#[doc = ""]
pub mod padcfg_40_47;
#[doc = "PADCFG_32_39 (rw) register accessor: an alias for `Reg<PADCFG_32_39_SPEC>`"]
pub type PADCFG_32_39 = crate::Reg<padcfg_32_39::PADCFG_32_39_SPEC>;
#[doc = ""]
pub mod padcfg_32_39;
#[doc = "INTSTATUS_32_63 (r) register accessor: an alias for `Reg<INTSTATUS_32_63_SPEC>`"]
pub type INTSTATUS_32_63 = crate::Reg<intstatus_32_63::INTSTATUS_32_63_SPEC>;
#[doc = "Bit 31 - 0 INTSTATUS (R) GPIO\\[63:32\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red."]
pub mod intstatus_32_63;
#[doc = "INTTYPE_48_63 (rw) register accessor: an alias for `Reg<INTTYPE_48_63_SPEC>`"]
pub type INTTYPE_48_63 = crate::Reg<inttype_48_63::INTTYPE_48_63_SPEC>;
#[doc = "Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[63:48\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
pub mod inttype_48_63;
#[doc = "INTTYPE_32_47 (rw) register accessor: an alias for `Reg<INTTYPE_32_47_SPEC>`"]
pub type INTTYPE_32_47 = crate::Reg<inttype_32_47::INTTYPE_32_47_SPEC>;
#[doc = "Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[47:32\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2b11: RFU"]
pub mod inttype_32_47;
#[doc = "INTEN_32_63 (rw) register accessor: an alias for `Reg<INTEN_32_63_SPEC>`"]
pub type INTEN_32_63 = crate::Reg<inten_32_63::INTEN_32_63_SPEC>;
#[doc = "Bit 31 - 0 INTEN (R/W) GPIO\\[63:32\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable interrupt for GPIO\\[i\\]"]
pub mod inten_32_63;
#[doc = "PADOUTCLR_32_63 (rw) register accessor: an alias for `Reg<PADOUTCLR_32_63_SPEC>`"]
pub type PADOUTCLR_32_63 = crate::Reg<padoutclr_32_63::PADOUTCLR_32_63_SPEC>;
#[doc = ""]
pub mod padoutclr_32_63;
#[doc = "PADOUTSET_32_63 (rw) register accessor: an alias for `Reg<PADOUTSET_32_63_SPEC>`"]
pub type PADOUTSET_32_63 = crate::Reg<padoutset_32_63::PADOUTSET_32_63_SPEC>;
#[doc = ""]
pub mod padoutset_32_63;
#[doc = "PADOUT_32_63 (rw) register accessor: an alias for `Reg<PADOUT_32_63_SPEC>`"]
pub type PADOUT_32_63 = crate::Reg<padout_32_63::PADOUT_32_63_SPEC>;
#[doc = "Bit 31 - 0 DATA_OUT (R/W) GPIO\\[63:32\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]"]
pub mod padout_32_63;
#[doc = "PADIN_32_63 (r) register accessor: an alias for `Reg<PADIN_32_63_SPEC>`"]
pub type PADIN_32_63 = crate::Reg<padin_32_63::PADIN_32_63_SPEC>;
#[doc = ""]
pub mod padin_32_63;
#[doc = "EN_32_63 (rw) register accessor: an alias for `Reg<EN_32_63_SPEC>`"]
pub type EN_32_63 = crate::Reg<en_32_63::EN_32_63_SPEC>;
#[doc = "GPIOEN (R/W) GPIO\\[63:32\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
pub mod en_32_63;
#[doc = "PADDIR_32_63 (rw) register accessor: an alias for `Reg<PADDIR_32_63_SPEC>`"]
pub type PADDIR_32_63 = crate::Reg<paddir_32_63::PADDIR_32_63_SPEC>;
#[doc = "Bit 31 - 0 DIR (R/W) GPIO\\[63:32\\]
direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
pub mod paddir_32_63;
#[doc = "PADCFG_08_15 (rw) register accessor: an alias for `Reg<PADCFG_08_15_SPEC>`"]
pub type PADCFG_08_15 = crate::Reg<padcfg_08_15::PADCFG_08_15_SPEC>;
#[doc = ""]
pub mod padcfg_08_15;
#[doc = "PADCFG_16_23 (rw) register accessor: an alias for `Reg<PADCFG_16_23_SPEC>`"]
pub type PADCFG_16_23 = crate::Reg<padcfg_16_23::PADCFG_16_23_SPEC>;
#[doc = ""]
pub mod padcfg_16_23;
#[doc = "PADCFG_24_31 (rw) register accessor: an alias for `Reg<PADCFG_24_31_SPEC>`"]
pub type PADCFG_24_31 = crate::Reg<padcfg_24_31::PADCFG_24_31_SPEC>;
#[doc = ""]
pub mod padcfg_24_31;
