#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_CONFIG {
    #[doc = "0x00 - "]
    pub eth_rx_clk: ETH_RX_CLK,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - "]
    pub eth_rxd: ETH_RXD,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - "]
    pub eth_rx_ctl: ETH_RX_CTL,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - "]
    pub eth_tx_clk: ETH_TX_CLK,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - "]
    pub eth_txd: ETH_TXD,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - "]
    pub eth_tx_ctl: ETH_TX_CTL,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - "]
    pub eth_mdc: ETH_MDC,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - "]
    pub eth_mdio: ETH_MDIO,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - "]
    pub c2c_tx_ack: C2C_TX_ACK,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - "]
    pub c2c_tx_vld: C2C_TX_VLD,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - "]
    pub c2c_tx_d: C2C_TX_D,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - "]
    pub c2c_rx_ack: C2C_RX_ACK,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - "]
    pub c2c_rx_vld: C2C_RX_VLD,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - "]
    pub c2c_rx_d: C2C_RX_D,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - "]
    pub rst_mstr: RST_MSTR,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - "]
    pub rst_slv: RST_SLV,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - "]
    pub refclk: REFCLK,
}
#[doc = "eth_rx_clk (rw) register accessor: an alias for `Reg<ETH_RX_CLK_SPEC>`"]
pub type ETH_RX_CLK = crate::Reg<eth_rx_clk::ETH_RX_CLK_SPEC>;
#[doc = ""]
pub mod eth_rx_clk;
#[doc = "eth_rxd (rw) register accessor: an alias for `Reg<ETH_RXD_SPEC>`"]
pub type ETH_RXD = crate::Reg<eth_rxd::ETH_RXD_SPEC>;
#[doc = ""]
pub mod eth_rxd;
#[doc = "eth_rx_ctl (rw) register accessor: an alias for `Reg<ETH_RX_CTL_SPEC>`"]
pub type ETH_RX_CTL = crate::Reg<eth_rx_ctl::ETH_RX_CTL_SPEC>;
#[doc = ""]
pub mod eth_rx_ctl;
#[doc = "eth_tx_clk (rw) register accessor: an alias for `Reg<ETH_TX_CLK_SPEC>`"]
pub type ETH_TX_CLK = crate::Reg<eth_tx_clk::ETH_TX_CLK_SPEC>;
#[doc = ""]
pub mod eth_tx_clk;
#[doc = "eth_txd (rw) register accessor: an alias for `Reg<ETH_TXD_SPEC>`"]
pub type ETH_TXD = crate::Reg<eth_txd::ETH_TXD_SPEC>;
#[doc = ""]
pub mod eth_txd;
#[doc = "eth_tx_ctl (rw) register accessor: an alias for `Reg<ETH_TX_CTL_SPEC>`"]
pub type ETH_TX_CTL = crate::Reg<eth_tx_ctl::ETH_TX_CTL_SPEC>;
#[doc = ""]
pub mod eth_tx_ctl;
#[doc = "eth_mdc (rw) register accessor: an alias for `Reg<ETH_MDC_SPEC>`"]
pub type ETH_MDC = crate::Reg<eth_mdc::ETH_MDC_SPEC>;
#[doc = ""]
pub mod eth_mdc;
#[doc = "eth_mdio (rw) register accessor: an alias for `Reg<ETH_MDIO_SPEC>`"]
pub type ETH_MDIO = crate::Reg<eth_mdio::ETH_MDIO_SPEC>;
#[doc = ""]
pub mod eth_mdio;
#[doc = "c2c_tx_ack (rw) register accessor: an alias for `Reg<C2C_TX_ACK_SPEC>`"]
pub type C2C_TX_ACK = crate::Reg<c2c_tx_ack::C2C_TX_ACK_SPEC>;
#[doc = ""]
pub mod c2c_tx_ack;
#[doc = "c2c_tx_vld (rw) register accessor: an alias for `Reg<C2C_TX_VLD_SPEC>`"]
pub type C2C_TX_VLD = crate::Reg<c2c_tx_vld::C2C_TX_VLD_SPEC>;
#[doc = ""]
pub mod c2c_tx_vld;
#[doc = "c2c_tx_d (rw) register accessor: an alias for `Reg<C2C_TX_D_SPEC>`"]
pub type C2C_TX_D = crate::Reg<c2c_tx_d::C2C_TX_D_SPEC>;
#[doc = ""]
pub mod c2c_tx_d;
#[doc = "c2c_rx_ack (rw) register accessor: an alias for `Reg<C2C_RX_ACK_SPEC>`"]
pub type C2C_RX_ACK = crate::Reg<c2c_rx_ack::C2C_RX_ACK_SPEC>;
#[doc = ""]
pub mod c2c_rx_ack;
#[doc = "c2c_rx_vld (rw) register accessor: an alias for `Reg<C2C_RX_VLD_SPEC>`"]
pub type C2C_RX_VLD = crate::Reg<c2c_rx_vld::C2C_RX_VLD_SPEC>;
#[doc = ""]
pub mod c2c_rx_vld;
#[doc = "c2c_rx_d (rw) register accessor: an alias for `Reg<C2C_RX_D_SPEC>`"]
pub type C2C_RX_D = crate::Reg<c2c_rx_d::C2C_RX_D_SPEC>;
#[doc = ""]
pub mod c2c_rx_d;
#[doc = "refclk (rw) register accessor: an alias for `Reg<REFCLK_SPEC>`"]
pub type REFCLK = crate::Reg<refclk::REFCLK_SPEC>;
#[doc = ""]
pub mod refclk;
#[doc = "rst_mstr (rw) register accessor: an alias for `Reg<RST_MSTR_SPEC>`"]
pub type RST_MSTR = crate::Reg<rst_mstr::RST_MSTR_SPEC>;
#[doc = ""]
pub mod rst_mstr;
#[doc = "rst_slv (rw) register accessor: an alias for `Reg<RST_SLV_SPEC>`"]
pub type RST_SLV = crate::Reg<rst_slv::RST_SLV_SPEC>;
#[doc = ""]
pub mod rst_slv;
