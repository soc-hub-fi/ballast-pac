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
#[doc = r"Register block"]
#[repr(C)]
pub struct MAC_REGISTERS {
    #[doc = "0x00 - used to set transmit Fifo high water mark"]
    pub tx_hwmark: crate::Reg<self::mac_registers::tx_hwmark::TX_HWMARK_SPEC>,
    #[doc = "0x04 - used to set transmit Fifo low water mark"]
    pub tx_lwmark: crate::Reg<self::mac_registers::tx_lwmark::TX_LWMARK_SPEC>,
    #[doc = "0x08 - pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
    pub pause_frame_send_en:
        crate::Reg<self::mac_registers::pause_frame_send_en::PAUSE_FRAME_SEND_EN_SPEC>,
    #[doc = "0x0c - pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
    pub pause_quanta_set: crate::Reg<self::mac_registers::pause_quanta_set::PAUSE_QUANTA_SET_SPEC>,
    #[doc = "0x10 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    pub ifgset: crate::Reg<self::mac_registers::ifgset::IFGSET_SPEC>,
    #[doc = "0x14 - This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
    pub full_duplex: crate::Reg<self::mac_registers::full_duplex::FULLDUPLEX_SPEC>,
    #[doc = "0x18 - When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
    pub max_retry: crate::Reg<self::mac_registers::max_retry::MAXRETRY_SPEC>,
    #[doc = "0x1c - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    pub mac_tx_add_en: crate::Reg<self::mac_registers::mac_tx_add_en::MAC_TX_ADD_EN_SPEC>,
    #[doc = "0x20 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    pub mac_tx_add_prom_data:
        crate::Reg<self::mac_registers::mac_tx_add_prom_data::MAC_TX_ADD_PROM_DATA_SPEC>,
    #[doc = "0x24 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    pub mac_tx_add_prom_add:
        crate::Reg<self::mac_registers::mac_tx_add_prom_add::MAC_TX_ADD_PROM_ADD_SPEC>,
    #[doc = "0x28 - Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
    pub mac_tx_add_prom_wr:
        crate::Reg<self::mac_registers::mac_tx_add_prom_wr::MAC_TX_ADD_PROM_WR_SPEC>,
    #[doc = "0x2c - When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data."]
    pub tx_pause_en: crate::Reg<self::mac_registers::tx_pause_en::TX_PAUSE_EN_SPEC>,
    #[doc = "0x30 - The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state."]
    pub xoff_cpu: crate::Reg<self::mac_registers::xoff_cpu::XOFF_CPU_SPEC>,
    #[doc = "0x34 - The rising pulse of xon_cpu signal is used to start transmit one PAUSE frame with quanta value of pause_quanta_set when the transmit in idle state."]
    pub xon_cpu: crate::Reg<self::mac_registers::xon_cpu::XON_CPU_SPEC>,
    #[doc = "0x38 - "]
    pub mac_rx_add_chk_en:
        crate::Reg<self::mac_registers::mac_rx_add_chk_en::MAC_RX_ADD_CHK_EN_SPEC>,
    #[doc = "0x3c - "]
    pub mac_rx_add_prom_data:
        crate::Reg<self::mac_registers::mac_rx_add_prom_data::MAC_RX_ADD_PROM_DATA_SPEC>,
    #[doc = "0x40 - "]
    pub mac_rx_add_prom_add:
        crate::Reg<self::mac_registers::mac_rx_add_prom_add::MAC_RX_ADD_PROM_ADD_SPEC>,
    #[doc = "0x44 - "]
    pub mac_rx_add_prom_wr:
        crate::Reg<self::mac_registers::mac_rx_add_prom_wr::MAC_RX_ADD_PROM_WR_SPEC>,
    #[doc = "0x48 - The broadcast packet filter will enable only when broadcast_filter_en is set"]
    pub broadcast_filter_en:
        crate::Reg<self::mac_registers::broadcast_filter_en::BROADCAST_FILTER_EN_SPEC>,
    #[doc = "0x4c - broadcast_bucket_depth register is used to setting the bucket depth"]
    pub broadcast_bucket_depth:
        crate::Reg<self::mac_registers::broadcast_bucket_depth::BROADCAST_BUCKET_DEPTH_SPEC>,
    #[doc = "0x50 - The bucket wil be periodically refilled after broadcast_bucket_interval time"]
    pub broadcast_bucket_interval:
        crate::Reg<self::mac_registers::broadcast_bucket_interval::BROADCAST_BUCKET_INTERVAL_SPEC>,
    #[doc = "0x54 - In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application."]
    pub rx_append_crc: crate::Reg<self::mac_registers::rx_append_crc::RX_APPEND_CRC_SPEC>,
    #[doc = "0x58 - used to set receive Fifo high water mark"]
    pub rx_hwmark: crate::Reg<self::mac_registers::rx_hwmark::RX_HWMARK_SPEC>,
    #[doc = "0x5c - used to set receive Fifo low water mark"]
    pub rx_lwmark: crate::Reg<self::mac_registers::rx_lwmark::RX_LWMARK_SPEC>,
    #[doc = "0x60 - By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking."]
    pub crc_chk_en: crate::Reg<self::mac_registers::crc_chk_en::CRC_CHK_EN_SPEC>,
    #[doc = "0x64 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    pub rx_ifg_set: crate::Reg<self::mac_registers::rx_ifg_set::RX_IFG_SET_SPEC>,
    #[doc = "0x68 - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
    pub rx_max_length: crate::Reg<self::mac_registers::rx_max_length::RX_MAX_LENGTH_SPEC>,
    #[doc = "0x6c - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
    pub rx_min_length: crate::Reg<self::mac_registers::rx_min_length::RX_MIN_LENGTH_SPEC>,
    #[doc = "0x70 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    pub cpu_rd_addr: crate::Reg<self::mac_registers::cpu_rd_addr::CPU_RD_ADDR_SPEC>,
    #[doc = "0x74 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    pub cpu_rd_apply: crate::Reg<self::mac_registers::cpu_rd_apply::CPU_RD_APPLY_SPEC>,
    #[doc = "0x78 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    pub cpu_rd_grant: crate::Reg<self::mac_registers::cpu_rd_grant::CPU_RD_GRANT_SPEC>,
    #[doc = "0x7c - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    pub cpu_rd_dout_l: crate::Reg<self::mac_registers::cpu_rd_dout_l::CPU_RD_DOUT_L_SPEC>,
    #[doc = "0x80 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    pub cpu_rd_dout_h: crate::Reg<self::mac_registers::cpu_rd_dout_h::CPU_RD_DOUT_H_SPEC>,
    #[doc = "0x84 - If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose."]
    pub line_loop_en: crate::Reg<self::mac_registers::line_loop_en::LINE_LOOP_EN_SPEC>,
    #[doc = "0x88 - This register is used to set speed level of ethernet mac core."]
    pub speed: crate::Reg<self::mac_registers::speed::SPEED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "MAC_registers"]
pub mod mac_registers;
#[doc = r"Register block"]
#[repr(C)]
pub struct DMA_REGISTERS {
    #[doc = "0x00 - rx_rb_start_address, tx_rb_start_address These registers store the start address for the ring buffer used by the DMA to read from or write to. Writing to these registers has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) or the read pointer (for TX) to the start of the ring buffer. Therefore, the associated size register (rx_rb_size or tx_rb_size) should be set first. The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register."]
    pub rx_rb_start_address:
        crate::Reg<self::dma_registers::rx_rb_start_address::RX_RB_START_ADDRESS_SPEC>,
    #[doc = "0x04 - Stores the ring buffer size for the associated DMA channel in bytes."]
    pub rx_rb_size: crate::Reg<self::dma_registers::rx_rb_size::RX_RB_SIZE_SPEC>,
    #[doc = "0x08 - Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer."]
    pub rx_rb_rd_pointer: crate::Reg<self::dma_registers::rx_rb_rd_pointer::RX_RB_RD_POINTER_SPEC>,
    #[doc = "0x0c - Data access to the package length FIFO. Length in bytes. rx_pkg_lgth is read-only. The DMA write channel pushes packet lengths to the FIFO after the corresponding AXI transfers for the packet have finished. Reading this register when the associated FIFO is not empty removes the first-written piece of data from the FIFO."]
    pub rx_pkg_lgth: crate::Reg<self::dma_registers::rx_pkg_lgth::RX_PKG_LGTH_SPEC>,
    #[doc = "0x10 - Read-only register denoting how many packet lengths are available or yet to be processed in the corresponding FIFO."]
    pub rx_pkg_lgth_cnt: crate::Reg<self::dma_registers::rx_pkg_lgth_cnt::RX_PKG_LGTH_CNT_SPEC>,
    #[doc = "0x14 - Register to store the start address for the ring buffer used by the DMA to write to. Writing to the register has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) to the start of the ring buffer. Therefore, the associated size register (tx_rb_size should be set first The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register."]
    pub tx_rb_start_addr: crate::Reg<self::dma_registers::tx_rb_start_addr::TX_RB_START_ADDR_SPEC>,
    #[doc = "0x18 - Stores the ring buffer size for the associated DMA channel in bytes."]
    pub tx_rb_size: crate::Reg<self::dma_registers::tx_rb_size::TX_RB_SIZE_SPEC>,
    #[doc = "0x1c - Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer."]
    pub tx_rb_rd_pointer: crate::Reg<self::dma_registers::tx_rb_rd_pointer::TX_RB_RD_POINTER_SPEC>,
    #[doc = "0x20 - Data access to the package length FIFO. Packet lengths in bytes. tx_pkg_lgth is write-only. To issue a packet to be sent by the MAC, the host processor writes the ethernet frame data to the ring buffer, starting from a 4-byte aligned address immediately after the end of the previous packet, and writes the length of the packet to tx_pkg_lgth, in this order."]
    pub tx_pkg_lgth: crate::Reg<self::dma_registers::tx_pkg_lgth::TX_PKG_LGTH_SPEC>,
    #[doc = "0x24 - Read-only registers denoting how many packet lengths are available or yet to be processed in the corresponding FIFO."]
    pub tx_pkg_lgth_cnt: crate::Reg<self::dma_registers::tx_pkg_lgth_cnt::TX_PKG_LGTH_CNT_SPEC>,
    #[doc = "0x28 - One bit corresponds to an interrupt line from the MAC/DMA. When an interrupt is raised and its corresponding bit in irq_mask is set, its corresponding bit in irq is set and the external IRQ line is asserted. The IRQ line is deasserted when all bits in the irq register are unset by a write by the processor. In other words, the external IRQ line is an OR-reduction of the irq register. Writes to the irq register cannot assert new interrupt requests; in other words, the value stored in the irq register is the old contents combined with the write value by a bitwise AND."]
    pub irq: crate::Reg<self::dma_registers::irq::IRQ_SPEC>,
    #[doc = "0x2c - "]
    pub irq_mask: crate::Reg<self::dma_registers::irq_mask::IRQ_MASK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DMA_registers"]
pub mod dma_registers;
#[doc = r"Register block"]
#[repr(C)]
pub struct MDIO {
    #[doc = "0x00 - Acts as input to MDIO clock divider. MDIO clock is derived from Reg_clk (125/2 MHz) by dividing it by this value."]
    pub mdio_divider: crate::Reg<self::mdio::mdio_divider::MDIO_DIVIDER_SPEC>,
    #[doc = "0x04 - Data to write over MDIO. Used whenever a write operation is intiated with a write to the MDIO_Ctrl register."]
    pub mdio_wr_data: crate::Reg<self::mdio::mdio_wr_data::MDIO_WRDATA_SPEC>,
    #[doc = "0x08 - MDIO register address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register."]
    pub mdio_reg_addr: crate::Reg<self::mdio::mdio_reg_addr::MDIO_REGADDR_SPEC>,
    #[doc = "0x0c - MDIO PHY address. Used whenever a write or read operation is intiated with a write to the MDIO_Ctrl register"]
    pub mdio_phy_addr: crate::Reg<self::mdio::mdio_phy_addr::MDIO_PHYADDR_SPEC>,
    #[doc = "0x10 - Three-bit bitfield. Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used. Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts. Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts. Bits 1 and 2 should not be set simultaneously."]
    pub mdio_ctrl: crate::Reg<self::mdio::mdio_ctrl::MDIO_CTRL_SPEC>,
    #[doc = "0x14 - After MDIO read transaction has completed, this register is updated with the read result."]
    pub mdio_rd_data: crate::Reg<self::mdio::mdio_rd_data::MDIO_RDDATA_SPEC>,
    #[doc = "0x18 - Single-bit register, reads as 1 if MDIO transfer is ongoing."]
    pub mdio_status: crate::Reg<self::mdio::mdio_status::MDIO_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "MDIO"]
pub mod mdio;
