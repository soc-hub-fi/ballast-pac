#[doc = r"Register block"]
#[repr(C)]
pub struct C2C_REMAP {
    #[doc = "0x00 - "]
    pub c2c_remap_tx_addr: C2C_REMAP_TX_ADDR,
    #[doc = "0x04 - "]
    pub c2c_remap_tx_mask: C2C_REMAP_TX_MASK,
    #[doc = "0x08 - "]
    pub c2c_remap_rx_addr: C2C_REMAP_RX_ADDR,
    #[doc = "0x0c - "]
    pub c2c_remap_rx_mask: C2C_REMAP_RX_MASK,
}
#[doc = "c2c_remap_tx_addr (rw) register accessor: an alias for `Reg<C2C_REMAP_TX_ADDR_SPEC>`"]
pub type C2C_REMAP_TX_ADDR = crate::Reg<c2c_remap_tx_addr::C2C_REMAP_TX_ADDR_SPEC>;
#[doc = ""]
pub mod c2c_remap_tx_addr;
#[doc = "c2c_remap_tx_mask (rw) register accessor: an alias for `Reg<C2C_REMAP_TX_MASK_SPEC>`"]
pub type C2C_REMAP_TX_MASK = crate::Reg<c2c_remap_tx_mask::C2C_REMAP_TX_MASK_SPEC>;
#[doc = ""]
pub mod c2c_remap_tx_mask;
#[doc = "c2c_remap_rx_addr (rw) register accessor: an alias for `Reg<C2C_REMAP_RX_ADDR_SPEC>`"]
pub type C2C_REMAP_RX_ADDR = crate::Reg<c2c_remap_rx_addr::C2C_REMAP_RX_ADDR_SPEC>;
#[doc = ""]
pub mod c2c_remap_rx_addr;
#[doc = "c2c_remap_rx_mask (rw) register accessor: an alias for `Reg<C2C_REMAP_RX_MASK_SPEC>`"]
pub type C2C_REMAP_RX_MASK = crate::Reg<c2c_remap_rx_mask::C2C_REMAP_RX_MASK_SPEC>;
#[doc = ""]
pub mod c2c_remap_rx_mask;
