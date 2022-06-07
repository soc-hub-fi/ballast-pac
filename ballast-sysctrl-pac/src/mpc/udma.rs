#[doc = "CTRL_CFG_CG register accessor: an alias for `Reg<CTRL_CFG_CG_SPEC>`"]
pub type CTRL_CFG_CG = crate::Reg<ctrl_cfg_cg::CTRL_CFG_CG_SPEC>;
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral"]
pub mod ctrl_cfg_cg;
#[doc = "CTRL_CFG_EVENT register accessor: an alias for `Reg<CTRL_CFG_EVENT_SPEC>`"]
pub type CTRL_CFG_EVENT = crate::Reg<ctrl_cfg_event::CTRL_CFG_EVENT_SPEC>;
#[doc = "uDMA peripherals external event configuration"]
pub mod ctrl_cfg_event;
#[doc = "CTRL_CFG_RST register accessor: an alias for `Reg<CTRL_CFG_RST_SPEC>`"]
pub type CTRL_CFG_RST = crate::Reg<ctrl_cfg_rst::CTRL_CFG_RST_SPEC>;
#[doc = "uDMA peripherals reset trigger (unimplemented)"]
pub mod ctrl_cfg_rst;
#[doc = "UART_RX_SADDR register accessor: an alias for `Reg<UART_RX_SADDR_SPEC>`"]
pub type UART_RX_SADDR = crate::Reg<uart_rx_saddr::UART_RX_SADDR_SPEC>;
#[doc = "uDMA RX UART buffer base address configuration register"]
pub mod uart_rx_saddr;
#[doc = "UART_RX_SIZE register accessor: an alias for `Reg<UART_RX_SIZE_SPEC>`"]
pub type UART_RX_SIZE = crate::Reg<uart_rx_size::UART_RX_SIZE_SPEC>;
#[doc = "uDMA RX UART buffer size configuration register"]
pub mod uart_rx_size;
#[doc = "UART_RX_CFG register accessor: an alias for `Reg<UART_RX_CFG_SPEC>`"]
pub type UART_RX_CFG = crate::Reg<uart_rx_cfg::UART_RX_CFG_SPEC>;
#[doc = "uDMA RX UART stream configuration register"]
pub mod uart_rx_cfg;
#[doc = "UART_TX_SADDR register accessor: an alias for `Reg<UART_TX_SADDR_SPEC>`"]
pub type UART_TX_SADDR = crate::Reg<uart_tx_saddr::UART_TX_SADDR_SPEC>;
#[doc = "uDMA TX UART buffer base address configuration register."]
pub mod uart_tx_saddr;
#[doc = "UART_TX_SIZE register accessor: an alias for `Reg<UART_TX_SIZE_SPEC>`"]
pub type UART_TX_SIZE = crate::Reg<uart_tx_size::UART_TX_SIZE_SPEC>;
#[doc = "uDMA TX UART buffer size configuration register"]
pub mod uart_tx_size;
#[doc = "UART_TX_CFG register accessor: an alias for `Reg<UART_TX_CFG_SPEC>`"]
pub type UART_TX_CFG = crate::Reg<uart_tx_cfg::UART_TX_CFG_SPEC>;
#[doc = "uDMA TX UART stream configuration register."]
pub mod uart_tx_cfg;
#[doc = "UART_ERROR register accessor: an alias for `Reg<UART_ERROR_SPEC>`"]
pub type UART_ERROR = crate::Reg<uart_error::UART_ERROR_SPEC>;
#[doc = "uDMA UART Error status"]
pub mod uart_error;
#[doc = "SPIM_RX_SIZE register accessor: an alias for `Reg<SPIM_RX_SIZE_SPEC>`"]
pub type SPIM_RX_SIZE = crate::Reg<spim_rx_size::SPIM_RX_SIZE_SPEC>;
#[doc = "RX SPI uDMA transfer size of buffer"]
pub mod spim_rx_size;
#[doc = "UART_DATA register accessor: an alias for `Reg<UART_DATA_SPEC>`"]
pub type UART_DATA = crate::Reg<uart_data::UART_DATA_SPEC>;
#[doc = "RX read data for polling or interrupt"]
pub mod uart_data;
#[doc = "SPIM_RX_SADDR register accessor: an alias for `Reg<SPIM_RX_SADDR_SPEC>`"]
pub type SPIM_RX_SADDR = crate::Reg<spim_rx_saddr::SPIM_RX_SADDR_SPEC>;
#[doc = "RX SPI uDMA transfer address of associated buffer"]
pub mod spim_rx_saddr;
#[doc = "UART_STATUS register accessor: an alias for `Reg<UART_STATUS_SPEC>`"]
pub type UART_STATUS = crate::Reg<uart_status::UART_STATUS_SPEC>;
#[doc = "uDMA UART status register"]
pub mod uart_status;
#[doc = "UART_IRQ_EN register accessor: an alias for `Reg<UART_IRQ_EN_SPEC>`"]
pub type UART_IRQ_EN = crate::Reg<uart_irq_en::UART_IRQ_EN_SPEC>;
#[doc = "uDMA UART Read or Error interrupt enable register"]
pub mod uart_irq_en;
#[doc = "UART_VALID register accessor: an alias for `Reg<UART_VALID_SPEC>`"]
pub type UART_VALID = crate::Reg<uart_valid::UART_VALID_SPEC>;
#[doc = "uDMA UART Read polling data valid flag register"]
pub mod uart_valid;
#[doc = "SPIM_CMD_SADDR register accessor: an alias for `Reg<SPIM_CMD_SADDR_SPEC>`"]
pub type SPIM_CMD_SADDR = crate::Reg<spim_cmd_saddr::SPIM_CMD_SADDR_SPEC>;
#[doc = "CMD SPI uDMA transfer address of associated buffer"]
pub mod spim_cmd_saddr;
#[doc = "SPIM_TX_SADDR register accessor: an alias for `Reg<SPIM_TX_SADDR_SPEC>`"]
pub type SPIM_TX_SADDR = crate::Reg<spim_tx_saddr::SPIM_TX_SADDR_SPEC>;
#[doc = "TX SPI uDMA transfer address of associated buffer"]
pub mod spim_tx_saddr;
#[doc = "SPIM_CMD_SIZE register accessor: an alias for `Reg<SPIM_CMD_SIZE_SPEC>`"]
pub type SPIM_CMD_SIZE = crate::Reg<spim_cmd_size::SPIM_CMD_SIZE_SPEC>;
#[doc = "CMD SPI uDMA transfer size of buffer"]
pub mod spim_cmd_size;
#[doc = "SPIM_TX_SIZE register accessor: an alias for `Reg<SPIM_TX_SIZE_SPEC>`"]
pub type SPIM_TX_SIZE = crate::Reg<spim_tx_size::SPIM_TX_SIZE_SPEC>;
#[doc = "TX SPI uDMA transfer size of buffer"]
pub mod spim_tx_size;
#[doc = "I2C0_TX_SADDR register accessor: an alias for `Reg<I2C0_TX_SADDR_SPEC>`"]
pub type I2C0_TX_SADDR = crate::Reg<i2c0_tx_saddr::I2C0_TX_SADDR_SPEC>;
#[doc = "uDMA TX I2C buffer base address configuration register."]
pub mod i2c0_tx_saddr;
#[doc = "I2C0_SETUP register accessor: an alias for `Reg<I2C0_SETUP_SPEC>`"]
pub type I2C0_SETUP = crate::Reg<i2c0_setup::I2C0_SETUP_SPEC>;
#[doc = "uDMA I2C Configuration register."]
pub mod i2c0_setup;
#[doc = "I2C0_CMD_CFG register accessor: an alias for `Reg<I2C0_CMD_CFG_SPEC>`"]
pub type I2C0_CMD_CFG = crate::Reg<i2c0_cmd_cfg::I2C0_CMD_CFG_SPEC>;
#[doc = "uDMA CMD I2C stream configuration register."]
pub mod i2c0_cmd_cfg;
#[doc = "I2C0_CMD_SADDR register accessor: an alias for `Reg<I2C0_CMD_SADDR_SPEC>`"]
pub type I2C0_CMD_SADDR = crate::Reg<i2c0_cmd_saddr::I2C0_CMD_SADDR_SPEC>;
#[doc = "uDMA CMD I2C buffer base address configuration register."]
pub mod i2c0_cmd_saddr;
#[doc = "I2C0_TX_SIZE register accessor: an alias for `Reg<I2C0_TX_SIZE_SPEC>`"]
pub type I2C0_TX_SIZE = crate::Reg<i2c0_tx_size::I2C0_TX_SIZE_SPEC>;
#[doc = "uDMA TX I2C buffer size configuration register"]
pub mod i2c0_tx_size;
#[doc = "I2C0_RX_SIZE register accessor: an alias for `Reg<I2C0_RX_SIZE_SPEC>`"]
pub type I2C0_RX_SIZE = crate::Reg<i2c0_rx_size::I2C0_RX_SIZE_SPEC>;
#[doc = "uDMA RX I2C buffer size configuration register"]
pub mod i2c0_rx_size;
#[doc = "SPIM_CMD_CFG register accessor: an alias for `Reg<SPIM_CMD_CFG_SPEC>`"]
pub type SPIM_CMD_CFG = crate::Reg<spim_cmd_cfg::SPIM_CMD_CFG_SPEC>;
#[doc = "CMD SPI uDMA transfer configuration"]
pub mod spim_cmd_cfg;
#[doc = "UART_SETUP register accessor: an alias for `Reg<UART_SETUP_SPEC>`"]
pub type UART_SETUP = crate::Reg<uart_setup::UART_SETUP_SPEC>;
#[doc = "UDMA UART configuration register."]
pub mod uart_setup;
#[doc = "SPIM_TX_CFG register accessor: an alias for `Reg<SPIM_TX_CFG_SPEC>`"]
pub type SPIM_TX_CFG = crate::Reg<spim_tx_cfg::SPIM_TX_CFG_SPEC>;
#[doc = "TX SPI uDMA transfer configuration"]
pub mod spim_tx_cfg;
#[doc = "I2C0_STATUS register accessor: an alias for `Reg<I2C0_STATUS_SPEC>`"]
pub type I2C0_STATUS = crate::Reg<i2c0_status::I2C0_STATUS_SPEC>;
#[doc = "uDMA I2C Status register"]
pub mod i2c0_status;
#[doc = "I2C0_CMD_SIZE register accessor: an alias for `Reg<I2C0_CMD_SIZE_SPEC>`"]
pub type I2C0_CMD_SIZE = crate::Reg<i2c0_cmd_size::I2C0_CMD_SIZE_SPEC>;
#[doc = "uDMA CMD I2C buffer size configuration register"]
pub mod i2c0_cmd_size;
#[doc = "I2C0_TX_CFG register accessor: an alias for `Reg<I2C0_TX_CFG_SPEC>`"]
pub type I2C0_TX_CFG = crate::Reg<i2c0_tx_cfg::I2C0_TX_CFG_SPEC>;
#[doc = "uDMA TX I2C stream configuration register"]
pub mod i2c0_tx_cfg;
#[doc = "I2C0_RX_CFG register accessor: an alias for `Reg<I2C0_RX_CFG_SPEC>`"]
pub type I2C0_RX_CFG = crate::Reg<i2c0_rx_cfg::I2C0_RX_CFG_SPEC>;
#[doc = "uDMA RX I2C stream configuration register."]
pub mod i2c0_rx_cfg;
#[doc = "I2C0_RX_SADDR register accessor: an alias for `Reg<I2C0_RX_SADDR_SPEC>`"]
pub type I2C0_RX_SADDR = crate::Reg<i2c0_rx_saddr::I2C0_RX_SADDR_SPEC>;
#[doc = "uDMA RX I2C buffer base address configuration register."]
pub mod i2c0_rx_saddr;
#[doc = "SPIM_RX_CFG register accessor: an alias for `Reg<SPIM_RX_CFG_SPEC>`"]
pub type SPIM_RX_CFG = crate::Reg<spim_rx_cfg::SPIM_RX_CFG_SPEC>;
#[doc = "RX SPI uDMA transfer configuration"]
pub mod spim_rx_cfg;
#[doc = "I2C1_RX_SADDR register accessor: an alias for `Reg<I2C1_RX_SADDR_SPEC>`"]
pub type I2C1_RX_SADDR = crate::Reg<i2c1_rx_saddr::I2C1_RX_SADDR_SPEC>;
#[doc = "uDMA RX I2C buffer base address configuration register."]
pub mod i2c1_rx_saddr;
#[doc = "I2C1_RX_SIZE register accessor: an alias for `Reg<I2C1_RX_SIZE_SPEC>`"]
pub type I2C1_RX_SIZE = crate::Reg<i2c1_rx_size::I2C1_RX_SIZE_SPEC>;
#[doc = "uDMA RX I2C buffer size configuration register"]
pub mod i2c1_rx_size;
#[doc = "I2C1_RX_CFG register accessor: an alias for `Reg<I2C1_RX_CFG_SPEC>`"]
pub type I2C1_RX_CFG = crate::Reg<i2c1_rx_cfg::I2C1_RX_CFG_SPEC>;
#[doc = "uDMA RX I2C stream configuration register."]
pub mod i2c1_rx_cfg;
#[doc = "I2C1_TX_SADDR register accessor: an alias for `Reg<I2C1_TX_SADDR_SPEC>`"]
pub type I2C1_TX_SADDR = crate::Reg<i2c1_tx_saddr::I2C1_TX_SADDR_SPEC>;
#[doc = "uDMA TX I2C buffer base address configuration register."]
pub mod i2c1_tx_saddr;
#[doc = "I2C1_TX_SIZE register accessor: an alias for `Reg<I2C1_TX_SIZE_SPEC>`"]
pub type I2C1_TX_SIZE = crate::Reg<i2c1_tx_size::I2C1_TX_SIZE_SPEC>;
#[doc = "uDMA TX I2C buffer size configuration register"]
pub mod i2c1_tx_size;
#[doc = "I2C1_TX_CFG register accessor: an alias for `Reg<I2C1_TX_CFG_SPEC>`"]
pub type I2C1_TX_CFG = crate::Reg<i2c1_tx_cfg::I2C1_TX_CFG_SPEC>;
#[doc = "uDMA TX I2C stream configuration register"]
pub mod i2c1_tx_cfg;
#[doc = "I2C1_CMD_SADDR register accessor: an alias for `Reg<I2C1_CMD_SADDR_SPEC>`"]
pub type I2C1_CMD_SADDR = crate::Reg<i2c1_cmd_saddr::I2C1_CMD_SADDR_SPEC>;
#[doc = "uDMA CMD I2C buffer base address configuration register."]
pub mod i2c1_cmd_saddr;
#[doc = "I2C1_CMD_SIZE register accessor: an alias for `Reg<I2C1_CMD_SIZE_SPEC>`"]
pub type I2C1_CMD_SIZE = crate::Reg<i2c1_cmd_size::I2C1_CMD_SIZE_SPEC>;
#[doc = "uDMA CMD I2C buffer size configuration register"]
pub mod i2c1_cmd_size;
#[doc = "I2C1_CMD_CFG register accessor: an alias for `Reg<I2C1_CMD_CFG_SPEC>`"]
pub type I2C1_CMD_CFG = crate::Reg<i2c1_cmd_cfg::I2C1_CMD_CFG_SPEC>;
#[doc = "uDMA CMD I2C stream configuration register."]
pub mod i2c1_cmd_cfg;
#[doc = "I2C1_STATUS register accessor: an alias for `Reg<I2C1_STATUS_SPEC>`"]
pub type I2C1_STATUS = crate::Reg<i2c1_status::I2C1_STATUS_SPEC>;
#[doc = "uDMA I2C Status register"]
pub mod i2c1_status;
#[doc = "CAM_CFG_FILTER register accessor: an alias for `Reg<CAM_CFG_FILTER_SPEC>`"]
pub type CAM_CFG_FILTER = crate::Reg<cam_cfg_filter::CAM_CFG_FILTER_SPEC>;
#[doc = "RGB coefficients configuration register"]
pub mod cam_cfg_filter;
#[doc = "CAM_CFG_UR register accessor: an alias for `Reg<CAM_CFG_UR_SPEC>`"]
pub type CAM_CFG_UR = crate::Reg<cam_cfg_ur::CAM_CFG_UR_SPEC>;
#[doc = "Upper Right corner configuration register"]
pub mod cam_cfg_ur;
#[doc = "CAM_CFG_GLOB register accessor: an alias for `Reg<CAM_CFG_GLOB_SPEC>`"]
pub type CAM_CFG_GLOB = crate::Reg<cam_cfg_glob::CAM_CFG_GLOB_SPEC>;
#[doc = "Global configuration register"]
pub mod cam_cfg_glob;
#[doc = "CAM_RX_SIZE register accessor: an alias for `Reg<CAM_RX_SIZE_SPEC>`"]
pub type CAM_RX_SIZE = crate::Reg<cam_rx_size::CAM_RX_SIZE_SPEC>;
#[doc = "RX Camera uDMA transfer size of buffer register"]
pub mod cam_rx_size;
#[doc = "I2S_PDM_SETUP register accessor: an alias for `Reg<I2S_PDM_SETUP_SPEC>`"]
pub type I2S_PDM_SETUP = crate::Reg<i2s_pdm_setup::I2S_PDM_SETUP_SPEC>;
#[doc = "Configuration of PDM module"]
pub mod i2s_pdm_setup;
#[doc = "TX_CH0_LEN1 register accessor: an alias for `Reg<TX_CH0_LEN1_SPEC>`"]
pub type TX_CH0_LEN1 = crate::Reg<tx_ch0_len1::TX_CH0_LEN1_SPEC>;
#[doc = "FILTER tx channel length2 register"]
pub mod tx_ch0_len1;
#[doc = "TX_CH0_CFG register accessor: an alias for `Reg<TX_CH0_CFG_SPEC>`"]
pub type TX_CH0_CFG = crate::Reg<tx_ch0_cfg::TX_CH0_CFG_SPEC>;
#[doc = "FILTER tx channel configuration register"]
pub mod tx_ch0_cfg;
#[doc = "CAM_VSYNC_POLARITY register accessor: an alias for `Reg<CAM_VSYNC_POLARITY_SPEC>`"]
pub type CAM_VSYNC_POLARITY = crate::Reg<cam_vsync_polarity::CAM_VSYNC_POLARITY_SPEC>;
#[doc = "VSYNC Polarity register"]
pub mod cam_vsync_polarity;
#[doc = "TX_CH0_LEN2 register accessor: an alias for `Reg<TX_CH0_LEN2_SPEC>`"]
pub type TX_CH0_LEN2 = crate::Reg<tx_ch0_len2::TX_CH0_LEN2_SPEC>;
#[doc = "FILTER tx channel 0 length2 register"]
pub mod tx_ch0_len2;
#[doc = "TX_CH0_LEN0 register accessor: an alias for `Reg<TX_CH0_LEN0_SPEC>`"]
pub type TX_CH0_LEN0 = crate::Reg<tx_ch0_len0::TX_CH0_LEN0_SPEC>;
#[doc = "FILTER tx channel length1 register"]
pub mod tx_ch0_len0;
#[doc = "TX_CH0_ADD register accessor: an alias for `Reg<TX_CH0_ADD_SPEC>`"]
pub type TX_CH0_ADD = crate::Reg<tx_ch0_add::TX_CH0_ADD_SPEC>;
#[doc = "FILTER tx channel address register"]
pub mod tx_ch0_add;
#[doc = "CAM_CFG_SIZE register accessor: an alias for `Reg<CAM_CFG_SIZE_SPEC>`"]
pub type CAM_CFG_SIZE = crate::Reg<cam_cfg_size::CAM_CFG_SIZE_SPEC>;
#[doc = "Horizontal Resolution configuration register"]
pub mod cam_cfg_size;
#[doc = "CAM_CFG_LL register accessor: an alias for `Reg<CAM_CFG_LL_SPEC>`"]
pub type CAM_CFG_LL = crate::Reg<cam_cfg_ll::CAM_CFG_LL_SPEC>;
#[doc = "Lower Left corner configuration register"]
pub mod cam_cfg_ll;
#[doc = "CAM_RX_CFG register accessor: an alias for `Reg<CAM_RX_CFG_SPEC>`"]
pub type CAM_RX_CFG = crate::Reg<cam_rx_cfg::CAM_RX_CFG_SPEC>;
#[doc = "RX Camera uDMA transfer configuration register"]
pub mod cam_rx_cfg;
#[doc = "CAM_RX_SADDR register accessor: an alias for `Reg<CAM_RX_SADDR_SPEC>`"]
pub type CAM_RX_SADDR = crate::Reg<cam_rx_saddr::CAM_RX_SADDR_SPEC>;
#[doc = "RX Camera uDMA transfer address of associated buffer register"]
pub mod cam_rx_saddr;
#[doc = "I2S_SLV_SETUP register accessor: an alias for `Reg<I2S_SLV_SETUP_SPEC>`"]
pub type I2S_SLV_SETUP = crate::Reg<i2s_slv_setup::I2S_SLV_SETUP_SPEC>;
#[doc = "Configuration of I2S slave"]
pub mod i2s_slv_setup;
#[doc = "I2S_TX_CFG register accessor: an alias for `Reg<I2S_TX_CFG_SPEC>`"]
pub type I2S_TX_CFG = crate::Reg<i2s_tx_cfg::I2S_TX_CFG_SPEC>;
#[doc = "TX Channel I2S uDMA transfer configuration"]
pub mod i2s_tx_cfg;
#[doc = "I2S_TX_SADDR register accessor: an alias for `Reg<I2S_TX_SADDR_SPEC>`"]
pub type I2S_TX_SADDR = crate::Reg<i2s_tx_saddr::I2S_TX_SADDR_SPEC>;
#[doc = "TX Channel I2S uDMA transfer address of associated buffer"]
pub mod i2s_tx_saddr;
#[doc = "I2S_RX_SIZE register accessor: an alias for `Reg<I2S_RX_SIZE_SPEC>`"]
pub type I2S_RX_SIZE = crate::Reg<i2s_rx_size::I2S_RX_SIZE_SPEC>;
#[doc = "RX Channel 0 I2S uDMA transfer size of buffer"]
pub mod i2s_rx_size;
#[doc = "I2C1_SETUP register accessor: an alias for `Reg<I2C1_SETUP_SPEC>`"]
pub type I2C1_SETUP = crate::Reg<i2c1_setup::I2C1_SETUP_SPEC>;
#[doc = "uDMA I2C Configuration register."]
pub mod i2c1_setup;
#[doc = "I2S_MST_SETUP register accessor: an alias for `Reg<I2S_MST_SETUP_SPEC>`"]
pub type I2S_MST_SETUP = crate::Reg<i2s_mst_setup::I2S_MST_SETUP_SPEC>;
#[doc = "Configuration of I2S master"]
pub mod i2s_mst_setup;
#[doc = "I2S_CLKCFG_SETUP register accessor: an alias for `Reg<I2S_CLKCFG_SETUP_SPEC>`"]
pub type I2S_CLKCFG_SETUP = crate::Reg<i2s_clkcfg_setup::I2S_CLKCFG_SETUP_SPEC>;
#[doc = "Clock configuration for both master, slave and pdm"]
pub mod i2s_clkcfg_setup;
#[doc = "I2S_TX_SIZE register accessor: an alias for `Reg<I2S_TX_SIZE_SPEC>`"]
pub type I2S_TX_SIZE = crate::Reg<i2s_tx_size::I2S_TX_SIZE_SPEC>;
#[doc = "TX Channel I2S uDMA transfer size of buffer"]
pub mod i2s_tx_size;
#[doc = "I2S_RX_CFG register accessor: an alias for `Reg<I2S_RX_CFG_SPEC>`"]
pub type I2S_RX_CFG = crate::Reg<i2s_rx_cfg::I2S_RX_CFG_SPEC>;
#[doc = "RX Channel 0 I2S uDMA transfer configuration"]
pub mod i2s_rx_cfg;
#[doc = "I2S_RX_SADDR register accessor: an alias for `Reg<I2S_RX_SADDR_SPEC>`"]
pub type I2S_RX_SADDR = crate::Reg<i2s_rx_saddr::I2S_RX_SADDR_SPEC>;
#[doc = "RX Channel0 I2S uDMA transfer address of associated buffer"]
pub mod i2s_rx_saddr;
#[doc = "TX_CH1_ADD register accessor: an alias for `Reg<TX_CH1_ADD_SPEC>`"]
pub type TX_CH1_ADD = crate::Reg<tx_ch1_add::TX_CH1_ADD_SPEC>;
#[doc = "FILTER tx channel address register"]
pub mod tx_ch1_add;
#[doc = "TX_CH1_CFG register accessor: an alias for `Reg<TX_CH1_CFG_SPEC>`"]
pub type TX_CH1_CFG = crate::Reg<tx_ch1_cfg::TX_CH1_CFG_SPEC>;
#[doc = "FILTER tx channel configuration register"]
pub mod tx_ch1_cfg;
#[doc = "TX_CH1_LEN0 register accessor: an alias for `Reg<TX_CH1_LEN0_SPEC>`"]
pub type TX_CH1_LEN0 = crate::Reg<tx_ch1_len0::TX_CH1_LEN0_SPEC>;
#[doc = "FILTER tx channel length1 register"]
pub mod tx_ch1_len0;
#[doc = "TX_CH1_LEN1 register accessor: an alias for `Reg<TX_CH1_LEN1_SPEC>`"]
pub type TX_CH1_LEN1 = crate::Reg<tx_ch1_len1::TX_CH1_LEN1_SPEC>;
#[doc = "FILTER tx channel length2 register"]
pub mod tx_ch1_len1;
#[doc = "FILT_CMD register accessor: an alias for `Reg<FILT_CMD_SPEC>`"]
pub type FILT_CMD = crate::Reg<filt_cmd::FILT_CMD_SPEC>;
#[doc = "FILTER start register"]
pub mod filt_cmd;
#[doc = "BINCU_VAL register accessor: an alias for `Reg<BINCU_VAL_SPEC>`"]
pub type BINCU_VAL = crate::Reg<bincu_val::BINCU_VAL_SPEC>;
#[doc = "FILTER binarization result count register"]
pub mod bincu_val;
#[doc = "BINCU_CNT register accessor: an alias for `Reg<BINCU_CNT_SPEC>`"]
pub type BINCU_CNT = crate::Reg<bincu_cnt::BINCU_CNT_SPEC>;
#[doc = "FILTER binarization count register"]
pub mod bincu_cnt;
#[doc = "AU_REG1 register accessor: an alias for `Reg<AU_REG1_SPEC>`"]
pub type AU_REG1 = crate::Reg<au_reg1::AU_REG1_SPEC>;
#[doc = "FILTER arithmetic unit 1 register"]
pub mod au_reg1;
#[doc = "AU_CFG register accessor: an alias for `Reg<AU_CFG_SPEC>`"]
pub type AU_CFG = crate::Reg<au_cfg::AU_CFG_SPEC>;
#[doc = "FILTER arithmetic unit configuration register"]
pub mod au_cfg;
#[doc = "RX_CH_LEN1 register accessor: an alias for `Reg<RX_CH_LEN1_SPEC>`"]
pub type RX_CH_LEN1 = crate::Reg<rx_ch_len1::RX_CH_LEN1_SPEC>;
#[doc = "FILTER RX channel length1 register"]
pub mod rx_ch_len1;
#[doc = "RX_CH_CFG register accessor: an alias for `Reg<RX_CH_CFG_SPEC>`"]
pub type RX_CH_CFG = crate::Reg<rx_ch_cfg::RX_CH_CFG_SPEC>;
#[doc = "FILTER RX channel configuration register"]
pub mod rx_ch_cfg;
#[doc = "TX_CH1_LEN2 register accessor: an alias for `Reg<TX_CH1_LEN2_SPEC>`"]
pub type TX_CH1_LEN2 = crate::Reg<tx_ch1_len2::TX_CH1_LEN2_SPEC>;
#[doc = "FILTER RX channel configuration register"]
pub mod tx_ch1_len2;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "FILTER status register"]
pub mod status;
#[doc = "FILT register accessor: an alias for `Reg<FILT_SPEC>`"]
pub type FILT = crate::Reg<filt::FILT_SPEC>;
#[doc = "FILTER control mode register"]
pub mod filt;
#[doc = "BINCU_SETUP register accessor: an alias for `Reg<BINCU_SETUP_SPEC>`"]
pub type BINCU_SETUP = crate::Reg<bincu_setup::BINCU_SETUP_SPEC>;
#[doc = "FILTER binarization result count register"]
pub mod bincu_setup;
#[doc = "BINCU_TH register accessor: an alias for `Reg<BINCU_TH_SPEC>`"]
pub type BINCU_TH = crate::Reg<bincu_th::BINCU_TH_SPEC>;
#[doc = "FILTER binarization threshold register"]
pub mod bincu_th;
#[doc = "AU_REG0 register accessor: an alias for `Reg<AU_REG0_SPEC>`"]
pub type AU_REG0 = crate::Reg<au_reg0::AU_REG0_SPEC>;
#[doc = "FILTER arithmetic unit 0 register"]
pub mod au_reg0;
#[doc = "RX_CH_LEN2 register accessor: an alias for `Reg<RX_CH_LEN2_SPEC>`"]
pub type RX_CH_LEN2 = crate::Reg<rx_ch_len2::RX_CH_LEN2_SPEC>;
#[doc = "FILTER RX channel length2 register"]
pub mod rx_ch_len2;
#[doc = "RX_CH_LEN0 register accessor: an alias for `Reg<RX_CH_LEN0_SPEC>`"]
pub type RX_CH_LEN0 = crate::Reg<rx_ch_len0::RX_CH_LEN0_SPEC>;
#[doc = "FILTER RX channel configuration register"]
pub mod rx_ch_len0;
#[doc = "RX_CH_ADD register accessor: an alias for `Reg<RX_CH_ADD_SPEC>`"]
pub type RX_CH_ADD = crate::Reg<rx_ch_add::RX_CH_ADD_SPEC>;
#[doc = "FILTER RX channel address register"]
pub mod rx_ch_add;
