#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0a - Apb_software_irq"]
    pub apb_software_irq: APB_SOFTWARE_IRQ,
    _reserved1: [u8; 0x01f6],
    #[doc = "0x200..0x24d - Global_intr_router"]
    pub global_intr_router: GLOBAL_INTR_ROUTER,
    _reserved2: [u8; 0x01b3],
    #[doc = "0x400..0x410 - C2C_remap"]
    pub c2c_remap: C2C_REMAP,
    _reserved3: [u8; 0x01f0],
    #[doc = "0x600..0x640 - Pad_config"]
    pub pad_config: PAD_CONFIG,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct APB_SOFTWARE_IRQ {
    #[doc = "0x00 - "]
    pub software_irq_read:
        crate::Reg<self::apb_software_irq::software_irq_read::SOFTWARE_IRQ_READ_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - "]
    pub software_irq_set:
        crate::Reg<self::apb_software_irq::software_irq_set::SOFTWARE_IRQ_SET_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - "]
    pub software_irq_clear:
        crate::Reg<self::apb_software_irq::software_irq_clear::SOFTWARE_IRQ_CLEAR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Apb_software_irq"]
pub mod apb_software_irq;
#[doc = r"Register block"]
#[repr(C)]
pub struct GLOBAL_INTR_ROUTER {
    #[doc = "0x00 - "]
    pub irq_0_route_reg_0:
        crate::Reg<self::global_intr_router::irq_0_route_reg_0::IRQ_0_ROUTE_REG_0_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - "]
    pub irq_1_route_reg_0:
        crate::Reg<self::global_intr_router::irq_1_route_reg_0::IRQ_1_ROUTE_REG_0_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - "]
    pub irq_2_route_reg_0:
        crate::Reg<self::global_intr_router::irq_2_route_reg_0::IRQ_2_ROUTE_REG_0_SPEC>,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - "]
    pub irq_0_route_reg_1:
        crate::Reg<self::global_intr_router::irq_0_route_reg_1::IRQ_0_ROUTE_REG_1_SPEC>,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - "]
    pub irq_1_route_reg_1:
        crate::Reg<self::global_intr_router::irq_1_route_reg_1::IRQ_1_ROUTE_REG_1_SPEC>,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - "]
    pub irq_2_route_reg_1:
        crate::Reg<self::global_intr_router::irq_2_route_reg_1::IRQ_2_ROUTE_REG_1_SPEC>,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - "]
    pub irq_0_route_reg_2:
        crate::Reg<self::global_intr_router::irq_0_route_reg_2::IRQ_0_ROUTE_REG_2_SPEC>,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - "]
    pub irq_1_route_reg_2:
        crate::Reg<self::global_intr_router::irq_1_route_reg_2::IRQ_1_ROUTE_REG_2_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - "]
    pub irq_2_route_reg_2:
        crate::Reg<self::global_intr_router::irq_2_route_reg_2::IRQ_2_ROUTE_REG_2_SPEC>,
    _reserved9: [u8; 0x07],
    #[doc = "0x28 - "]
    pub irq_1_route_reg_3:
        crate::Reg<self::global_intr_router::irq_1_route_reg_3::IRQ_1_ROUTE_REG_3_SPEC>,
    _reserved10: [u8; 0x03],
    #[doc = "0x2c - "]
    pub irq_2_route_reg_3:
        crate::Reg<self::global_intr_router::irq_2_route_reg_3::IRQ_2_ROUTE_REG_3_SPEC>,
    _reserved11: [u8; 0x07],
    #[doc = "0x34 - "]
    pub irq_1_route_reg_4:
        crate::Reg<self::global_intr_router::irq_1_route_reg_4::IRQ_1_ROUTE_REG_4_SPEC>,
    _reserved12: [u8; 0x0b],
    #[doc = "0x40 - "]
    pub irq_1_route_reg_5:
        crate::Reg<self::global_intr_router::irq_1_route_reg_5::IRQ_1_ROUTE_REG_5_SPEC>,
    _reserved13: [u8; 0x0b],
    #[doc = "0x4c - "]
    pub irq_1_route_reg_6:
        crate::Reg<self::global_intr_router::irq_1_route_reg_6::IRQ_1_ROUTE_REG_6_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Global_intr_router"]
pub mod global_intr_router;
#[doc = r"Register block"]
#[repr(C)]
pub struct C2C_REMAP {
    #[doc = "0x00 - "]
    pub c2c_remap_tx_addr: crate::Reg<self::c2c_remap::c2c_remap_tx_addr::C2C_REMAP_TX_ADDR_SPEC>,
    #[doc = "0x04 - "]
    pub c2c_remap_tx_mask: crate::Reg<self::c2c_remap::c2c_remap_tx_mask::C2C_REMAP_TX_MASK_SPEC>,
    #[doc = "0x08 - "]
    pub c2c_remap_rx_addr: crate::Reg<self::c2c_remap::c2c_remap_rx_addr::C2C_REMAP_RX_ADDR_SPEC>,
    #[doc = "0x0c - "]
    pub c2c_remap_rx_mask: crate::Reg<self::c2c_remap::c2c_remap_rx_mask::C2C_REMAP_RX_MASK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "C2C_remap"]
pub mod c2c_remap;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_CONFIG {
    #[doc = "0x00 - "]
    pub eth_rx_clk: crate::Reg<self::pad_config::eth_rx_clk::ETH_RX_CLK_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - "]
    pub eth_rxd: crate::Reg<self::pad_config::eth_rxd::ETH_RXD_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - "]
    pub eth_rx_ctl: crate::Reg<self::pad_config::eth_rx_ctl::ETH_RX_CTL_SPEC>,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - "]
    pub eth_tx_clk: crate::Reg<self::pad_config::eth_tx_clk::ETH_TX_CLK_SPEC>,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - "]
    pub eth_txd: crate::Reg<self::pad_config::eth_txd::ETH_TXD_SPEC>,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - "]
    pub eth_tx_ctl: crate::Reg<self::pad_config::eth_tx_ctl::ETH_TX_CTL_SPEC>,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - "]
    pub eth_mdc: crate::Reg<self::pad_config::eth_mdc::ETH_MDC_SPEC>,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - "]
    pub eth_mdio: crate::Reg<self::pad_config::eth_mdio::ETH_MDIO_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - "]
    pub c2c_tx_ack: crate::Reg<self::pad_config::c2c_tx_ack::C2C_TX_ACK_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - "]
    pub c2c_tx_vld: crate::Reg<self::pad_config::c2c_tx_vld::C2C_TX_VLD_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - "]
    pub c2c_tx_d: crate::Reg<self::pad_config::c2c_tx_d::C2C_TX_D_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - "]
    pub c2c_rx_ack: crate::Reg<self::pad_config::c2c_rx_ack::C2C_RX_ACK_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - "]
    pub c2c_rx_vld: crate::Reg<self::pad_config::c2c_rx_vld::C2C_RX_VLD_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x34 - "]
    pub c2c_rx_d: crate::Reg<self::pad_config::c2c_rx_d::C2C_RX_D_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x38 - "]
    pub rst_mstr: crate::Reg<self::pad_config::rst_mstr::RST_MSTR_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - "]
    pub rst_slv: crate::Reg<self::pad_config::rst_slv::RST_SLV_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - "]
    pub refclk: crate::Reg<self::pad_config::refclk::REFCLK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Pad_config"]
pub mod pad_config;
