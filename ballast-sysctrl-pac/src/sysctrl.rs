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
    pub soc_control: SOCCONTROL,
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
    pub boot_config: BOOTCONFIG,
    _reserved11: [u8; 0x0e28],
    #[doc = "0x105000..0x105108 - AdvancedTimer"]
    pub advanced_timer: ADVANCEDTIMER,
    _reserved12: [u8; 0x0ef8],
    #[doc = "0x106000..0x10608c - SocEventGenerator"]
    pub soc_event_generator: SOCEVENTGENERATOR,
    _reserved13: [u8; 0x2f74],
    #[doc = "0x109000..0x109028 - EventInterruptUnit"]
    pub event_interrupt_unit: EVENTINTERRUPTUNIT,
    _reserved14: [u8; 0x1fd8],
    #[doc = "0x10b000..0x10b028 - Timer"]
    pub timer: TIMER,
    _reserved15: [u8; 0x0001_4fd8],
    #[doc = "0x120000..0x120060 - SDIO"]
    pub sdio: SDIO,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
    pub dir: crate::Reg<self::gpio::dir::DIR_SPEC>,
    #[doc = "0x04 - GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
    pub en: crate::Reg<self::gpio::en::EN_SPEC>,
    #[doc = "0x08 - GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
    pub in_: crate::Reg<self::gpio::in_::IN_SPEC>,
    #[doc = "0x0c - GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
    pub out: crate::Reg<self::gpio::out::OUT_SPEC>,
    _reserved4: [u8; 0x18],
    #[doc = "0x28 - "]
    pub pad_cfg_0_3: crate::Reg<self::gpio::pad_cfg_0_3::PAD_CFG_0_3_SPEC>,
    #[doc = "0x2c - "]
    pub pad_cfg_4_7: crate::Reg<self::gpio::pad_cfg_4_7::PAD_CFG_4_7_SPEC>,
    #[doc = "0x30 - "]
    pub pad_cfg_8_11: crate::Reg<self::gpio::pad_cfg_8_11::PAD_CFG_8_11_SPEC>,
    #[doc = "0x34 - "]
    pub pad_cfg_12_15: crate::Reg<self::gpio::pad_cfg_12_15::PAD_CFG_12_15_SPEC>,
}
#[doc = r"Register block"]
#[doc = "GPIO"]
pub mod gpio;
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
    _reserved24: [u8; 0x02d4],
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
    pub filt_status: crate::Reg<self::udma::filt_status::FILT_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "UDMA"]
pub mod udma;
#[doc = r"Register block"]
#[repr(C)]
pub struct ADVANCEDTIMER {
    #[doc = "0x00 - ADV_TIMER0 command register"]
    pub t0_cmd: crate::Reg<self::advanced_timer::t0_cmd::T0_CMD_SPEC>,
    #[doc = "0x04 - ADV_TIMER0 configuration register."]
    pub t0_config: crate::Reg<self::advanced_timer::t0_config::T0_CONFIG_SPEC>,
    #[doc = "0x08 - ADV_TIMER0 threshold configuration register."]
    pub t0_threshold: crate::Reg<self::advanced_timer::t0_threshold::T0_THRESHOLD_SPEC>,
    #[doc = "0x0c..0x1c - ADV_TIMER0 channel 0 threshold configuration register"]
    pub t0_th_channel: [crate::Reg<self::advanced_timer::t0_th_channel::T0_TH_CHANNEL_SPEC>; 4],
    _reserved4: [u8; 0x10],
    #[doc = "0x2c - ADV_TIMER0 counter register"]
    pub t0_counter: crate::Reg<self::advanced_timer::t0_counter::T0_COUNTER_SPEC>,
    _reserved5: [u8; 0x10],
    #[doc = "0x40 - ADV_TIMER1 command register"]
    pub t1_cmd: crate::Reg<self::advanced_timer::t1_cmd::T1_CMD_SPEC>,
    #[doc = "0x44 - ADV_TIMER1 configuration register"]
    pub t1_config: crate::Reg<self::advanced_timer::t1_config::T1_CONFIG_SPEC>,
    #[doc = "0x48 - ADV_TIMER1 threshold configuration register"]
    pub t1_threshold: crate::Reg<self::advanced_timer::t1_threshold::T1_THRESHOLD_SPEC>,
    #[doc = "0x4c..0x5c - ADV_TIMER1 channel 0 threshold configuration register"]
    pub t1_th_channel: [crate::Reg<self::advanced_timer::t1_th_channel::T1_TH_CHANNEL_SPEC>; 4],
    _reserved9: [u8; 0x10],
    #[doc = "0x6c - ADV_TIMER1 counter register"]
    pub t1_counter: crate::Reg<self::advanced_timer::t1_counter::T1_COUNTER_SPEC>,
    _reserved10: [u8; 0x10],
    #[doc = "0x80 - ADV_TIMER2 command register"]
    pub t2_cmd: crate::Reg<self::advanced_timer::t2_cmd::T2_CMD_SPEC>,
    #[doc = "0x84 - ADV_TIMER2 configuration register"]
    pub t2_config: crate::Reg<self::advanced_timer::t2_config::T2_CONFIG_SPEC>,
    #[doc = "0x88 - ADV_TIMER2 threshold configuration register"]
    pub t2_threshold: crate::Reg<self::advanced_timer::t2_threshold::T2_THRESHOLD_SPEC>,
    #[doc = "0x8c..0x9c - ADV_TIMER2 channel 0 threshold configuration register"]
    pub t2_th_channel: [crate::Reg<self::advanced_timer::t2_th_channel::T2_TH_CHANNEL_SPEC>; 4],
    _reserved14: [u8; 0x10],
    #[doc = "0xac - ADV_TIMER2 counter register"]
    pub t2_counter: crate::Reg<self::advanced_timer::t2_counter::T2_COUNTER_SPEC>,
    _reserved15: [u8; 0x10],
    #[doc = "0xc0 - ADV_TIMER3 command register"]
    pub t3_cmd: crate::Reg<self::advanced_timer::t3_cmd::T3_CMD_SPEC>,
    #[doc = "0xc4 - ADV_TIMER3 configuration register"]
    pub t3_config: crate::Reg<self::advanced_timer::t3_config::T3_CONFIG_SPEC>,
    #[doc = "0xc8 - ADV_TIMER3 threshold configuration register"]
    pub t3_threshold: crate::Reg<self::advanced_timer::t3_threshold::T3_THRESHOLD_SPEC>,
    #[doc = "0xcc..0xdc - ADV_TIMER3 channel 0 threshold configuration register"]
    pub t3_th_channel: [crate::Reg<self::advanced_timer::t3_th_channel::T3_TH_CHANNEL_SPEC>; 4],
    _reserved19: [u8; 0x10],
    #[doc = "0xec - ADV_TIMER3 counter register"]
    pub t3_counter: crate::Reg<self::advanced_timer::t3_counter::T3_COUNTER_SPEC>,
    _reserved20: [u8; 0x10],
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
pub struct SOCEVENTGENERATOR {
    #[doc = "0x00 - SoC software events trigger register"]
    pub sw_event: crate::Reg<self::soc_event_generator::sw_event::SW_EVENT_SPEC>,
    #[doc = "0x04..0x24 - Events 0-31 dispatch mask to FC"]
    pub fc_mask: [crate::Reg<self::soc_event_generator::fc_mask::FC_MASK_SPEC>; 8],
    _reserved2: [u8; 0x20],
    #[doc = "0x44..0x64 - Events 0-31 dispatch mask to peripherals"]
    pub pr_mask: [crate::Reg<self::soc_event_generator::pr_mask::PR_MASK_SPEC>; 8],
    #[doc = "0x64..0x84 - Events 0-31 event queue overflow"]
    pub err: [crate::Reg<self::soc_event_generator::err::ERR_SPEC>; 8],
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
    #[doc = "0x24 - Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
    pub fifo_data: crate::Reg<self::event_interrupt_unit::fifo_data::FIFO_DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "EventInterruptUnit"]
pub mod event_interrupt_unit;
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
pub struct SDIO {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - "]
    pub cmd_op: crate::Reg<self::sdio::cmd_op::CMD_OP_SPEC>,
    #[doc = "0x24 - "]
    pub cmd_arg: crate::Reg<self::sdio::cmd_arg::CMD_ARG_SPEC>,
    #[doc = "0x28 - "]
    pub data_setup: crate::Reg<self::sdio::data_setup::DATA_SETUP_SPEC>,
    #[doc = "0x2c - "]
    pub start: crate::Reg<self::sdio::start::START_SPEC>,
    #[doc = "0x30 - "]
    pub rsp0: crate::Reg<self::sdio::rsp0::RSP0_SPEC>,
    #[doc = "0x34 - "]
    pub rsp1: crate::Reg<self::sdio::rsp1::RSP1_SPEC>,
    #[doc = "0x38 - "]
    pub rsp2: crate::Reg<self::sdio::rsp2::RSP2_SPEC>,
    #[doc = "0x3c - "]
    pub rsp3: crate::Reg<self::sdio::rsp3::RSP3_SPEC>,
    #[doc = "0x40 - "]
    pub clk_div_0: crate::Reg<self::sdio::clk_div_0::CLK_DIV_0_SPEC>,
    #[doc = "0x44 - "]
    pub status: crate::Reg<self::sdio::status::STATUS_SPEC>,
    #[doc = "0x48 - Card Identification Word0"]
    pub cid0: crate::Reg<self::sdio::cid0::CID0_SPEC>,
    #[doc = "0x4c - Card Identification Word 1"]
    pub cid1: crate::Reg<self::sdio::cid1::CID1_SPEC>,
    #[doc = "0x50 - Card Identification Word 2"]
    pub cid2: crate::Reg<self::sdio::cid2::CID2_SPEC>,
    #[doc = "0x54 - Card Identification Word 3"]
    pub cid3: crate::Reg<self::sdio::cid3::CID3_SPEC>,
    #[doc = "0x58 - "]
    pub rca: crate::Reg<self::sdio::rca::RCA_SPEC>,
    #[doc = "0x5c - "]
    pub stop: crate::Reg<self::sdio::stop::STOP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SDIO"]
pub mod sdio;
#[doc = r"Register block"]
#[repr(C)]
pub struct SOCCONTROL {
    #[doc = "0x00 - This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register."]
    pub info: crate::Reg<self::soc_control::info::INFO_SPEC>,
    #[doc = "0x04 - This register holds the boot address."]
    pub fcboot: crate::Reg<self::soc_control::fcboot::FCBOOT_SPEC>,
    #[doc = "0x08 - This register contains the value of the fetch enable signal of the core."]
    pub fcfetch: crate::Reg<self::soc_control::fcfetch::FCFETCH_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\])."]
    pub pad_mux: crate::Reg<self::soc_control::pad_mux::PAD_MUX_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - All 10 bit fields have reset value of 10'b10_0011_0100: 0 drive strenght 1 drive strenght 2 trigger 3 trigger 4 rate 5 output en(0) 6 hold 7 pull enable 8 pd(0)/pu(1) 9 input en(1)"]
    pub pad_cfg_0: crate::Reg<self::soc_control::pad_cfg_0::PAD_CFG_0_SPEC>,
    #[doc = "0x24 - "]
    pub pad_cfg_1: crate::Reg<self::soc_control::pad_cfg_1::PAD_CFG_1_SPEC>,
    #[doc = "0x28 - "]
    pub pad_cfg_2: crate::Reg<self::soc_control::pad_cfg_2::PAD_CFG_2_SPEC>,
    #[doc = "0x2c - "]
    pub pad_cfg_3: crate::Reg<self::soc_control::pad_cfg_3::PAD_CFG_3_SPEC>,
    #[doc = "0x30 - "]
    pub pad_cfg_4: crate::Reg<self::soc_control::pad_cfg_4::PAD_CFG_4_SPEC>,
    #[doc = "0x34 - "]
    pub pad_cfg_5: crate::Reg<self::soc_control::pad_cfg_5::PAD_CFG_5_SPEC>,
    _reserved10: [u8; 0x28],
    #[doc = "0x60 - "]
    pub tta_pll_loop_ctrl: crate::Reg<self::soc_control::tta_pll_loop_ctrl::TTA_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x64 - "]
    pub tta_pll_div: crate::Reg<self::soc_control::tta_pll_div::TTA_PLL_DIV_SPEC>,
    #[doc = "0x68 - "]
    pub tta_pll_debug_ctrl:
        crate::Reg<self::soc_control::tta_pll_debug_ctrl::TTA_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x6c - "]
    pub tta_pll_enable: crate::Reg<self::soc_control::tta_pll_enable::TTA_PLL_ENABLE_SPEC>,
    #[doc = "0x70 - "]
    pub cluster_ctrl: crate::Reg<self::soc_control::cluster_ctrl::CLUSTER_CTRL_SPEC>,
    #[doc = "0x74 - Register to read or write from JTAG"]
    pub jtagreg: crate::Reg<self::soc_control::jtagreg::JTAGREG_SPEC>,
    #[doc = "0x78 - "]
    pub ctrl_per: crate::Reg<self::soc_control::ctrl_per::CTRL_PER_SPEC>,
    #[doc = "0x7c - "]
    pub cluster_irq: crate::Reg<self::soc_control::cluster_irq::CLUSTER_IRQ_SPEC>,
    #[doc = "0x80 - "]
    pub cluster_boot_addr0:
        crate::Reg<self::soc_control::cluster_boot_addr0::CLUSTER_BOOT_ADDR0_SPEC>,
    #[doc = "0x84 - "]
    pub cluster_boot_addr1:
        crate::Reg<self::soc_control::cluster_boot_addr1::CLUSTER_BOOT_ADDR1_SPEC>,
    #[doc = "0x88 - "]
    pub tta_pll_spare_ctrl:
        crate::Reg<self::soc_control::tta_pll_spare_ctrl::TTA_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x8c - "]
    pub tta_pll_tmux_sel: crate::Reg<self::soc_control::tta_pll_tmux_sel::TTA_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x90 - "]
    pub tta_pll_status1: crate::Reg<self::soc_control::tta_pll_status1::TTA_PLL_STATUS1_SPEC>,
    #[doc = "0x94 - "]
    pub topperiph_clk_div: crate::Reg<self::soc_control::topperiph_clk_div::TOPPERIPH_CLK_DIV_SPEC>,
    #[doc = "0x98 - Subsystem Clock selection. Bit definition for TTA, Ethernet, AI, HPC subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub clk_ctrl1: crate::Reg<self::soc_control::clk_ctrl1::CLK_CTRL1_SPEC>,
    #[doc = "0x9c - Subsystem Clock selection. Bit definition for MPC, Interconnect, C2C and CoreHW subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub clk_ctrl2: crate::Reg<self::soc_control::clk_ctrl2::CLK_CTRL2_SPEC>,
    #[doc = "0xa0 - These 2 registers (CORESTATUS, CS_RO) contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
    pub corestatus: crate::Reg<self::soc_control::corestatus::CORESTATUS_SPEC>,
    #[doc = "0xa4 - "]
    pub slow_pulse_div: crate::Reg<self::soc_control::slow_pulse_div::SLOW_PULSE_DIV_SPEC>,
    #[doc = "0xa8 - "]
    pub periph_clk_div: crate::Reg<self::soc_control::periph_clk_div::PERIPH_CLK_DIV_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0xb0 - Assertion of the Subsystem bit, deasserts the reset to the corresponding subsystem. Bit definition is mentioned in adjoining table"]
    pub ss_reset_en: crate::Reg<self::soc_control::ss_reset_en::SS_RESET_EN_SPEC>,
    #[doc = "0xb4 - Subsystem clock enable register"]
    pub ss_clk_en: crate::Reg<self::soc_control::ss_clk_en::SS_CLK_EN_SPEC>,
    #[doc = "0xb8 - Subsystem Clock selection. Bit definition for Top peripheral subsystem. *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub clk_ctrl3: crate::Reg<self::soc_control::clk_ctrl3::CLK_CTRL3_SPEC>,
    #[doc = "0xbc - "]
    pub tta_pll_status2: crate::Reg<self::soc_control::tta_pll_status2::TTA_PLL_STATUS2_SPEC>,
    #[doc = "0xc0 - "]
    pub cs_ro: crate::Reg<self::soc_control::cs_ro::CS_RO_SPEC>,
    #[doc = "0xc4 - Boot Sel value"]
    pub bootsel: crate::Reg<self::soc_control::bootsel::BOOTSEL_SPEC>,
    #[doc = "0xc8 - "]
    pub clksel: crate::Reg<self::soc_control::clksel::CLKSEL_SPEC>,
    #[doc = "0xcc - Clock divider ratio for the 3 Interconnect modules"]
    pub inter_clk_div: crate::Reg<self::soc_control::inter_clk_div::INTER_CLK_DIV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SocControl"]
pub mod soc_control;
#[doc = r"Register block"]
#[repr(C)]
pub struct ETH_PLL {
    #[doc = "0x00 - "]
    pub eth_pll_loop_ctrl: crate::Reg<self::eth_pll::eth_pll_loop_ctrl::ETH_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub eth_pll_div: crate::Reg<self::eth_pll::eth_pll_div::ETH_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub eth_pll_debug_ctrl: crate::Reg<self::eth_pll::eth_pll_debug_ctrl::ETH_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub eth_pll_enable: crate::Reg<self::eth_pll::eth_pll_enable::ETH_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub eth_pll_spare_ctrl: crate::Reg<self::eth_pll::eth_pll_spare_ctrl::ETH_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub eth_pll_tmux_sel: crate::Reg<self::eth_pll::eth_pll_tmux_sel::ETH_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub eth_pll_status1: crate::Reg<self::eth_pll::eth_pll_status1::ETH_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub eth_pll_status2: crate::Reg<self::eth_pll::eth_pll_status2::ETH_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "ETH_PLL"]
pub mod eth_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct AI_PLL {
    #[doc = "0x00 - "]
    pub ai_pll_loop_ctrl: crate::Reg<self::ai_pll::ai_pll_loop_ctrl::AI_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub ai_pll_div: crate::Reg<self::ai_pll::ai_pll_div::AI_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub ai_pll_debug_ctrl: crate::Reg<self::ai_pll::ai_pll_debug_ctrl::AI_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub ai_pll_enable: crate::Reg<self::ai_pll::ai_pll_enable::AI_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub ai_pll_spare_ctrl: crate::Reg<self::ai_pll::ai_pll_spare_ctrl::AI_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub ai_pll_tmux_sel: crate::Reg<self::ai_pll::ai_pll_tmux_sel::AI_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub ai_pll_status1: crate::Reg<self::ai_pll::ai_pll_status1::AI_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub ai_pll_status2: crate::Reg<self::ai_pll::ai_pll_status2::AI_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "AI_PLL"]
pub mod ai_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct HPC_PLL {
    #[doc = "0x00 - "]
    pub hpc_pll_loop_ctrl: crate::Reg<self::hpc_pll::hpc_pll_loop_ctrl::HPC_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub hpc_pll_div: crate::Reg<self::hpc_pll::hpc_pll_div::HPC_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub hpc_pll_debug_ctrl: crate::Reg<self::hpc_pll::hpc_pll_debug_ctrl::HPC_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub hpc_pll_enable: crate::Reg<self::hpc_pll::hpc_pll_enable::HPC_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub hpc_pll_spare_ctrl: crate::Reg<self::hpc_pll::hpc_pll_spare_ctrl::HPC_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub hpc_pll_tmux_sel: crate::Reg<self::hpc_pll::hpc_pll_tmux_sel::HPC_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub hpc_pll_status1: crate::Reg<self::hpc_pll::hpc_pll_status1::HPC_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub hpc_pll_status2: crate::Reg<self::hpc_pll::hpc_pll_status2::HPC_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HPC_PLL"]
pub mod hpc_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct PULP_PLL {
    #[doc = "0x00 - "]
    pub pulp_pll_loop_ctrl: crate::Reg<self::pulp_pll::pulp_pll_loop_ctrl::PULP_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub pulp_pll_div: crate::Reg<self::pulp_pll::pulp_pll_div::PULP_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub pulp_pll_debug_ctrl:
        crate::Reg<self::pulp_pll::pulp_pll_debug_ctrl::PULP_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub pulp_pll_enable: crate::Reg<self::pulp_pll::pulp_pll_enable::PULP_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub pulp_pll_spare_ctrl:
        crate::Reg<self::pulp_pll::pulp_pll_spare_ctrl::PULP_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub pulp_pll_tmux_sel: crate::Reg<self::pulp_pll::pulp_pll_tmux_sel::PULP_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub pulp_pll_status1: crate::Reg<self::pulp_pll::pulp_pll_status1::PULP_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub pulp_pll_status2: crate::Reg<self::pulp_pll::pulp_pll_status2::PULP_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PULP_PLL"]
pub mod pulp_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct INTER_PLL {
    #[doc = "0x00 - "]
    pub inter_pll_loop_ctrl:
        crate::Reg<self::inter_pll::inter_pll_loop_ctrl::INTER_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub inter_pll_div: crate::Reg<self::inter_pll::inter_pll_div::INTER_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub inter_pll_debug_ctrl:
        crate::Reg<self::inter_pll::inter_pll_debug_ctrl::INTER_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub inter_pll_enable: crate::Reg<self::inter_pll::inter_pll_enable::INTER_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub inter_pll_spare_ctrl:
        crate::Reg<self::inter_pll::inter_pll_spare_ctrl::INTER_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub inter_pll_tmux_sel:
        crate::Reg<self::inter_pll::inter_pll_tmux_sel::INTER_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub inter_pll_status1: crate::Reg<self::inter_pll::inter_pll_status1::INTER_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub inter_pll_status2: crate::Reg<self::inter_pll::inter_pll_status2::INTER_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "INTER_PLL"]
pub mod inter_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct C2C_PLL {
    #[doc = "0x00 - "]
    pub c2c_pll_loop_ctrl: crate::Reg<self::c2c_pll::c2c_pll_loop_ctrl::C2C_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub c2c_pll_div: crate::Reg<self::c2c_pll::c2c_pll_div::C2C_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub c2c_pll_debug_ctrl: crate::Reg<self::c2c_pll::c2c_pll_debug_ctrl::C2C_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub c2c_pll_enable: crate::Reg<self::c2c_pll::c2c_pll_enable::C2C_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub c2c_pll_spare_ctrl: crate::Reg<self::c2c_pll::c2c_pll_spare_ctrl::C2C_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub c2c_pll_tmux_sel: crate::Reg<self::c2c_pll::c2c_pll_tmux_sel::C2C_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub c2c_pll_status1: crate::Reg<self::c2c_pll::c2c_pll_status1::C2C_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub c2c_pll_status2: crate::Reg<self::c2c_pll::c2c_pll_status2::C2C_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "C2C_PLL"]
pub mod c2c_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct TOPPERIPH_PLL {
    #[doc = "0x00 - "]
    pub topperiph_pll_loop_ctrl:
        crate::Reg<self::topperiph_pll::topperiph_pll_loop_ctrl::TOPPERIPH_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub topperiph_pll_div:
        crate::Reg<self::topperiph_pll::topperiph_pll_div::TOPPERIPH_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub topperiph_pll_debug_ctrl:
        crate::Reg<self::topperiph_pll::topperiph_pll_debug_ctrl::TOPPERIPH_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub topperiph_pll_enable:
        crate::Reg<self::topperiph_pll::topperiph_pll_enable::TOPPERIPH_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub topperiph_pll_spare_ctrl:
        crate::Reg<self::topperiph_pll::topperiph_pll_spare_ctrl::TOPPERIPH_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub topperiph_pll_tmux_sel:
        crate::Reg<self::topperiph_pll::topperiph_pll_tmux_sel::TOPPERIPH_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub topperiph_pll_status1:
        crate::Reg<self::topperiph_pll::topperiph_pll_status1::TOPPERIPH_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub topperiph_pll_status2:
        crate::Reg<self::topperiph_pll::topperiph_pll_status2::TOPPERIPH_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "TOPPERIPH_PLL"]
pub mod topperiph_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct BOOTCONFIG {
    #[doc = "0x00 - "]
    pub boot_cfg: crate::Reg<self::boot_config::boot_cfg::BOOT_CFG_SPEC>,
    #[doc = "0x04 - "]
    pub boot_status: crate::Reg<self::boot_config::boot_status::BOOT_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "BootConfig"]
pub mod boot_config;
