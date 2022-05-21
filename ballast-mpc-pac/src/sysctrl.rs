#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0010_1000],
    #[doc = "0x101000..0x101040 - GPIO"]
    pub gpio: GPIO,
    _reserved1: [u8; 0x0fc0],
    #[doc = "0x102000..0x102464 - UDMA"]
    pub udma: UDMA,
    _reserved2: [u8; 0x1b9c],
    #[doc = "0x104000..0x1040d0 - SocControl_TTA_PLL"]
    pub soc_control_tta_pll: SOCCONTROL_TTA_PLL,
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
    #[doc = "0x120000..0x12005c - SDIO"]
    pub sdio: SDIO,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input."]
    pub paddir: crate::Reg<self::gpio::paddir::PADDIR_SPEC>,
    #[doc = "0x04 - Input Values"]
    pub padin: crate::Reg<self::gpio::padin::PADIN_SPEC>,
    #[doc = "0x08 - Output values."]
    pub padout: crate::Reg<self::gpio::padout::PADOUT_SPEC>,
    #[doc = "0x0c - Interrupt enable per input bit. INTTYPE0 and INTTYPE1 control the interrupt triggering behavior. There are four triggers available - INTTYPE0 = 0, INTTYPE1 = 0: Level 1 - INTTYPE0 = 1, INTTYPE1 = 0: Level 0 - INTTYPE0 = 0, INTTYPE1 = 1: Rise - INTTYPE0 = 1, INTTYPE1 = 1: Fall"]
    pub inten: crate::Reg<self::gpio::inten::INTEN_SPEC>,
    #[doc = "0x10 - Interrupt Type 0. Controls the interrupt trigger behavior together with INTTYPE1. Use INTEN to enable interrupts first."]
    pub inttype0: crate::Reg<self::gpio::inttype0::INTTYPE0_SPEC>,
    #[doc = "0x14 - Controls the interrupt trigger behavior together with INTTYPE0. Use INTEN to enable interrupts first."]
    pub inttype1: crate::Reg<self::gpio::inttype1::INTTYPE1_SPEC>,
    #[doc = "0x18 - Contains interrupt status per GPIO line. The status register is cleared when read. Similarly the interrupt line is high while a bit is set in interrupt status and will be deasserted when the status register is read."]
    pub intstatus: crate::Reg<self::gpio::intstatus::INTSTATUS_SPEC>,
    #[doc = "0x1c - Contains the enable bit per GPIO line."]
    pub gpioen: crate::Reg<self::gpio::gpioen::GPIOEN_SPEC>,
    #[doc = "0x20 - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg0: crate::Reg<self::gpio::padcfg0::PADCFG0_SPEC>,
    #[doc = "0x24 - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg1: crate::Reg<self::gpio::padcfg1::PADCFG1_SPEC>,
    #[doc = "0x28 - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg2: crate::Reg<self::gpio::padcfg2::PADCFG2_SPEC>,
    #[doc = "0x2c - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg3: crate::Reg<self::gpio::padcfg3::PADCFG3_SPEC>,
    #[doc = "0x30 - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg4: crate::Reg<self::gpio::padcfg4::PADCFG4_SPEC>,
    #[doc = "0x34 - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg5: crate::Reg<self::gpio::padcfg5::PADCFG5_SPEC>,
    #[doc = "0x38 - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg6: crate::Reg<self::gpio::padcfg6::PADCFG6_SPEC>,
    #[doc = "0x3c - The pad configuration registers control various aspects of the pads that are typically used in ASICs, e.g. drive strength, Schmitt-Triggers, Slew Rate, etc. Since those configuration parameters depend on the exact pads used, each implementation is free to use the PADCFG0-7 registers in every way it wants and also leave them unconnected, if unneeded. 13 Writing to the PADOUTSET address (0x1A10_1040), the content of the PADOUT register is updated with its content \"ored\" with the write data. Writing to the PADOUTCLR address (0x1A10_1044), the content of the PADOUT register is updated with its content \"anded\" with the inverted write data."]
    pub padcfg7: crate::Reg<self::gpio::padcfg7::PADCFG7_SPEC>,
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
    pub reg_tx_ch0_add: crate::Reg<self::udma::reg_tx_ch0_add::REG_TX_CH0_ADD_SPEC>,
    #[doc = "0x404 - FILTER tx channel configuration register"]
    pub reg_tx_ch0_cfg: crate::Reg<self::udma::reg_tx_ch0_cfg::REG_TX_CH0_CFG_SPEC>,
    #[doc = "0x408 - FILTER tx channel length1 register"]
    pub reg_tx_ch0_len0: crate::Reg<self::udma::reg_tx_ch0_len0::REG_TX_CH0_LEN0_SPEC>,
    #[doc = "0x40c - FILTER tx channel length2 register"]
    pub reg_tx_ch0_len1: crate::Reg<self::udma::reg_tx_ch0_len1::REG_TX_CH0_LEN1_SPEC>,
    #[doc = "0x410 - FILTER tx channel 0 length2 register"]
    pub reg_tx_ch0_len2: crate::Reg<self::udma::reg_tx_ch0_len2::REG_TX_CH0_LEN2_SPEC>,
    #[doc = "0x414 - FILTER tx channel address register"]
    pub reg_tx_ch1_add: crate::Reg<self::udma::reg_tx_ch1_add::REG_TX_CH1_ADD_SPEC>,
    #[doc = "0x418 - FILTER tx channel configuration register"]
    pub reg_tx_ch1_cfg: crate::Reg<self::udma::reg_tx_ch1_cfg::REG_TX_CH1_CFG_SPEC>,
    #[doc = "0x41c - FILTER tx channel length1 register"]
    pub reg_tx_ch1_len0: crate::Reg<self::udma::reg_tx_ch1_len0::REG_TX_CH1_LEN0_SPEC>,
    #[doc = "0x420 - FILTER tx channel length2 register"]
    pub reg_tx_ch1_len1: crate::Reg<self::udma::reg_tx_ch1_len1::REG_TX_CH1_LEN1_SPEC>,
    #[doc = "0x424 - FILTER RX channel configuration register"]
    pub reg_tx_ch1_len2: crate::Reg<self::udma::reg_tx_ch1_len2::REG_TX_CH1_LEN2_SPEC>,
    #[doc = "0x428 - FILTER RX channel address register"]
    pub reg_rx_ch_add: crate::Reg<self::udma::reg_rx_ch_add::REG_RX_CH_ADD_SPEC>,
    #[doc = "0x42c - FILTER RX channel configuration register"]
    pub reg_rx_ch_cfg: crate::Reg<self::udma::reg_rx_ch_cfg::REG_RX_CH_CFG_SPEC>,
    #[doc = "0x430 - FILTER RX channel configuration register"]
    pub reg_rx_ch_len0: crate::Reg<self::udma::reg_rx_ch_len0::REG_RX_CH_LEN0_SPEC>,
    #[doc = "0x434 - FILTER RX channel length1 register"]
    pub reg_rx_ch_len1: crate::Reg<self::udma::reg_rx_ch_len1::REG_RX_CH_LEN1_SPEC>,
    #[doc = "0x438 - FILTER RX channel length2 register"]
    pub reg_rx_ch_len2: crate::Reg<self::udma::reg_rx_ch_len2::REG_RX_CH_LEN2_SPEC>,
    #[doc = "0x43c - FILTER arithmetic unit configuration register"]
    pub reg_au_cfg: crate::Reg<self::udma::reg_au_cfg::REG_AU_CFG_SPEC>,
    #[doc = "0x440 - FILTER arithmetic unit 0 register"]
    pub reg_au_reg0: crate::Reg<self::udma::reg_au_reg0::REG_AU_REG0_SPEC>,
    #[doc = "0x444 - FILTER arithmetic unit 1 register"]
    pub reg_au_reg1: crate::Reg<self::udma::reg_au_reg1::REG_AU_REG1_SPEC>,
    #[doc = "0x448 - FILTER binarization threshold register"]
    pub reg_bincu_th: crate::Reg<self::udma::reg_bincu_th::REG_BINCU_TH_SPEC>,
    #[doc = "0x44c - FILTER binarization count register"]
    pub reg_bincu_cnt: crate::Reg<self::udma::reg_bincu_cnt::REG_BINCU_CNT_SPEC>,
    #[doc = "0x450 - FILTER binarization result count register"]
    pub reg_bincu_setup: crate::Reg<self::udma::reg_bincu_setup::REG_BINCU_SETUP_SPEC>,
    #[doc = "0x454 - FILTER binarization result count register"]
    pub reg_bincu_val: crate::Reg<self::udma::reg_bincu_val::REG_BINCU_VAL_SPEC>,
    #[doc = "0x458 - FILTER control mode register"]
    pub reg_filt: crate::Reg<self::udma::reg_filt::REG_FILT_SPEC>,
    #[doc = "0x45c - FILTER start register"]
    pub reg_filt_cmd: crate::Reg<self::udma::reg_filt_cmd::REG_FILT_CMD_SPEC>,
    #[doc = "0x460 - FILTER status register"]
    pub reg_status: crate::Reg<self::udma::reg_status::REG_STATUS_SPEC>,
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
}
#[doc = r"Register block"]
#[doc = "SDIO"]
pub mod sdio;
#[doc = r"Register block"]
#[repr(C)]
pub struct SOCCONTROL_TTA_PLL {
    #[doc = "0x00 - This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register."]
    pub info: crate::Reg<self::soc_control_tta_pll::info::INFO_SPEC>,
    #[doc = "0x04 - This register holds the boot address."]
    pub fcboot: crate::Reg<self::soc_control_tta_pll::fcboot::FCBOOT_SPEC>,
    #[doc = "0x08 - This register contains the value of the fetch enable signal of the core."]
    pub fcfetch: crate::Reg<self::soc_control_tta_pll::fcfetch::FCFETCH_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\])."]
    pub pad_mux_0: crate::Reg<self::soc_control_tta_pll::pad_mux_0::PAD_MUX_0_SPEC>,
    #[doc = "0x14 - The content of these registers can be used to multiplex pads when targeting an ASIC. The second register (0x1A10_4014) can be used to sets the mux (2 bit select) from pin 16 (bits \\[1:0\\]) to 31 (bits \\[31:30\\])."]
    pub pad_mux_1: crate::Reg<self::soc_control_tta_pll::pad_mux_1::PAD_MUX_1_SPEC>,
    #[doc = "0x18 - The content of these registers can be used to multiplex pads when targeting an ASIC. The third register (0x1A10_4018) can be used to sets the mux (2 bit select) from pin 32 (bits \\[1:0\\]) to 47 (bits \\[31:30\\])."]
    pub pad_mux_2: crate::Reg<self::soc_control_tta_pll::pad_mux_2::PAD_MUX_2_SPEC>,
    #[doc = "0x1c - The content of these registers can be used to multiplex pads when targeting an ASIC. The forth register (0x1A10_401C) can be used to sets the mux (2 bit select) from pin 48 (bits \\[1:0\\]) to 63 (bits \\[31:30\\])."]
    pub pad_mux_3: crate::Reg<self::soc_control_tta_pll::pad_mux_3::PAD_MUX_3_SPEC>,
    #[doc = "0x20 - 6-bits per GPIO. Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_0: crate::Reg<self::soc_control_tta_pll::pad_cfg_0::PAD_CFG_0_SPEC>,
    #[doc = "0x24 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_1: crate::Reg<self::soc_control_tta_pll::pad_cfg_1::PAD_CFG_1_SPEC>,
    #[doc = "0x28 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_2: crate::Reg<self::soc_control_tta_pll::pad_cfg_2::PAD_CFG_2_SPEC>,
    #[doc = "0x2c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_3: crate::Reg<self::soc_control_tta_pll::pad_cfg_3::PAD_CFG_3_SPEC>,
    #[doc = "0x30 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_4: crate::Reg<self::soc_control_tta_pll::pad_cfg_4::PAD_CFG_4_SPEC>,
    #[doc = "0x34 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_5: crate::Reg<self::soc_control_tta_pll::pad_cfg_5::PAD_CFG_5_SPEC>,
    #[doc = "0x38 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_6: crate::Reg<self::soc_control_tta_pll::pad_cfg_6::PAD_CFG_6_SPEC>,
    #[doc = "0x3c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_7: crate::Reg<self::soc_control_tta_pll::pad_cfg_7::PAD_CFG_7_SPEC>,
    #[doc = "0x40 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_8: crate::Reg<self::soc_control_tta_pll::pad_cfg_8::PAD_CFG_8_SPEC>,
    #[doc = "0x44 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_9: crate::Reg<self::soc_control_tta_pll::pad_cfg_9::PAD_CFG_9_SPEC>,
    #[doc = "0x48 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_10: crate::Reg<self::soc_control_tta_pll::pad_cfg_10::PAD_CFG_10_SPEC>,
    #[doc = "0x4c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_11: crate::Reg<self::soc_control_tta_pll::pad_cfg_11::PAD_CFG_11_SPEC>,
    #[doc = "0x50 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_12: crate::Reg<self::soc_control_tta_pll::pad_cfg_12::PAD_CFG_12_SPEC>,
    #[doc = "0x54 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_13: crate::Reg<self::soc_control_tta_pll::pad_cfg_13::PAD_CFG_13_SPEC>,
    #[doc = "0x58 - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_14: crate::Reg<self::soc_control_tta_pll::pad_cfg_14::PAD_CFG_14_SPEC>,
    #[doc = "0x5c - Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
    pub pad_cfg_15: crate::Reg<self::soc_control_tta_pll::pad_cfg_15::PAD_CFG_15_SPEC>,
    #[doc = "0x60 - "]
    pub reg_tta_pll_loop_ctrl:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_loop_ctrl::REG_TTA_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x64 - "]
    pub reg_tta_pll_div:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_div::REG_TTA_PLL_DIV_SPEC>,
    #[doc = "0x68 - "]
    pub reg_tta_pll_debug_ctrl:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_debug_ctrl::REG_TTA_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x6c - "]
    pub reg_tta_pll_enable:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_enable::REG_TTA_PLL_ENABLE_SPEC>,
    #[doc = "0x70 - "]
    pub cluster_ctrl: crate::Reg<self::soc_control_tta_pll::cluster_ctrl::CLUSTER_CTRL_SPEC>,
    #[doc = "0x74 - Register to read or write from JTAG"]
    pub jtagreg: crate::Reg<self::soc_control_tta_pll::jtagreg::JTAGREG_SPEC>,
    #[doc = "0x78 - "]
    pub ctrl_per: crate::Reg<self::soc_control_tta_pll::ctrl_per::CTRL_PER_SPEC>,
    #[doc = "0x7c - "]
    pub cluster_irq: crate::Reg<self::soc_control_tta_pll::cluster_irq::CLUSTER_IRQ_SPEC>,
    #[doc = "0x80 - "]
    pub cluster_boot_addr0:
        crate::Reg<self::soc_control_tta_pll::cluster_boot_addr0::CLUSTER_BOOT_ADDR0_SPEC>,
    #[doc = "0x84 - "]
    pub cluster_boot_addr1:
        crate::Reg<self::soc_control_tta_pll::cluster_boot_addr1::CLUSTER_BOOT_ADDR1_SPEC>,
    #[doc = "0x88 - "]
    pub reg_tta_pll_spare_ctrl:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_spare_ctrl::REG_TTA_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x8c - "]
    pub reg_tta_pll_tmux_sel:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_tmux_sel::REG_TTA_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x90 - "]
    pub reg_tta_pll_status1:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_status1::REG_TTA_PLL_STATUS1_SPEC>,
    #[doc = "0x94 - "]
    pub reg_topperiph_clk_div:
        crate::Reg<self::soc_control_tta_pll::reg_topperiph_clk_div::REG_TOPPERIPH_CLK_DIV_SPEC>,
    #[doc = "0x98 - Subsystem Clock selection. Bit definition *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub reg_ss_clk_ctrl1:
        crate::Reg<self::soc_control_tta_pll::reg_ss_clk_ctrl1::REG_SS_CLK_CTRL1_SPEC>,
    #[doc = "0x9c - Subsystem Clock selection. Bit definition *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub reg_ss_clk_ctrl2:
        crate::Reg<self::soc_control_tta_pll::reg_ss_clk_ctrl2::REG_SS_CLK_CTRL2_SPEC>,
    #[doc = "0xa0 - These 2 registers (CORESTATUS, CS_RO) contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
    pub corestatus: crate::Reg<self::soc_control_tta_pll::corestatus::CORESTATUS_SPEC>,
    #[doc = "0xa4 - "]
    pub reg_slow_pulse_div:
        crate::Reg<self::soc_control_tta_pll::reg_slow_pulse_div::REG_SLOW_PULSE_DIV_SPEC>,
    #[doc = "0xa8 - "]
    pub reg_periph_clk_div:
        crate::Reg<self::soc_control_tta_pll::reg_periph_clk_div::REG_PERIPH_CLK_DIV_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0xb0 - Assertion of the Subsystem bit, deasserts the reset to the corresponding subsystem. Bit definition is mentioned in adjoining table SS Clock and Reset Enable layout: 0: Pulpissimo 4: interconnect 7: Top peripheral 8: c2c 12: CoreHW 16: TTA 20: Ethernet 24: AI 28: HPC Other bits unused"]
    pub reg_ss_reset_en:
        crate::Reg<self::soc_control_tta_pll::reg_ss_reset_en::REG_SS_RESET_EN_SPEC>,
    #[doc = "0xb4 - SS Clock and Reset Enable layout: 0: Pulpissimo 4: interconnect 7: Top peripheral 8: c2c 12: CoreHW 16: TTA 20: Ethernet 24: AI 28: HPC Other bits unused"]
    pub reg_ss_clk_en: crate::Reg<self::soc_control_tta_pll::reg_ss_clk_en::REG_SS_CLK_EN_SPEC>,
    #[doc = "0xb8 - Subsystem Clock selection. Bit definition *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub reg_ss_clk_ctrl3:
        crate::Reg<self::soc_control_tta_pll::reg_ss_clk_ctrl3::REG_SS_CLK_CTRL3_SPEC>,
    #[doc = "0xbc - "]
    pub reg_tta_pll_status2:
        crate::Reg<self::soc_control_tta_pll::reg_tta_pll_status2::REG_TTA_PLL_STATUS2_SPEC>,
    #[doc = "0xc0 - "]
    pub cs_ro: crate::Reg<self::soc_control_tta_pll::cs_ro::CS_RO_SPEC>,
    #[doc = "0xc4 - Boot Sel value"]
    pub bootsel: crate::Reg<self::soc_control_tta_pll::bootsel::BOOTSEL_SPEC>,
    #[doc = "0xc8 - "]
    pub clksel: crate::Reg<self::soc_control_tta_pll::clksel::CLKSEL_SPEC>,
    #[doc = "0xcc - Clock divider ratio for the 3 Interconnect modules"]
    pub reg_inter_clk_div:
        crate::Reg<self::soc_control_tta_pll::reg_inter_clk_div::REG_INTER_CLK_DIV_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SocControl_TTA_PLL"]
pub mod soc_control_tta_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct ETH_PLL {
    #[doc = "0x00 - "]
    pub reg_eth_pll_loop_ctrl:
        crate::Reg<self::eth_pll::reg_eth_pll_loop_ctrl::REG_ETH_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub reg_eth_pll_div: crate::Reg<self::eth_pll::reg_eth_pll_div::REG_ETH_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_eth_pll_debug_ctrl:
        crate::Reg<self::eth_pll::reg_eth_pll_debug_ctrl::REG_ETH_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub reg_eth_pll_enable: crate::Reg<self::eth_pll::reg_eth_pll_enable::REG_ETH_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_eth_pll_spare_ctrl:
        crate::Reg<self::eth_pll::reg_eth_pll_spare_ctrl::REG_ETH_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub reg_eth_pll_tmux_sel:
        crate::Reg<self::eth_pll::reg_eth_pll_tmux_sel::REG_ETH_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub reg_eth_pll_status1:
        crate::Reg<self::eth_pll::reg_eth_pll_status1::REG_ETH_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_eth_pll_status2:
        crate::Reg<self::eth_pll::reg_eth_pll_status2::REG_ETH_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "ETH_PLL"]
pub mod eth_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct AI_PLL {
    #[doc = "0x00 - "]
    pub reg_ai_pll_loop_ctrl:
        crate::Reg<self::ai_pll::reg_ai_pll_loop_ctrl::REG_AI_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub reg_ai_pll_div: crate::Reg<self::ai_pll::reg_ai_pll_div::REG_AI_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_ai_pll_debug_ctrl:
        crate::Reg<self::ai_pll::reg_ai_pll_debug_ctrl::REG_AI_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub reg_ai_pll_enable: crate::Reg<self::ai_pll::reg_ai_pll_enable::REG_AI_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_ai_pll_spare_ctrl:
        crate::Reg<self::ai_pll::reg_ai_pll_spare_ctrl::REG_AI_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub reg_ai_pll_tmux_sel:
        crate::Reg<self::ai_pll::reg_ai_pll_tmux_sel::REG_AI_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub reg_ai_pll_status1: crate::Reg<self::ai_pll::reg_ai_pll_status1::REG_AI_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_ai_pll_status2: crate::Reg<self::ai_pll::reg_ai_pll_status2::REG_AI_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "AI_PLL"]
pub mod ai_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct HPC_PLL {
    #[doc = "0x00 - "]
    pub reg_hpc_pll_loop_ctrl:
        crate::Reg<self::hpc_pll::reg_hpc_pll_loop_ctrl::REG_HPC_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub reg_hpc_pll_div: crate::Reg<self::hpc_pll::reg_hpc_pll_div::REG_HPC_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_hpc_pll_debug_ctrl:
        crate::Reg<self::hpc_pll::reg_hpc_pll_debug_ctrl::REG_HPC_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub reg_hpc_pll_enable: crate::Reg<self::hpc_pll::reg_hpc_pll_enable::REG_HPC_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_hpc_pll_spare_ctrl:
        crate::Reg<self::hpc_pll::reg_hpc_pll_spare_ctrl::REG_HPC_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub reg_hpc_pll_tmux_sel:
        crate::Reg<self::hpc_pll::reg_hpc_pll_tmux_sel::REG_HPC_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub reg_hpc_pll_status1:
        crate::Reg<self::hpc_pll::reg_hpc_pll_status1::REG_HPC_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_hpc_pll_status2:
        crate::Reg<self::hpc_pll::reg_hpc_pll_status2::REG_HPC_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HPC_PLL"]
pub mod hpc_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct PULP_PLL {
    #[doc = "0x00 - "]
    pub reg_pulp_pll_loop_ctrl:
        crate::Reg<self::pulp_pll::reg_pulp_pll_loop_ctrl::REG_PULP_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub reg_pulp_pll_div: crate::Reg<self::pulp_pll::reg_pulp_pll_div::REG_PULP_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_pulp_pll_debug_ctrl:
        crate::Reg<self::pulp_pll::reg_pulp_pll_debug_ctrl::REG_PULP_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub reg_pulp_pll_enable:
        crate::Reg<self::pulp_pll::reg_pulp_pll_enable::REG_PULP_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_pulp_pll_spare_ctrl:
        crate::Reg<self::pulp_pll::reg_pulp_pll_spare_ctrl::REG_PULP_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub reg_pulp_pll_tmux_sel:
        crate::Reg<self::pulp_pll::reg_pulp_pll_tmux_sel::REG_PULP_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub reg_pulp_pll_status1:
        crate::Reg<self::pulp_pll::reg_pulp_pll_status1::REG_PULP_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_pulp_pll_status2:
        crate::Reg<self::pulp_pll::reg_pulp_pll_status2::REG_PULP_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PULP_PLL"]
pub mod pulp_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct INTER_PLL {
    #[doc = "0x00 - "]
    pub reg_inter_pll_loop_ctrl:
        crate::Reg<self::inter_pll::reg_inter_pll_loop_ctrl::REG_INTER_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub reg_inter_pll_div: crate::Reg<self::inter_pll::reg_inter_pll_div::REG_INTER_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_inter_pll_debug_ctrl:
        crate::Reg<self::inter_pll::reg_inter_pll_debug_ctrl::REG_INTER_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub reg_inter_pll_enable:
        crate::Reg<self::inter_pll::reg_inter_pll_enable::REG_INTER_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_inter_pll_spare_ctrl:
        crate::Reg<self::inter_pll::reg_inter_pll_spare_ctrl::REG_INTER_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub reg_inter_pll_tmux_sel:
        crate::Reg<self::inter_pll::reg_inter_pll_tmux_sel::REG_INTER_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub reg_inter_pll_status1:
        crate::Reg<self::inter_pll::reg_inter_pll_status1::REG_INTER_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_inter_pll_status2:
        crate::Reg<self::inter_pll::reg_inter_pll_status2::REG_INTER_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "INTER_PLL"]
pub mod inter_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct C2C_PLL {
    #[doc = "0x00 - "]
    pub reg_c2c_pll_loop_ctrl:
        crate::Reg<self::c2c_pll::reg_c2c_pll_loop_ctrl::REG_C2C_PLL_LOOP_CTRL_SPEC>,
    #[doc = "0x04 - "]
    pub reg_c2c_pll_div: crate::Reg<self::c2c_pll::reg_c2c_pll_div::REG_C2C_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_c2c_pll_debug_ctrl:
        crate::Reg<self::c2c_pll::reg_c2c_pll_debug_ctrl::REG_C2C_PLL_DEBUG_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub reg_c2c_pll_enable: crate::Reg<self::c2c_pll::reg_c2c_pll_enable::REG_C2C_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_c2c_pll_spare_ctrl:
        crate::Reg<self::c2c_pll::reg_c2c_pll_spare_ctrl::REG_C2C_PLL_SPARE_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub reg_c2c_pll_tmux_sel:
        crate::Reg<self::c2c_pll::reg_c2c_pll_tmux_sel::REG_C2C_PLL_TMUX_SEL_SPEC>,
    #[doc = "0x18 - "]
    pub reg_c2c_pll_status1:
        crate::Reg<self::c2c_pll::reg_c2c_pll_status1::REG_C2C_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_c2c_pll_status2:
        crate::Reg<self::c2c_pll::reg_c2c_pll_status2::REG_C2C_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "C2C_PLL"]
pub mod c2c_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct TOPPERIPH_PLL {
    #[doc = "0x00 - "]
    pub reg_topperiph_pll_loop_ctrl: crate::Reg<
        self::topperiph_pll::reg_topperiph_pll_loop_ctrl::REG_TOPPERIPH_PLL_LOOP_CTRL_SPEC,
    >,
    #[doc = "0x04 - "]
    pub reg_topperiph_pll_div:
        crate::Reg<self::topperiph_pll::reg_topperiph_pll_div::REG_TOPPERIPH_PLL_DIV_SPEC>,
    #[doc = "0x08 - "]
    pub reg_topperiph_pll_debug_ctrl: crate::Reg<
        self::topperiph_pll::reg_topperiph_pll_debug_ctrl::REG_TOPPERIPH_PLL_DEBUG_CTRL_SPEC,
    >,
    #[doc = "0x0c - "]
    pub reg_topperiph_pll_enable:
        crate::Reg<self::topperiph_pll::reg_topperiph_pll_enable::REG_TOPPERIPH_PLL_ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub reg_topperiph_pll_spare_ctrl: crate::Reg<
        self::topperiph_pll::reg_topperiph_pll_spare_ctrl::REG_TOPPERIPH_PLL_SPARE_CTRL_SPEC,
    >,
    #[doc = "0x14 - "]
    pub reg_topperiph_pll_tmux_sel: crate::Reg<
        self::topperiph_pll::reg_topperiph_pll_tmux_sel::REG_TOPPERIPH_PLL_TMUX_SEL_SPEC,
    >,
    #[doc = "0x18 - "]
    pub reg_topperiph_pll_status1:
        crate::Reg<self::topperiph_pll::reg_topperiph_pll_status1::REG_TOPPERIPH_PLL_STATUS1_SPEC>,
    #[doc = "0x1c - "]
    pub reg_topperiph_pll_status2:
        crate::Reg<self::topperiph_pll::reg_topperiph_pll_status2::REG_TOPPERIPH_PLL_STATUS2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "TOPPERIPH_PLL"]
pub mod topperiph_pll;
#[doc = r"Register block"]
#[repr(C)]
pub struct BOOTCONFIG {
    #[doc = "0x00 - "]
    pub reg_boot_cfg: crate::Reg<self::boot_config::reg_boot_cfg::REG_BOOT_CFG_SPEC>,
    #[doc = "0x04 - "]
    pub reg_boot_status: crate::Reg<self::boot_config::reg_boot_status::REG_BOOT_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "BootConfig"]
pub mod boot_config;
