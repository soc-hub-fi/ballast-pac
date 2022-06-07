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
#[doc = "SPIM_RX_CFG register accessor: an alias for `Reg<SPIM_RX_CFG_SPEC>`"]
pub type SPIM_RX_CFG = crate::Reg<spim_rx_cfg::SPIM_RX_CFG_SPEC>;
#[doc = "RX SPI uDMA transfer configuration"]
pub mod spim_rx_cfg;
#[doc = "TX_CH0_LEN1 register accessor: an alias for `Reg<TX_CH0_LEN1_SPEC>`"]
pub type TX_CH0_LEN1 = crate::Reg<tx_ch0_len1::TX_CH0_LEN1_SPEC>;
#[doc = "FILTER tx channel length2 register"]
pub mod tx_ch0_len1;
#[doc = "TX_CH0_CFG register accessor: an alias for `Reg<TX_CH0_CFG_SPEC>`"]
pub type TX_CH0_CFG = crate::Reg<tx_ch0_cfg::TX_CH0_CFG_SPEC>;
#[doc = "FILTER tx channel configuration register"]
pub mod tx_ch0_cfg;
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
