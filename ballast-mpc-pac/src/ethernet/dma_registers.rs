#[doc = r"Register block"]
#[repr(C)]
pub struct DMA_REGISTERS {
    #[doc = "0x00 - rx_rb_start_address, tx_rb_start_address These registers store the start address for the ring buffer used by the DMA to read from or write to. Writing to these registers has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) or the read pointer (for TX) to the start of the ring buffer. Therefore, the associated size register (rx_rb_size or tx_rb_size) should be set first. The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register."]
    pub rx_rb_start_address: RX_RB_START_ADDRESS,
    #[doc = "0x04 - Stores the ring buffer size for the associated DMA channel in bytes."]
    pub rx_rb_size: RX_RB_SIZE,
    #[doc = "0x08 - Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer."]
    pub rx_rb_rd_pointer: RX_RB_RD_POINTER,
    #[doc = "0x0c - Data access to the package length FIFO. Length in bytes. rx_pkg_lgth is read-only. The DMA write channel pushes packet lengths to the FIFO after the corresponding AXI transfers for the packet have finished. Reading this register when the associated FIFO is not empty removes the first-written piece of data from the FIFO."]
    pub rx_pkg_lgth: RX_PKG_LGTH,
    #[doc = "0x10 - Read-only register denoting how many packet lengths are available or yet to be processed in the corresponding FIFO."]
    pub rx_pkg_lgth_cnt: RX_PKG_LGTH_CNT,
    #[doc = "0x14 - Register to store the start address for the ring buffer used by the DMA to write to. Writing to the register has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) to the start of the ring buffer. Therefore, the associated size register (tx_rb_size should be set first The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register."]
    pub tx_rb_start_addr: TX_RB_START_ADDR,
    #[doc = "0x18 - Stores the ring buffer size for the associated DMA channel in bytes."]
    pub tx_rb_size: TX_RB_SIZE,
    #[doc = "0x1c - Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer."]
    pub tx_rb_rd_pointer: TX_RB_RD_POINTER,
    #[doc = "0x20 - Data access to the package length FIFO. Packet lengths in bytes. tx_pkg_lgth is write-only. To issue a packet to be sent by the MAC, the host processor writes the ethernet frame data to the ring buffer, starting from a 4-byte aligned address immediately after the end of the previous packet, and writes the length of the packet to tx_pkg_lgth, in this order."]
    pub tx_pkg_lgth: TX_PKG_LGTH,
    #[doc = "0x24 - Read-only registers denoting how many packet lengths are available or yet to be processed in the corresponding FIFO."]
    pub tx_pkg_lgth_cnt: TX_PKG_LGTH_CNT,
    #[doc = "0x28 - One bit corresponds to an interrupt line from the MAC/DMA. When an interrupt is raised and its corresponding bit in irq_mask is set, its corresponding bit in irq is set and the external IRQ line is asserted. The IRQ line is deasserted when all bits in the irq register are unset by a write by the processor. In other words, the external IRQ line is an OR-reduction of the irq register. Writes to the irq register cannot assert new interrupt requests; in other words, the value stored in the irq register is the old contents combined with the write value by a bitwise AND."]
    pub irq: IRQ,
    #[doc = "0x2c - "]
    pub irq_mask: IRQ_MASK,
}
#[doc = "irq (rw) register accessor: an alias for `Reg<IRQ_SPEC>`"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "One bit corresponds to an interrupt line from the MAC/DMA. When an interrupt is raised and its corresponding bit in irq_mask is set, its corresponding bit in irq is set and the external IRQ line is asserted. The IRQ line is deasserted when all bits in the irq register are unset by a write by the processor. In other words, the external IRQ line is an OR-reduction of the irq register. Writes to the irq register cannot assert new interrupt requests; in other words, the value stored in the irq register is the old contents combined with the write value by a bitwise AND."]
pub mod irq;
#[doc = "irq_mask (rw) register accessor: an alias for `Reg<IRQ_MASK_SPEC>`"]
pub type IRQ_MASK = crate::Reg<irq_mask::IRQ_MASK_SPEC>;
#[doc = ""]
pub mod irq_mask;
#[doc = "rx_pkg_lgth (r) register accessor: an alias for `Reg<RX_PKG_LGTH_SPEC>`"]
pub type RX_PKG_LGTH = crate::Reg<rx_pkg_lgth::RX_PKG_LGTH_SPEC>;
#[doc = "Data access to the package length FIFO. Length in bytes. rx_pkg_lgth is read-only. The DMA write channel pushes packet lengths to the FIFO after the corresponding AXI transfers for the packet have finished. Reading this register when the associated FIFO is not empty removes the first-written piece of data from the FIFO."]
pub mod rx_pkg_lgth;
#[doc = "rx_pkg_lgth_cnt (r) register accessor: an alias for `Reg<RX_PKG_LGTH_CNT_SPEC>`"]
pub type RX_PKG_LGTH_CNT = crate::Reg<rx_pkg_lgth_cnt::RX_PKG_LGTH_CNT_SPEC>;
#[doc = "Read-only register denoting how many packet lengths are available or yet to be processed in the corresponding FIFO."]
pub mod rx_pkg_lgth_cnt;
#[doc = "rx_rb_rd_pointer (rw) register accessor: an alias for `Reg<RX_RB_RD_POINTER_SPEC>`"]
pub type RX_RB_RD_POINTER = crate::Reg<rx_rb_rd_pointer::RX_RB_RD_POINTER_SPEC>;
#[doc = "Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer."]
pub mod rx_rb_rd_pointer;
#[doc = "rx_rb_size (rw) register accessor: an alias for `Reg<RX_RB_SIZE_SPEC>`"]
pub type RX_RB_SIZE = crate::Reg<rx_rb_size::RX_RB_SIZE_SPEC>;
#[doc = "Stores the ring buffer size for the associated DMA channel in bytes."]
pub mod rx_rb_size;
#[doc = "rx_rb_start_address (rw) register accessor: an alias for `Reg<RX_RB_START_ADDRESS_SPEC>`"]
pub type RX_RB_START_ADDRESS = crate::Reg<rx_rb_start_address::RX_RB_START_ADDRESS_SPEC>;
#[doc = "rx_rb_start_address, tx_rb_start_address These registers store the start address for the ring buffer used by the DMA to read from or write to. Writing to these registers has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) or the read pointer (for TX) to the start of the ring buffer. Therefore, the associated size register (rx_rb_size or tx_rb_size) should be set first. The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register."]
pub mod rx_rb_start_address;
#[doc = "tx_pkg_lgth (w) register accessor: an alias for `Reg<TX_PKG_LGTH_SPEC>`"]
pub type TX_PKG_LGTH = crate::Reg<tx_pkg_lgth::TX_PKG_LGTH_SPEC>;
#[doc = "Data access to the package length FIFO. Packet lengths in bytes. tx_pkg_lgth is write-only. To issue a packet to be sent by the MAC, the host processor writes the ethernet frame data to the ring buffer, starting from a 4-byte aligned address immediately after the end of the previous packet, and writes the length of the packet to tx_pkg_lgth, in this order."]
pub mod tx_pkg_lgth;
#[doc = "tx_pkg_lgth_cnt (r) register accessor: an alias for `Reg<TX_PKG_LGTH_CNT_SPEC>`"]
pub type TX_PKG_LGTH_CNT = crate::Reg<tx_pkg_lgth_cnt::TX_PKG_LGTH_CNT_SPEC>;
#[doc = "Read-only registers denoting how many packet lengths are available or yet to be processed in the corresponding FIFO."]
pub mod tx_pkg_lgth_cnt;
#[doc = "tx_rb_start_addr (rw) register accessor: an alias for `Reg<TX_RB_START_ADDR_SPEC>`"]
pub type TX_RB_START_ADDR = crate::Reg<tx_rb_start_addr::TX_RB_START_ADDR_SPEC>;
#[doc = "Register to store the start address for the ring buffer used by the DMA to write to. Writing to the register has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) to the start of the ring buffer. Therefore, the associated size register (tx_rb_size should be set first The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register."]
pub mod tx_rb_start_addr;
#[doc = "tx_rb_size (rw) register accessor: an alias for `Reg<TX_RB_SIZE_SPEC>`"]
pub type TX_RB_SIZE = crate::Reg<tx_rb_size::TX_RB_SIZE_SPEC>;
#[doc = "Stores the ring buffer size for the associated DMA channel in bytes."]
pub mod tx_rb_size;
#[doc = "tx_rb_rd_pointer (r) register accessor: an alias for `Reg<TX_RB_RD_POINTER_SPEC>`"]
pub type TX_RB_RD_POINTER = crate::Reg<tx_rb_rd_pointer::TX_RB_RD_POINTER_SPEC>;
#[doc = "Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer."]
pub mod tx_rb_rd_pointer;
