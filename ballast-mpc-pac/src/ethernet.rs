#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x8c - MAC_registers"]
    pub mac_registers: MAC_REGISTERS,
    #[doc = "0x8c..0xa8 - MDIO"]
    pub mdio: MDIO,
    _reserved2: [u8; 0x0158],
    #[doc = "0x200..0x230 - DMA_registers"]
    pub dma_registers: DMA_REGISTERS,
}
#[doc = "MAC_registers"]
pub use self::mac_registers::MAC_REGISTERS;
#[doc = r"Cluster"]
#[doc = "MAC_registers"]
pub mod mac_registers;
#[doc = "DMA_registers"]
pub use self::dma_registers::DMA_REGISTERS;
#[doc = r"Cluster"]
#[doc = "DMA_registers"]
pub mod dma_registers;
#[doc = "MDIO"]
pub use self::mdio::MDIO;
#[doc = r"Cluster"]
#[doc = "MDIO"]
pub mod mdio;
