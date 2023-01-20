#[doc = r"Register block"]
#[repr(C)]
pub struct MDIO {
    #[doc = "0x00 - Acts as input to MDIO clock divider. MDIO clock is derived from Reg_clk (125/2 MHz) by dividing it by this value."]
    pub mdio_divider: MDIO_DIVIDER,
    #[doc = "0x04 - Data to write over MDIO. Used whenever a write operation is intiated with a write to the MDIO_Ctrl register."]
    pub mdio_wr_data: MDIO_WR_DATA,
    #[doc = "0x08 - MDIO register address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register."]
    pub mdio_reg_addr: MDIO_REG_ADDR,
    #[doc = "0x0c - MDIO PHY address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register"]
    pub mdio_phy_addr: MDIO_PHY_ADDR,
    #[doc = "0x10 - Three-bit bitfield. Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used. Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts. Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts. Bits 1 and 2 should not be set simultaneously."]
    pub mdio_ctrl: MDIO_CTRL,
    #[doc = "0x14 - After MDIO read transaction has completed, this register is updated with the read result."]
    pub mdio_rd_data: MDIO_RD_DATA,
    #[doc = "0x18 - Single-bit register, reads as 1 if MDIO transfer is ongoing."]
    pub mdio_status: MDIO_STATUS,
}
#[doc = "MDIO_Divider (rw) register accessor: an alias for `Reg<MDIO_DIVIDER_SPEC>`"]
pub type MDIO_DIVIDER = crate::Reg<mdio_divider::MDIO_DIVIDER_SPEC>;
#[doc = "Acts as input to MDIO clock divider. MDIO clock is derived from Reg_clk (125/2 MHz) by dividing it by this value."]
pub mod mdio_divider;
#[doc = "MDIO_WrData (rw) register accessor: an alias for `Reg<MDIO_WR_DATA_SPEC>`"]
pub type MDIO_WR_DATA = crate::Reg<mdio_wr_data::MDIO_WR_DATA_SPEC>;
#[doc = "Data to write over MDIO. Used whenever a write operation is intiated with a write to the MDIO_Ctrl register."]
pub mod mdio_wr_data;
#[doc = "MDIO_RegAddr (rw) register accessor: an alias for `Reg<MDIO_REG_ADDR_SPEC>`"]
pub type MDIO_REG_ADDR = crate::Reg<mdio_reg_addr::MDIO_REG_ADDR_SPEC>;
#[doc = "MDIO register address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register."]
pub mod mdio_reg_addr;
#[doc = "MDIO_PhyAddr (rw) register accessor: an alias for `Reg<MDIO_PHY_ADDR_SPEC>`"]
pub type MDIO_PHY_ADDR = crate::Reg<mdio_phy_addr::MDIO_PHY_ADDR_SPEC>;
#[doc = "MDIO PHY address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register"]
pub mod mdio_phy_addr;
#[doc = "MDIO_Ctrl (rw) register accessor: an alias for `Reg<MDIO_CTRL_SPEC>`"]
pub type MDIO_CTRL = crate::Reg<mdio_ctrl::MDIO_CTRL_SPEC>;
#[doc = "Three-bit bitfield. Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used. Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts. Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts. Bits 1 and 2 should not be set simultaneously."]
pub mod mdio_ctrl;
#[doc = "MDIO_RdData (r) register accessor: an alias for `Reg<MDIO_RD_DATA_SPEC>`"]
pub type MDIO_RD_DATA = crate::Reg<mdio_rd_data::MDIO_RD_DATA_SPEC>;
#[doc = "After MDIO read transaction has completed, this register is updated with the read result."]
pub mod mdio_rd_data;
#[doc = "MDIO_Status (r) register accessor: an alias for `Reg<MDIO_STATUS_SPEC>`"]
pub type MDIO_STATUS = crate::Reg<mdio_status::MDIO_STATUS_SPEC>;
#[doc = "Single-bit register, reads as 1 if MDIO transfer is ongoing."]
pub mod mdio_status;
