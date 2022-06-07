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
    pub soc_control: SOCCONTROL,
    _reserved3: [u8; 0x0f34],
    #[doc = "0x105000..0x105108 - AdvancedTimer"]
    pub advanced_timer: ADVANCEDTIMER,
    _reserved4: [u8; 0x0ef8],
    #[doc = "0x106000..0x10608c - SocEventGenerator"]
    pub soc_event_generator: SOCEVENTGENERATOR,
    _reserved5: [u8; 0x2f74],
    #[doc = "0x109000..0x109028 - EventInterruptUnit"]
    pub event_interrupt_unit: EVENTINTERRUPTUNIT,
    _reserved6: [u8; 0x1fd8],
    #[doc = "0x10b000..0x10b028 - Timer"]
    pub timer: TIMER,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - Bit 31 - 0 DIR (R/W) GPIO\\[31:0\\]
direction configuration bitfield: - bit\\[i\\]=1’b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1’b1: Output mode for GPIO\\[i\\]"]
    pub paddir_00_31: crate::Reg<self::gpio::paddir_00_31::PADDIR_00_31_SPEC>,
    #[doc = "0x04 - Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1’b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1’b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if it’s direction is configured in input mode."]
    pub en_00_31: crate::Reg<self::gpio::en_00_31::EN_00_31_SPEC>,
    #[doc = "0x08 - Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
    pub padin_00_31: crate::Reg<self::gpio::padin_00_31::PADIN_00_31_SPEC>,
    #[doc = "0x0c - Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
    pub padout_00_31: crate::Reg<self::gpio::padout_00_31::PADOUT_00_31_SPEC>,
    #[doc = "0x10 - GPIO pad output set register"]
    pub padoutset_00_31: crate::Reg<self::gpio::padoutset_00_31::PADOUTSET_00_31_SPEC>,
    #[doc = "0x14 - GPIO pad output clear register"]
    pub padoutclr_00_31: crate::Reg<self::gpio::padoutclr_00_31::PADOUTCLR_00_31_SPEC>,
    #[doc = "0x18 - "]
    pub inten_00_31: crate::Reg<self::gpio::inten_00_31::INTEN_00_31_SPEC>,
    #[doc = "0x1c - Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[15:0\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2’b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2’b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2’b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2’b11: RFU"]
    pub inttype_00_15: crate::Reg<self::gpio::inttype_00_15::INTTYPE_00_15_SPEC>,
    #[doc = "0x20 - Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[31:16\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2’b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b11: RFU"]
    pub inttype_16_31: crate::Reg<self::gpio::inttype_16_31::INTTYPE_16_31_SPEC>,
    #[doc = "0x24 - Bit 31 - 0 INTSTATUS (R) GPIO\\[31:0\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red"]
    pub intstatus_00_31: crate::Reg<self::gpio::intstatus_00_31::INTSTATUS_00_31_SPEC>,
    #[doc = "0x28 - supports configuration for 4 PADS"]
    pub padcfg_00_07: crate::Reg<self::gpio::padcfg_00_07::PADCFG_00_07_SPEC>,
    #[doc = "0x2c - "]
    pub padcfg_08_15: crate::Reg<self::gpio::padcfg_08_15::PADCFG_08_15_SPEC>,
    #[doc = "0x30 - "]
    pub padcfg_16_23: crate::Reg<self::gpio::padcfg_16_23::PADCFG_16_23_SPEC>,
    #[doc = "0x34 - "]
    pub padcfg_24_31: crate::Reg<self::gpio::padcfg_24_31::PADCFG_24_31_SPEC>,
    #[doc = "0x38 - Bit 31 - 0 DIR (R/W) GPIO\\[63:32\\]
direction configuration bitfield: - bit\\[i\\]=1’b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1’b1: Output mode for GPIO\\[i\\]"]
    pub paddir_32_63: crate::Reg<self::gpio::paddir_32_63::PADDIR_32_63_SPEC>,
    #[doc = "0x3c - GPIOEN (R/W) GPIO\\[63:32\\]
clock enable configuration bitfield: - bit\\[i\\]=1’b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1’b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if it’s direction is configured in input mode."]
    pub en_32_63: crate::Reg<self::gpio::en_32_63::EN_32_63_SPEC>,
    #[doc = "0x40 - "]
    pub padin_32_63: crate::Reg<self::gpio::padin_32_63::PADIN_32_63_SPEC>,
    #[doc = "0x44 - Bit 31 - 0 DATA_OUT (R/W) GPIO\\[63:32\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]"]
    pub padout_32_63: crate::Reg<self::gpio::padout_32_63::PADOUT_32_63_SPEC>,
    #[doc = "0x48 - "]
    pub padoutset_32_63: crate::Reg<self::gpio::padoutset_32_63::PADOUTSET_32_63_SPEC>,
    #[doc = "0x4c - "]
    pub padoutclr_32_63: crate::Reg<self::gpio::padoutclr_32_63::PADOUTCLR_32_63_SPEC>,
    #[doc = "0x50 - Bit 31 - 0 INTEN (R/W) GPIO\\[63:32\\]
interrupt enable configuration bitfield: - bit\\[i\\]=1’b0: disable interrupt for GPIO\\[i\\]
- bit\\[i\\]=1’b1: enable interrupt for GPIO\\[i\\]"]
    pub inten_32_63: crate::Reg<self::gpio::inten_32_63::INTEN_32_63_SPEC>,
    #[doc = "0x54 - Bit 31 - 0 INTTYPE0 (R/W) GPIO\\[47:32\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2’b00: interrupt on falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2’b01: interrupt on rising edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2’b10: interrupt on rising and falling edge for GPIO\\[i\\]
- bit\\[2*i+1:2*i\\]=2’b11: RFU"]
    pub inttype_32_47: crate::Reg<self::gpio::inttype_32_47::INTTYPE_32_47_SPEC>,
    #[doc = "0x58 - Bit 31 - 0 INTTYPE1 (R/W) GPIO\\[63:48\\]
interrupt type configuration bitfield: - bit\\[2*i+1:2*i\\]=2’b00: interrupt on falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b01: interrupt on rising edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b10: interrupt on rising and falling edge for GPIO\\[16+i\\]
- bit\\[2*i+1:2*i\\]=2’b11: RFU"]
    pub inttype_48_63: crate::Reg<self::gpio::inttype_48_63::INTTYPE_48_63_SPEC>,
    #[doc = "0x5c - Bit 31 - 0 INTSTATUS (R) GPIO\\[63:32\\]
Interrupt status flags bitfield. INTSTATUS\\[i\\]=1 when interrupt received on GPIO\\[i\\]. INTSTATUS is cleared when it is red. GPIO interrupt line is also cleared when INTSTATUS register is red."]
    pub intstatus_32_63: crate::Reg<self::gpio::intstatus_32_63::INTSTATUS_32_63_SPEC>,
    #[doc = "0x60 - "]
    pub padcfg_32_39: crate::Reg<self::gpio::padcfg_32_39::PADCFG_32_39_SPEC>,
    #[doc = "0x64 - "]
    pub padcfg_40_47: crate::Reg<self::gpio::padcfg_40_47::PADCFG_40_47_SPEC>,
    #[doc = "0x68 - "]
    pub padcfg_48_55: crate::Reg<self::gpio::padcfg_48_55::PADCFG_48_55_SPEC>,
    #[doc = "0x6c - "]
    pub padcfg_56_63: crate::Reg<self::gpio::padcfg_56_63::PADCFG_56_63_SPEC>,
}
#[doc = r"Register block"]
#[doc = "GPIO"]
pub mod gpio;
#[doc = r"Register block"]
#[repr(C)]
pub struct SOCCONTROL {
    #[doc = "0x00 - This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register."]
    pub info: crate::Reg<self::soc_control::info::INFO_SPEC>,
    #[doc = "0x04 - This register holds the boot address."]
    pub boot_adr: crate::Reg<self::soc_control::boot_adr::BOOT_ADR_SPEC>,
    #[doc = "0x08 - This register contains the value of the fetch enable signal of the core."]
    pub fetch_enable: crate::Reg<self::soc_control::fetch_enable::FETCH_ENABLE_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\])."]
    pub pad_mux_0: crate::Reg<self::soc_control::pad_mux_0::PAD_MUX_0_SPEC>,
    #[doc = "0x14 - The content of these registers can be used to multiplex pads when targeting an ASIC. The second register (0x1A10_4014) can be used to sets the mux (2 bit select) from pin 16 (bits \\[1:0\\]) to 31 (bits \\[31:30\\])."]
    pub pad_mux_1: crate::Reg<self::soc_control::pad_mux_1::PAD_MUX_1_SPEC>,
    #[doc = "0x18 - The content of these registers can be used to multiplex pads when targeting an ASIC. The third register (0x1A10_4018) can be used to sets the mux (2 bit select) from pin 32 (bits \\[1:0\\]) to 47 (bits \\[31:30\\])."]
    pub pad_mux_2: crate::Reg<self::soc_control::pad_mux_2::PAD_MUX_2_SPEC>,
    #[doc = "0x1c - The content of these registers can be used to multiplex pads when targeting an ASIC. The forth register (0x1A10_401C) can be used to sets the mux (2 bit select) from pin 48 (bits \\[1:0\\]) to 63 (bits \\[31:30\\])."]
    pub pad_mux_3: crate::Reg<self::soc_control::pad_mux_3::PAD_MUX_3_SPEC>,
    #[doc = "0x20 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_0: crate::Reg<self::soc_control::pad_cfg_0::PAD_CFG_0_SPEC>,
    #[doc = "0x24 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_1: crate::Reg<self::soc_control::pad_cfg_1::PAD_CFG_1_SPEC>,
    #[doc = "0x28 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_2: crate::Reg<self::soc_control::pad_cfg_2::PAD_CFG_2_SPEC>,
    #[doc = "0x2c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_3: crate::Reg<self::soc_control::pad_cfg_3::PAD_CFG_3_SPEC>,
    #[doc = "0x30 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_4: crate::Reg<self::soc_control::pad_cfg_4::PAD_CFG_4_SPEC>,
    #[doc = "0x34 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_5: crate::Reg<self::soc_control::pad_cfg_5::PAD_CFG_5_SPEC>,
    #[doc = "0x38 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_6: crate::Reg<self::soc_control::pad_cfg_6::PAD_CFG_6_SPEC>,
    #[doc = "0x3c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_7: crate::Reg<self::soc_control::pad_cfg_7::PAD_CFG_7_SPEC>,
    #[doc = "0x40 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_8: crate::Reg<self::soc_control::pad_cfg_8::PAD_CFG_8_SPEC>,
    #[doc = "0x44 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_9: crate::Reg<self::soc_control::pad_cfg_9::PAD_CFG_9_SPEC>,
    #[doc = "0x48 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_10: crate::Reg<self::soc_control::pad_cfg_10::PAD_CFG_10_SPEC>,
    #[doc = "0x4c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_11: crate::Reg<self::soc_control::pad_cfg_11::PAD_CFG_11_SPEC>,
    #[doc = "0x50 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_12: crate::Reg<self::soc_control::pad_cfg_12::PAD_CFG_12_SPEC>,
    #[doc = "0x54 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_13: crate::Reg<self::soc_control::pad_cfg_13::PAD_CFG_13_SPEC>,
    #[doc = "0x58 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_14: crate::Reg<self::soc_control::pad_cfg_14::PAD_CFG_14_SPEC>,
    #[doc = "0x5c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_15: crate::Reg<self::soc_control::pad_cfg_15::PAD_CFG_15_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0x74 - This register contains the value of the input from the JTAG and can be used to write 8bit in the JTAG output register for system-to-JTAG communications."]
    pub jtag_reg: crate::Reg<self::soc_control::jtag_reg::JTAG_REG_SPEC>,
    _reserved24: [u8; 0x28],
    #[doc = "0xa0 - 2 Core Status registers contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
    pub core_status_0: crate::Reg<self::soc_control::core_status_0::CORE_STATUS_0_SPEC>,
    _reserved25: [u8; 0x1c],
    #[doc = "0xc0 - 2 Core Status registers contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
    pub core_status_1: crate::Reg<self::soc_control::core_status_1::CORE_STATUS_1_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xc8 - This register contains whether the system clock is coming from the FLL or the FLL is bypassed. It is a read-only register by the core but it can be written via JTAG."]
    pub fll_clock_select: crate::Reg<self::soc_control::fll_clock_select::FLL_CLOCK_SELECT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SocControl"]
pub mod soc_control;
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTINTERRUPTUNIT {
    #[doc = "0x00 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    pub mask_read: crate::Reg<self::event_interrupt_unit::mask_read::MASK_READ_SPEC>,
    #[doc = "0x04 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    pub mask_set: crate::Reg<self::event_interrupt_unit::mask_set::MASK_SET_SPEC>,
    #[doc = "0x08 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    pub mask_clear: crate::Reg<self::event_interrupt_unit::mask_clear::MASK_CLEAR_SPEC>,
    #[doc = "0x0c - This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
    pub int_read: crate::Reg<self::event_interrupt_unit::int_read::INT_READ_SPEC>,
    #[doc = "0x10 - INT_read"]
    pub int_set: crate::Reg<self::event_interrupt_unit::int_set::INT_SET_SPEC>,
    #[doc = "0x14 - "]
    pub int_clear: crate::Reg<self::event_interrupt_unit::int_clear::INT_CLEAR_SPEC>,
    #[doc = "0x18 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    pub ack_read: crate::Reg<self::event_interrupt_unit::ack_read::ACK_READ_SPEC>,
    #[doc = "0x1c - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    pub ack_set: crate::Reg<self::event_interrupt_unit::ack_set::ACK_SET_SPEC>,
    #[doc = "0x20 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    pub ack_clear: crate::Reg<self::event_interrupt_unit::ack_clear::ACK_CLEAR_SPEC>,
    #[doc = "0x24 - "]
    pub fifo_data: crate::Reg<self::event_interrupt_unit::fifo_data::FIFO_DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "EventInterruptUnit"]
pub mod event_interrupt_unit;
#[doc = r"Register block"]
#[repr(C)]
pub struct SOCEVENTGENERATOR {
    #[doc = "0x00 - SoC software events trigger register"]
    pub sw_event: crate::Reg<self::soc_event_generator::sw_event::SW_EVENT_SPEC>,
    #[doc = "0x04 - Events 0-31 dispatch mask to FC"]
    pub fc_mask0: crate::Reg<self::soc_event_generator::fc_mask0::FC_MASK0_SPEC>,
    #[doc = "0x08 - Events 32-63 dispatch mask to FC"]
    pub fc_mask1: crate::Reg<self::soc_event_generator::fc_mask1::FC_MASK1_SPEC>,
    #[doc = "0x0c - Events 64-95 dispatch mask to FC"]
    pub fc_mask2: crate::Reg<self::soc_event_generator::fc_mask2::FC_MASK2_SPEC>,
    #[doc = "0x10 - Events 96-127 dispatch mask to FC"]
    pub fc_mask3: crate::Reg<self::soc_event_generator::fc_mask3::FC_MASK3_SPEC>,
    #[doc = "0x14 - Events 128-159 dispatch mask to FC"]
    pub fc_mask4: crate::Reg<self::soc_event_generator::fc_mask4::FC_MASK4_SPEC>,
    #[doc = "0x18 - Events 160-191 dispatch mask to FC"]
    pub fc_mask5: crate::Reg<self::soc_event_generator::fc_mask5::FC_MASK5_SPEC>,
    #[doc = "0x1c - Events 191-223 dispatch mask to FC"]
    pub fc_mask6: crate::Reg<self::soc_event_generator::fc_mask6::FC_MASK6_SPEC>,
    #[doc = "0x20 - F Events 224-255 dispatch mask to peripherals"]
    pub fc_mask7: crate::Reg<self::soc_event_generator::fc_mask7::FC_MASK7_SPEC>,
    _reserved9: [u8; 0x20],
    #[doc = "0x44 - Events 0-31 dispatch mask to peripherals"]
    pub pr_mask0: crate::Reg<self::soc_event_generator::pr_mask0::PR_MASK0_SPEC>,
    #[doc = "0x48 - "]
    pub pr_mask1: crate::Reg<self::soc_event_generator::pr_mask1::PR_MASK1_SPEC>,
    #[doc = "0x4c - Events 0-31 dispatch mask to peripheralsEvents 64-95 dispatch mask to peripherals"]
    pub pr_mask2: crate::Reg<self::soc_event_generator::pr_mask2::PR_MASK2_SPEC>,
    #[doc = "0x50 - Events 96-127 dispatch mask to peripherals"]
    pub pr_mask3: crate::Reg<self::soc_event_generator::pr_mask3::PR_MASK3_SPEC>,
    #[doc = "0x54 - Events 128-159 dispatch mask to peripheral"]
    pub pr_mask4: crate::Reg<self::soc_event_generator::pr_mask4::PR_MASK4_SPEC>,
    #[doc = "0x58 - Events 160-191 dispatch mask to peripherals"]
    pub pr_mask5: crate::Reg<self::soc_event_generator::pr_mask5::PR_MASK5_SPEC>,
    #[doc = "0x5c - Events 191-223 dispatch mask to peripherals"]
    pub pr_mask6: crate::Reg<self::soc_event_generator::pr_mask6::PR_MASK6_SPEC>,
    #[doc = "0x60 - Events 224-255 dispatch mask to peripherals"]
    pub pr_mask7: crate::Reg<self::soc_event_generator::pr_mask7::PR_MASK7_SPEC>,
    #[doc = "0x64 - Events 0-31 event queue overflow"]
    pub err0: crate::Reg<self::soc_event_generator::err0::ERR0_SPEC>,
    #[doc = "0x68 - Events 32-63 event queue overflow"]
    pub err1: crate::Reg<self::soc_event_generator::err1::ERR1_SPEC>,
    #[doc = "0x6c - Events 64-95 event queue overflow"]
    pub err2: crate::Reg<self::soc_event_generator::err2::ERR2_SPEC>,
    #[doc = "0x70 - Events 96-127 event queue overflow"]
    pub err3: crate::Reg<self::soc_event_generator::err3::ERR3_SPEC>,
    #[doc = "0x74 - Events 128-159 event queue overflow"]
    pub err4: crate::Reg<self::soc_event_generator::err4::ERR4_SPEC>,
    #[doc = "0x78 - Events 160-191 event queue overflow"]
    pub err5: crate::Reg<self::soc_event_generator::err5::ERR5_SPEC>,
    #[doc = "0x7c - Events 191-223 event queue overflow"]
    pub err6: crate::Reg<self::soc_event_generator::err6::ERR6_SPEC>,
    #[doc = "0x80 - Events 224-255 event queue overflow"]
    pub err7: crate::Reg<self::soc_event_generator::err7::ERR7_SPEC>,
    #[doc = "0x84 - "]
    pub timer_lo: crate::Reg<self::soc_event_generator::timer_lo::TIMER_LO_SPEC>,
    #[doc = "0x88 - Trigger Timer HI of APB Timer with event"]
    pub timer_hi: crate::Reg<self::soc_event_generator::timer_hi::TIMER_HI_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SocEventGenerator"]
pub mod soc_event_generator;
#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Timer Low Configuration register"]
    pub cfg_lo: crate::Reg<self::timer::cfg_lo::CFG_LO_SPEC>,
    #[doc = "0x04 - Timer High Configuration register"]
    pub cfg_hi: crate::Reg<self::timer::cfg_hi::CFG_HI_SPEC>,
    #[doc = "0x08 - Timer Low counter value register"]
    pub cnt_lo: crate::Reg<self::timer::cnt_lo::CNT_LO_SPEC>,
    #[doc = "0x0c - Timer High counter value register"]
    pub cnt_hi: crate::Reg<self::timer::cnt_hi::CNT_HI_SPEC>,
    #[doc = "0x10 - Timer Low comparator value register"]
    pub cmp_lo: crate::Reg<self::timer::cmp_lo::CMP_LO_SPEC>,
    #[doc = "0x14 - Timer High comparator value register"]
    pub cmp_hi: crate::Reg<self::timer::cmp_hi::CMP_HI_SPEC>,
    #[doc = "0x18 - Start Timer Low counting register"]
    pub start_lo: crate::Reg<self::timer::start_lo::START_LO_SPEC>,
    #[doc = "0x1c - Start Timer High counting register"]
    pub start_hi: crate::Reg<self::timer::start_hi::START_HI_SPEC>,
    #[doc = "0x20 - Reset Timer Low counter register"]
    pub reset_lo: crate::Reg<self::timer::reset_lo::RESET_LO_SPEC>,
    #[doc = "0x24 - Reset Timer High counter register"]
    pub reset_hi: crate::Reg<self::timer::reset_hi::RESET_HI_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Timer"]
pub mod timer;
#[doc = r"Register block"]
#[repr(C)]
pub struct ADVANCEDTIMER {
    #[doc = "0x00 - ADV_TIMER0 command register"]
    pub t0_cmd: crate::Reg<self::advanced_timer::t0_cmd::T0_CMD_SPEC>,
    #[doc = "0x04 - ADV_TIMER0 configuration register."]
    pub t0_config: crate::Reg<self::advanced_timer::t0_config::T0_CONFIG_SPEC>,
    #[doc = "0x08 - ADV_TIMER0 threshold configuration register."]
    pub t0_threshold: crate::Reg<self::advanced_timer::t0_threshold::T0_THRESHOLD_SPEC>,
    #[doc = "0x0c - ADV_TIMER0 channel 0 threshold configuration register"]
    pub t0_th_channel0: crate::Reg<self::advanced_timer::t0_th_channel0::T0_TH_CHANNEL0_SPEC>,
    #[doc = "0x10 - ADV_TIMER0 channel 1 threshold configuration register"]
    pub t0_th_channel1: crate::Reg<self::advanced_timer::t0_th_channel1::T0_TH_CHANNEL1_SPEC>,
    #[doc = "0x14 - ADV_TIMER0 channel 2 threshold configuration register"]
    pub t0_th_channel2: crate::Reg<self::advanced_timer::t0_th_channel2::T0_TH_CHANNEL2_SPEC>,
    #[doc = "0x18 - ADV_TIMER0 channel 3 threshold configuration register"]
    pub t0_th_channel3: crate::Reg<self::advanced_timer::t0_th_channel3::T0_TH_CHANNEL3_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - ADV_TIMER0 counter register"]
    pub t0_counter: crate::Reg<self::advanced_timer::t0_counter::T0_COUNTER_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - ADV_TIMER1 command register"]
    pub t1_cmd: crate::Reg<self::advanced_timer::t1_cmd::T1_CMD_SPEC>,
    #[doc = "0x44 - ADV_TIMER1 configuration register"]
    pub t1_config: crate::Reg<self::advanced_timer::t1_config::T1_CONFIG_SPEC>,
    #[doc = "0x48 - ADV_TIMER1 threshold configuration register"]
    pub t1_threshold: crate::Reg<self::advanced_timer::t1_threshold::T1_THRESHOLD_SPEC>,
    #[doc = "0x4c - ADV_TIMER1 channel 0 threshold configuration register"]
    pub t1_th_channel0: crate::Reg<self::advanced_timer::t1_th_channel0::T1_TH_CHANNEL0_SPEC>,
    #[doc = "0x50 - ADV_TIMER1 channel 1 threshold configuration register"]
    pub t1_th_channel1: crate::Reg<self::advanced_timer::t1_th_channel1::T1_TH_CHANNEL1_SPEC>,
    #[doc = "0x54 - ADV_TIMER1 channel 2 threshold configuration register"]
    pub t1_th_channel2: crate::Reg<self::advanced_timer::t1_th_channel2::T1_TH_CHANNEL2_SPEC>,
    #[doc = "0x58 - ADV_TIMER1 channel 3 threshold configuration register"]
    pub t1_th_channel3: crate::Reg<self::advanced_timer::t1_th_channel3::T1_TH_CHANNEL3_SPEC>,
    _reserved15: [u8; 0x10],
    #[doc = "0x6c - ADV_TIMER1 counter register"]
    pub t1_counter: crate::Reg<self::advanced_timer::t1_counter::T1_COUNTER_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x80 - ADV_TIMER2 command register"]
    pub t2_cmd: crate::Reg<self::advanced_timer::t2_cmd::T2_CMD_SPEC>,
    #[doc = "0x84 - ADV_TIMER2 configuration register"]
    pub t2_config: crate::Reg<self::advanced_timer::t2_config::T2_CONFIG_SPEC>,
    #[doc = "0x88 - ADV_TIMER2 threshold configuration register"]
    pub t2_threshold: crate::Reg<self::advanced_timer::t2_threshold::T2_THRESHOLD_SPEC>,
    #[doc = "0x8c - ADV_TIMER2 channel 0 threshold configuration register"]
    pub t2_th_channel0: crate::Reg<self::advanced_timer::t2_th_channel0::T2_TH_CHANNEL0_SPEC>,
    #[doc = "0x90 - ADV_TIMER2 channel 1 threshold configuration register"]
    pub t2_th_channel1: crate::Reg<self::advanced_timer::t2_th_channel1::T2_TH_CHANNEL1_SPEC>,
    #[doc = "0x94 - ADV_TIMER2 channel 2 threshold configuration register"]
    pub t2_th_channel2: crate::Reg<self::advanced_timer::t2_th_channel2::T2_TH_CHANNEL2_SPEC>,
    #[doc = "0x98 - ADV_TIMER2 channel 3 threshold configuration register"]
    pub t2_th_channel3: crate::Reg<self::advanced_timer::t2_th_channel3::T2_TH_CHANNEL3_SPEC>,
    _reserved23: [u8; 0x10],
    #[doc = "0xac - ADV_TIMER2 counter register"]
    pub t2_counter: crate::Reg<self::advanced_timer::t2_counter::T2_COUNTER_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0xc0 - ADV_TIMER3 command register"]
    pub t3_cmd: crate::Reg<self::advanced_timer::t3_cmd::T3_CMD_SPEC>,
    #[doc = "0xc4 - ADV_TIMER3 configuration register"]
    pub t3_config: crate::Reg<self::advanced_timer::t3_config::T3_CONFIG_SPEC>,
    #[doc = "0xc8 - ADV_TIMER3 threshold configuration register"]
    pub t3_threshold: crate::Reg<self::advanced_timer::t3_threshold::T3_THRESHOLD_SPEC>,
    #[doc = "0xcc - ADV_TIMER3 channel 0 threshold configuration register"]
    pub t3_th_channel0: crate::Reg<self::advanced_timer::t3_th_channel0::T3_TH_CHANNEL0_SPEC>,
    #[doc = "0xd0 - ADV_TIMER3 channel 1 threshold configuration register"]
    pub t3_th_channel1: crate::Reg<self::advanced_timer::t3_th_channel1::T3_TH_CHANNEL1_SPEC>,
    #[doc = "0xd4 - ADV_TIMER3 channel 2 threshold configuration register"]
    pub t3_th_channel2: crate::Reg<self::advanced_timer::t3_th_channel2::T3_TH_CHANNEL2_SPEC>,
    #[doc = "0xd8 - ADV_TIMER3 channel 3 threshold configuration register"]
    pub t3_th_channel3: crate::Reg<self::advanced_timer::t3_th_channel3::T3_TH_CHANNEL3_SPEC>,
    _reserved31: [u8; 0x10],
    #[doc = "0xec - ADV_TIMER3 counter register"]
    pub t3_counter: crate::Reg<self::advanced_timer::t3_counter::T3_COUNTER_SPEC>,
    _reserved32: [u8; 0x10],
    #[doc = "0x100 - ADV_TIMERS events configuration register."]
    pub event_cfg: crate::Reg<self::advanced_timer::event_cfg::EVENT_CFG_SPEC>,
    #[doc = "0x104 - ADV_TIMERS channels clock gating configuration register."]
    pub cg: crate::Reg<self::advanced_timer::cg::CG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "AdvancedTimer"]
pub mod advanced_timer;
#[doc = r"Register block"]
#[repr(C)]
pub struct UDMA {
    #[doc = "0x00 - uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
    pub ctrl_cfg_cg: crate::Reg<self::udma::ctrl_cfg_cg::CTRL_CFG_CG_SPEC>,
    #[doc = "0x04 - uDMA peripherals external event configuration"]
    pub ctrl_cfg_event: crate::Reg<self::udma::ctrl_cfg_event::CTRL_CFG_EVENT_SPEC>,
    #[doc = "0x08 - uDMA peripherals reset trigger (unimplemented)"]
    pub ctrl_cfg_rst: crate::Reg<self::udma::ctrl_cfg_rst::CTRL_CFG_RST_SPEC>,
    _reserved3: [u8; 0x74],
    #[doc = "0x80 - uDMA RX UART buffer base address configuration register"]
    pub uart_rx_saddr: crate::Reg<self::udma::uart_rx_saddr::UART_RX_SADDR_SPEC>,
    #[doc = "0x84 - uDMA RX UART buffer size configuration register"]
    pub uart_rx_size: crate::Reg<self::udma::uart_rx_size::UART_RX_SIZE_SPEC>,
    #[doc = "0x88 - uDMA RX UART stream configuration register"]
    pub uart_rx_cfg: crate::Reg<self::udma::uart_rx_cfg::UART_RX_CFG_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x90 - uDMA TX UART buffer base address configuration register."]
    pub uart_tx_saddr: crate::Reg<self::udma::uart_tx_saddr::UART_TX_SADDR_SPEC>,
    #[doc = "0x94 - uDMA TX UART buffer size configuration register"]
    pub uart_tx_size: crate::Reg<self::udma::uart_tx_size::UART_TX_SIZE_SPEC>,
    #[doc = "0x98 - uDMA TX UART stream configuration register."]
    pub uart_tx_cfg: crate::Reg<self::udma::uart_tx_cfg::UART_TX_CFG_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0xa0 - uDMA UART status register"]
    pub uart_status: crate::Reg<self::udma::uart_status::UART_STATUS_SPEC>,
    #[doc = "0xa4 - UDMA UART configuration register."]
    pub uart_setup: crate::Reg<self::udma::uart_setup::UART_SETUP_SPEC>,
    #[doc = "0xa8 - uDMA UART Error status"]
    pub uart_error: crate::Reg<self::udma::uart_error::UART_ERROR_SPEC>,
    #[doc = "0xac - uDMA UART Read or Error interrupt enable register"]
    pub uart_irq_en: crate::Reg<self::udma::uart_irq_en::UART_IRQ_EN_SPEC>,
    #[doc = "0xb0 - uDMA UART Read polling data valid flag register"]
    pub uart_valid: crate::Reg<self::udma::uart_valid::UART_VALID_SPEC>,
    #[doc = "0xb4 - RX read data for polling or interrupt"]
    pub uart_data: crate::Reg<self::udma::uart_data::UART_DATA_SPEC>,
    _reserved15: [u8; 0x48],
    #[doc = "0x100 - RX SPI uDMA transfer address of associated buffer"]
    pub spim_rx_saddr: crate::Reg<self::udma::spim_rx_saddr::SPIM_RX_SADDR_SPEC>,
    #[doc = "0x104 - RX SPI uDMA transfer size of buffer"]
    pub spim_rx_size: crate::Reg<self::udma::spim_rx_size::SPIM_RX_SIZE_SPEC>,
    #[doc = "0x108 - RX SPI uDMA transfer configuration"]
    pub spim_rx_cfg: crate::Reg<self::udma::spim_rx_cfg::SPIM_RX_CFG_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x110 - TX SPI uDMA transfer address of associated buffer"]
    pub spim_tx_saddr: crate::Reg<self::udma::spim_tx_saddr::SPIM_TX_SADDR_SPEC>,
    #[doc = "0x114 - TX SPI uDMA transfer size of buffer"]
    pub spim_tx_size: crate::Reg<self::udma::spim_tx_size::SPIM_TX_SIZE_SPEC>,
    #[doc = "0x118 - TX SPI uDMA transfer configuration"]
    pub spim_tx_cfg: crate::Reg<self::udma::spim_tx_cfg::SPIM_TX_CFG_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x120 - CMD SPI uDMA transfer address of associated buffer"]
    pub spim_cmd_saddr: crate::Reg<self::udma::spim_cmd_saddr::SPIM_CMD_SADDR_SPEC>,
    #[doc = "0x124 - CMD SPI uDMA transfer size of buffer"]
    pub spim_cmd_size: crate::Reg<self::udma::spim_cmd_size::SPIM_CMD_SIZE_SPEC>,
    #[doc = "0x128 - CMD SPI uDMA transfer configuration"]
    pub spim_cmd_cfg: crate::Reg<self::udma::spim_cmd_cfg::SPIM_CMD_CFG_SPEC>,
    _reserved24: [u8; 0x54],
    #[doc = "0x180 - uDMA RX I2C buffer base address configuration register."]
    pub i2c0_rx_saddr: crate::Reg<self::udma::i2c0_rx_saddr::I2C0_RX_SADDR_SPEC>,
    #[doc = "0x184 - uDMA RX I2C buffer size configuration register"]
    pub i2c0_rx_size: crate::Reg<self::udma::i2c0_rx_size::I2C0_RX_SIZE_SPEC>,
    #[doc = "0x188 - uDMA RX I2C stream configuration register."]
    pub i2c0_rx_cfg: crate::Reg<self::udma::i2c0_rx_cfg::I2C0_RX_CFG_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x190 - uDMA TX I2C buffer base address configuration register."]
    pub i2c0_tx_saddr: crate::Reg<self::udma::i2c0_tx_saddr::I2C0_TX_SADDR_SPEC>,
    #[doc = "0x194 - uDMA TX I2C buffer size configuration register"]
    pub i2c0_tx_size: crate::Reg<self::udma::i2c0_tx_size::I2C0_TX_SIZE_SPEC>,
    #[doc = "0x198 - uDMA TX I2C stream configuration register"]
    pub i2c0_tx_cfg: crate::Reg<self::udma::i2c0_tx_cfg::I2C0_TX_CFG_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x1a0 - uDMA CMD I2C buffer base address configuration register."]
    pub i2c0_cmd_saddr: crate::Reg<self::udma::i2c0_cmd_saddr::I2C0_CMD_SADDR_SPEC>,
    #[doc = "0x1a4 - uDMA CMD I2C buffer size configuration register"]
    pub i2c0_cmd_size: crate::Reg<self::udma::i2c0_cmd_size::I2C0_CMD_SIZE_SPEC>,
    #[doc = "0x1a8 - uDMA CMD I2C stream configuration register."]
    pub i2c0_cmd_cfg: crate::Reg<self::udma::i2c0_cmd_cfg::I2C0_CMD_CFG_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x1b0 - uDMA I2C Status register"]
    pub i2c0_status: crate::Reg<self::udma::i2c0_status::I2C0_STATUS_SPEC>,
    #[doc = "0x1b4 - uDMA I2C Configuration register."]
    pub i2c0_setup: crate::Reg<self::udma::i2c0_setup::I2C0_SETUP_SPEC>,
    _reserved35: [u8; 0x48],
    #[doc = "0x200 - uDMA RX I2C buffer base address configuration register."]
    pub i2c1_rx_saddr: crate::Reg<self::udma::i2c1_rx_saddr::I2C1_RX_SADDR_SPEC>,
    #[doc = "0x204 - uDMA RX I2C buffer size configuration register"]
    pub i2c1_rx_size: crate::Reg<self::udma::i2c1_rx_size::I2C1_RX_SIZE_SPEC>,
    #[doc = "0x208 - uDMA RX I2C stream configuration register."]
    pub i2c1_rx_cfg: crate::Reg<self::udma::i2c1_rx_cfg::I2C1_RX_CFG_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x210 - uDMA TX I2C buffer base address configuration register."]
    pub i2c1_tx_saddr: crate::Reg<self::udma::i2c1_tx_saddr::I2C1_TX_SADDR_SPEC>,
    #[doc = "0x214 - uDMA TX I2C buffer size configuration register"]
    pub i2c1_tx_size: crate::Reg<self::udma::i2c1_tx_size::I2C1_TX_SIZE_SPEC>,
    #[doc = "0x218 - uDMA TX I2C stream configuration register"]
    pub i2c1_tx_cfg: crate::Reg<self::udma::i2c1_tx_cfg::I2C1_TX_CFG_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x220 - uDMA CMD I2C buffer base address configuration register."]
    pub i2c1_cmd_saddr: crate::Reg<self::udma::i2c1_cmd_saddr::I2C1_CMD_SADDR_SPEC>,
    #[doc = "0x224 - uDMA CMD I2C buffer size configuration register"]
    pub i2c1_cmd_size: crate::Reg<self::udma::i2c1_cmd_size::I2C1_CMD_SIZE_SPEC>,
    #[doc = "0x228 - uDMA CMD I2C stream configuration register."]
    pub i2c1_cmd_cfg: crate::Reg<self::udma::i2c1_cmd_cfg::I2C1_CMD_CFG_SPEC>,
    _reserved44: [u8; 0x04],
    #[doc = "0x230 - uDMA I2C Status register"]
    pub i2c1_status: crate::Reg<self::udma::i2c1_status::I2C1_STATUS_SPEC>,
    #[doc = "0x234 - uDMA I2C Configuration register."]
    pub i2c1_setup: crate::Reg<self::udma::i2c1_setup::I2C1_SETUP_SPEC>,
    _reserved46: [u8; 0xc8],
    #[doc = "0x300 - RX Channel0 I2S uDMA transfer address of associated buffer"]
    pub i2s_rx_saddr: crate::Reg<self::udma::i2s_rx_saddr::I2S_RX_SADDR_SPEC>,
    #[doc = "0x304 - RX Channel 0 I2S uDMA transfer size of buffer"]
    pub i2s_rx_size: crate::Reg<self::udma::i2s_rx_size::I2S_RX_SIZE_SPEC>,
    #[doc = "0x308 - RX Channel 0 I2S uDMA transfer configuration"]
    pub i2s_rx_cfg: crate::Reg<self::udma::i2s_rx_cfg::I2S_RX_CFG_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x310 - TX Channel I2S uDMA transfer address of associated buffer"]
    pub i2s_tx_saddr: crate::Reg<self::udma::i2s_tx_saddr::I2S_TX_SADDR_SPEC>,
    #[doc = "0x314 - TX Channel I2S uDMA transfer size of buffer"]
    pub i2s_tx_size: crate::Reg<self::udma::i2s_tx_size::I2S_TX_SIZE_SPEC>,
    #[doc = "0x318 - TX Channel I2S uDMA transfer configuration"]
    pub i2s_tx_cfg: crate::Reg<self::udma::i2s_tx_cfg::I2S_TX_CFG_SPEC>,
    _reserved52: [u8; 0x04],
    #[doc = "0x320 - Clock configuration for both master, slave and pdm"]
    pub i2s_clkcfg_setup: crate::Reg<self::udma::i2s_clkcfg_setup::I2S_CLKCFG_SETUP_SPEC>,
    #[doc = "0x324 - Configuration of I2S slave"]
    pub i2s_slv_setup: crate::Reg<self::udma::i2s_slv_setup::I2S_SLV_SETUP_SPEC>,
    #[doc = "0x328 - Configuration of I2S master"]
    pub i2s_mst_setup: crate::Reg<self::udma::i2s_mst_setup::I2S_MST_SETUP_SPEC>,
    #[doc = "0x32c - Configuration of PDM module"]
    pub i2s_pdm_setup: crate::Reg<self::udma::i2s_pdm_setup::I2S_PDM_SETUP_SPEC>,
    _reserved56: [u8; 0x50],
    #[doc = "0x380 - RX Camera uDMA transfer address of associated buffer register"]
    pub cam_rx_saddr: crate::Reg<self::udma::cam_rx_saddr::CAM_RX_SADDR_SPEC>,
    #[doc = "0x384 - RX Camera uDMA transfer size of buffer register"]
    pub cam_rx_size: crate::Reg<self::udma::cam_rx_size::CAM_RX_SIZE_SPEC>,
    #[doc = "0x388 - RX Camera uDMA transfer configuration register"]
    pub cam_rx_cfg: crate::Reg<self::udma::cam_rx_cfg::CAM_RX_CFG_SPEC>,
    _reserved59: [u8; 0x14],
    #[doc = "0x3a0 - Global configuration register"]
    pub cam_cfg_glob: crate::Reg<self::udma::cam_cfg_glob::CAM_CFG_GLOB_SPEC>,
    #[doc = "0x3a4 - Lower Left corner configuration register"]
    pub cam_cfg_ll: crate::Reg<self::udma::cam_cfg_ll::CAM_CFG_LL_SPEC>,
    #[doc = "0x3a8 - Upper Right corner configuration register"]
    pub cam_cfg_ur: crate::Reg<self::udma::cam_cfg_ur::CAM_CFG_UR_SPEC>,
    #[doc = "0x3ac - Horizontal Resolution configuration register"]
    pub cam_cfg_size: crate::Reg<self::udma::cam_cfg_size::CAM_CFG_SIZE_SPEC>,
    #[doc = "0x3b0 - RGB coefficients configuration register"]
    pub cam_cfg_filter: crate::Reg<self::udma::cam_cfg_filter::CAM_CFG_FILTER_SPEC>,
    #[doc = "0x3b4 - VSYNC Polarity register"]
    pub cam_vsync_polarity: crate::Reg<self::udma::cam_vsync_polarity::CAM_VSYNC_POLARITY_SPEC>,
    _reserved65: [u8; 0x48],
    #[doc = "0x400 - FILTER tx channel address register"]
    pub tx_ch0_add: crate::Reg<self::udma::tx_ch0_add::TX_CH0_ADD_SPEC>,
    #[doc = "0x404 - FILTER tx channel configuration register"]
    pub tx_ch0_cfg: crate::Reg<self::udma::tx_ch0_cfg::TX_CH0_CFG_SPEC>,
    #[doc = "0x408 - FILTER tx channel length1 register"]
    pub tx_ch0_len0: crate::Reg<self::udma::tx_ch0_len0::TX_CH0_LEN0_SPEC>,
    #[doc = "0x40c - FILTER tx channel length2 register"]
    pub tx_ch0_len1: crate::Reg<self::udma::tx_ch0_len1::TX_CH0_LEN1_SPEC>,
    #[doc = "0x410 - FILTER tx channel 0 length2 register"]
    pub tx_ch0_len2: crate::Reg<self::udma::tx_ch0_len2::TX_CH0_LEN2_SPEC>,
    #[doc = "0x414 - FILTER tx channel address register"]
    pub tx_ch1_add: crate::Reg<self::udma::tx_ch1_add::TX_CH1_ADD_SPEC>,
    #[doc = "0x418 - FILTER tx channel configuration register"]
    pub tx_ch1_cfg: crate::Reg<self::udma::tx_ch1_cfg::TX_CH1_CFG_SPEC>,
    #[doc = "0x41c - FILTER tx channel length1 register"]
    pub tx_ch1_len0: crate::Reg<self::udma::tx_ch1_len0::TX_CH1_LEN0_SPEC>,
    #[doc = "0x420 - FILTER tx channel length2 register"]
    pub tx_ch1_len1: crate::Reg<self::udma::tx_ch1_len1::TX_CH1_LEN1_SPEC>,
    #[doc = "0x424 - FILTER RX channel configuration register"]
    pub tx_ch1_len2: crate::Reg<self::udma::tx_ch1_len2::TX_CH1_LEN2_SPEC>,
    #[doc = "0x428 - FILTER RX channel address register"]
    pub rx_ch_add: crate::Reg<self::udma::rx_ch_add::RX_CH_ADD_SPEC>,
    #[doc = "0x42c - FILTER RX channel configuration register"]
    pub rx_ch_cfg: crate::Reg<self::udma::rx_ch_cfg::RX_CH_CFG_SPEC>,
    #[doc = "0x430 - FILTER RX channel configuration register"]
    pub rx_ch_len0: crate::Reg<self::udma::rx_ch_len0::RX_CH_LEN0_SPEC>,
    #[doc = "0x434 - FILTER RX channel length1 register"]
    pub rx_ch_len1: crate::Reg<self::udma::rx_ch_len1::RX_CH_LEN1_SPEC>,
    #[doc = "0x438 - FILTER RX channel length2 register"]
    pub rx_ch_len2: crate::Reg<self::udma::rx_ch_len2::RX_CH_LEN2_SPEC>,
    #[doc = "0x43c - FILTER arithmetic unit configuration register"]
    pub au_cfg: crate::Reg<self::udma::au_cfg::AU_CFG_SPEC>,
    #[doc = "0x440 - FILTER arithmetic unit 0 register"]
    pub au_reg0: crate::Reg<self::udma::au_reg0::AU_REG0_SPEC>,
    #[doc = "0x444 - FILTER arithmetic unit 1 register"]
    pub au_reg1: crate::Reg<self::udma::au_reg1::AU_REG1_SPEC>,
    #[doc = "0x448 - FILTER binarization threshold register"]
    pub bincu_th: crate::Reg<self::udma::bincu_th::BINCU_TH_SPEC>,
    #[doc = "0x44c - FILTER binarization count register"]
    pub bincu_cnt: crate::Reg<self::udma::bincu_cnt::BINCU_CNT_SPEC>,
    #[doc = "0x450 - FILTER binarization result count register"]
    pub bincu_setup: crate::Reg<self::udma::bincu_setup::BINCU_SETUP_SPEC>,
    #[doc = "0x454 - FILTER binarization result count register"]
    pub bincu_val: crate::Reg<self::udma::bincu_val::BINCU_VAL_SPEC>,
    #[doc = "0x458 - FILTER control mode register"]
    pub filt: crate::Reg<self::udma::filt::FILT_SPEC>,
    #[doc = "0x45c - FILTER start register"]
    pub filt_cmd: crate::Reg<self::udma::filt_cmd::FILT_CMD_SPEC>,
    #[doc = "0x460 - FILTER status register"]
    pub status: crate::Reg<self::udma::status::STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "UDMA"]
pub mod udma;
