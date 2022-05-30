#[doc = "Tx_Hwmark register accessor: an alias for `Reg<TX_HWMARK_SPEC>`"]
pub type TX_HWMARK = crate::Reg<tx_hwmark::TX_HWMARK_SPEC>;
#[doc = "used to set transmit Fifo high water mark"]
pub mod tx_hwmark;
#[doc = "Tx_Lwmark register accessor: an alias for `Reg<TX_LWMARK_SPEC>`"]
pub type TX_LWMARK = crate::Reg<tx_lwmark::TX_LWMARK_SPEC>;
#[doc = "used to set transmit Fifo low water mark"]
pub mod tx_lwmark;
#[doc = "MAC_rx_add_prom_add register accessor: an alias for `Reg<MAC_RX_ADD_PROM_ADD_SPEC>`"]
pub type MAC_RX_ADD_PROM_ADD = crate::Reg<mac_rx_add_prom_add::MAC_RX_ADD_PROM_ADD_SPEC>;
#[doc = ""]
pub mod mac_rx_add_prom_add;
#[doc = "Line_loop_en register accessor: an alias for `Reg<LINE_LOOP_EN_SPEC>`"]
pub type LINE_LOOP_EN = crate::Reg<line_loop_en::LINE_LOOP_EN_SPEC>;
#[doc = "If Line_loop_en =1 , the packet transmited to Phy will loopback to receive side. This function is used for test purpose."]
pub mod line_loop_en;
#[doc = "CPU_rd_dout_l register accessor: an alias for `Reg<CPU_RD_DOUT_L_SPEC>`"]
pub type CPU_RD_DOUT_L = crate::Reg<cpu_rd_dout_l::CPU_RD_DOUT_L_SPEC>;
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub mod cpu_rd_dout_l;
#[doc = "CPU_rd_apply register accessor: an alias for `Reg<CPU_RD_APPLY_SPEC>`"]
pub type CPU_RD_APPLY = crate::Reg<cpu_rd_apply::CPU_RD_APPLY_SPEC>;
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub mod cpu_rd_apply;
#[doc = "RX_MIN_LENGTH register accessor: an alias for `Reg<RX_MIN_LENGTH_SPEC>`"]
pub type RX_MIN_LENGTH = crate::Reg<rx_min_length::RX_MIN_LENGTH_SPEC>;
#[doc = "The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
pub mod rx_min_length;
#[doc = "RX_IFG_SET register accessor: an alias for `Reg<RX_IFG_SET_SPEC>`"]
pub type RX_IFG_SET = crate::Reg<rx_ifg_set::RX_IFG_SET_SPEC>;
#[doc = "RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub mod rx_ifg_set;
#[doc = "Rx_Lwmark register accessor: an alias for `Reg<RX_LWMARK_SPEC>`"]
pub type RX_LWMARK = crate::Reg<rx_lwmark::RX_LWMARK_SPEC>;
#[doc = "used to set receive Fifo low water mark"]
pub mod rx_lwmark;
#[doc = "RX_APPEND_CRC register accessor: an alias for `Reg<RX_APPEND_CRC_SPEC>`"]
pub type RX_APPEND_CRC = crate::Reg<rx_append_crc::RX_APPEND_CRC_SPEC>;
#[doc = "In some condition, the user application need MAC to retain FCS of ethernet frame. When RX_APPEND_CRC signal is equal “1” , the FCS of ethernet frame will transmit to user application."]
pub mod rx_append_crc;
#[doc = "broadcast_bucket_depth register accessor: an alias for `Reg<BROADCAST_BUCKET_DEPTH_SPEC>`"]
pub type BROADCAST_BUCKET_DEPTH = crate::Reg<broadcast_bucket_depth::BROADCAST_BUCKET_DEPTH_SPEC>;
#[doc = "broadcast_bucket_depth register is used to setting the bucket depth"]
pub mod broadcast_bucket_depth;
#[doc = "MAC_rx_add_prom_wr register accessor: an alias for `Reg<MAC_RX_ADD_PROM_WR_SPEC>`"]
pub type MAC_RX_ADD_PROM_WR = crate::Reg<mac_rx_add_prom_wr::MAC_RX_ADD_PROM_WR_SPEC>;
#[doc = ""]
pub mod mac_rx_add_prom_wr;
#[doc = "MAC_rx_add_chk_en register accessor: an alias for `Reg<MAC_RX_ADD_CHK_EN_SPEC>`"]
pub type MAC_RX_ADD_CHK_EN = crate::Reg<mac_rx_add_chk_en::MAC_RX_ADD_CHK_EN_SPEC>;
#[doc = ""]
pub mod mac_rx_add_chk_en;
#[doc = "xoff_cpu register accessor: an alias for `Reg<XOFF_CPU_SPEC>`"]
pub type XOFF_CPU = crate::Reg<xoff_cpu::XOFF_CPU_SPEC>;
#[doc = "The rising pulse of xoff_cpu signal is used to start transmit one PAUSE frame when the transmit in idle state with quanta zero, asking remote ethernet controller jump out from pause state."]
pub mod xoff_cpu;
#[doc = "MAC_tx_add_prom_wr register accessor: an alias for `Reg<MAC_TX_ADD_PROM_WR_SPEC>`"]
pub type MAC_TX_ADD_PROM_WR = crate::Reg<mac_tx_add_prom_wr::MAC_TX_ADD_PROM_WR_SPEC>;
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub mod mac_tx_add_prom_wr;
#[doc = "MAC_tx_add_prom_data register accessor: an alias for `Reg<MAC_TX_ADD_PROM_DATA_SPEC>`"]
pub type MAC_TX_ADD_PROM_DATA = crate::Reg<mac_tx_add_prom_data::MAC_TX_ADD_PROM_DATA_SPEC>;
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub mod mac_tx_add_prom_data;
#[doc = "MaxRetry register accessor: an alias for `Reg<MAXRETRY_SPEC>`"]
pub type MAXRETRY = crate::Reg<max_retry::MAXRETRY_SPEC>;
#[doc = "When collision occurred in half duplex mode, the transmit state machine will try to transmit this collision packet again. If one packet collide MaxRetry times, this packet will be drop and never try again."]
pub mod max_retry;
#[doc = "IFGset register accessor: an alias for `Reg<IFGSET_SPEC>`"]
pub type IFGSET = crate::Reg<ifgset::IFGSET_SPEC>;
#[doc = "RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub mod ifgset;
#[doc = "pause_frame_send_en register accessor: an alias for `Reg<PAUSE_FRAME_SEND_EN_SPEC>`"]
pub type PAUSE_FRAME_SEND_EN = crate::Reg<pause_frame_send_en::PAUSE_FRAME_SEND_EN_SPEC>;
#[doc = "pause_frame_send_en register is used to enable transmit logic to send PAUSE frame."]
pub mod pause_frame_send_en;
#[doc = "Speed register accessor: an alias for `Reg<SPEED_SPEC>`"]
pub type SPEED = crate::Reg<speed::SPEED_SPEC>;
#[doc = "This register is used to set speed level of ethernet mac core."]
pub mod speed;
#[doc = "CPU_rd_dout_h register accessor: an alias for `Reg<CPU_RD_DOUT_H_SPEC>`"]
pub type CPU_RD_DOUT_H = crate::Reg<cpu_rd_dout_h::CPU_RD_DOUT_H_SPEC>;
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub mod cpu_rd_dout_h;
#[doc = "CPU_rd_grant register accessor: an alias for `Reg<CPU_RD_GRANT_SPEC>`"]
pub type CPU_RD_GRANT = crate::Reg<cpu_rd_grant::CPU_RD_GRANT_SPEC>;
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub mod cpu_rd_grant;
#[doc = "CPU_rd_addr register accessor: an alias for `Reg<CPU_RD_ADDR_SPEC>`"]
pub type CPU_RD_ADDR = crate::Reg<cpu_rd_addr::CPU_RD_ADDR_SPEC>;
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub mod cpu_rd_addr;
#[doc = "RX_MAX_LENGTH register accessor: an alias for `Reg<RX_MAX_LENGTH_SPEC>`"]
pub type RX_MAX_LENGTH = crate::Reg<rx_max_length::RX_MAX_LENGTH_SPEC>;
#[doc = "The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
pub mod rx_max_length;
#[doc = "CRC_chk_en register accessor: an alias for `Reg<CRC_CHK_EN_SPEC>`"]
pub type CRC_CHK_EN = crate::Reg<crc_chk_en::CRC_CHK_EN_SPEC>;
#[doc = "By default, the receive logic will drop any packet with FCS checksum error. By setting CRC_chk_en register to zero, you can disable received packet FCS checking."]
pub mod crc_chk_en;
#[doc = "Rx_Hwmark register accessor: an alias for `Reg<RX_HWMARK_SPEC>`"]
pub type RX_HWMARK = crate::Reg<rx_hwmark::RX_HWMARK_SPEC>;
#[doc = "used to set receive Fifo high water mark"]
pub mod rx_hwmark;
#[doc = "broadcast_bucket_interval register accessor: an alias for `Reg<BROADCAST_BUCKET_INTERVAL_SPEC>`"]
pub type BROADCAST_BUCKET_INTERVAL =
    crate::Reg<broadcast_bucket_interval::BROADCAST_BUCKET_INTERVAL_SPEC>;
#[doc = "The bucket wil be periodically refilled after broadcast_bucket_interval time"]
pub mod broadcast_bucket_interval;
#[doc = "broadcast_filter_en register accessor: an alias for `Reg<BROADCAST_FILTER_EN_SPEC>`"]
pub type BROADCAST_FILTER_EN = crate::Reg<broadcast_filter_en::BROADCAST_FILTER_EN_SPEC>;
#[doc = "The broadcast packet filter will enable only when broadcast_filter_en is set"]
pub mod broadcast_filter_en;
#[doc = "MAC_rx_add_prom_data register accessor: an alias for `Reg<MAC_RX_ADD_PROM_DATA_SPEC>`"]
pub type MAC_RX_ADD_PROM_DATA = crate::Reg<mac_rx_add_prom_data::MAC_RX_ADD_PROM_DATA_SPEC>;
#[doc = ""]
pub mod mac_rx_add_prom_data;
#[doc = "xon_cpu register accessor: an alias for `Reg<XON_CPU_SPEC>`"]
pub type XON_CPU = crate::Reg<xon_cpu::XON_CPU_SPEC>;
#[doc = "The rising pulse of xon_cpu signal is used to start transmit one PAUSE frame with quanta value of pause_quanta_set when the transmit in idle state."]
pub mod xon_cpu;
#[doc = "tx_pause_en register accessor: an alias for `Reg<TX_PAUSE_EN_SPEC>`"]
pub type TX_PAUSE_EN = crate::Reg<tx_pause_en::TX_PAUSE_EN_SPEC>;
#[doc = "When this register is “1”, this core will respond to received pause frame.The transmit state machine will enter PAUSE state according to quanta value in received packet . One quanta time is equal to the time of transmit 512bit data."]
pub mod tx_pause_en;
#[doc = "MAC_tx_add_prom_add register accessor: an alias for `Reg<MAC_TX_ADD_PROM_ADD_SPEC>`"]
pub type MAC_TX_ADD_PROM_ADD = crate::Reg<mac_tx_add_prom_add::MAC_TX_ADD_PROM_ADD_SPEC>;
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub mod mac_tx_add_prom_add;
#[doc = "MAC_tx_add_en register accessor: an alias for `Reg<MAC_TX_ADD_EN_SPEC>`"]
pub type MAC_TX_ADD_EN = crate::Reg<mac_tx_add_en::MAC_TX_ADD_EN_SPEC>;
#[doc = "Those registers are used to set mac address which will replace the target mac address of transmit packet. This function will be enable one when register MAC_tx_add_en set to “1”. At the rising edge of signal MAC_tx_add_prom_wr, the value of MAC_tx_add_prom_data will be write to prom address MAC_tx_add_prom_add. You need repeat six times to write six bytes length target mac to prom."]
pub mod mac_tx_add_en;
#[doc = "FullDuplex register accessor: an alias for `Reg<FULLDUPLEX_SPEC>`"]
pub type FULLDUPLEX = crate::Reg<full_duplex::FULLDUPLEX_SPEC>;
#[doc = "This config register is only used in 100Mbps and 10Mbps. When FullDuplex register is equal to “1” , the transmit state machine will be FullDuplex mode. Otherwise, the transmit state machine will detect collision ,perform random slot time back off , retransmit collision packet and some other half-duplex operations."]
pub mod full_duplex;
#[doc = "pause_quanta_set register accessor: an alias for `Reg<PAUSE_QUANTA_SET_SPEC>`"]
pub type PAUSE_QUANTA_SET = crate::Reg<pause_quanta_set::PAUSE_QUANTA_SET_SPEC>;
#[doc = "pause_quanta_set register is used to setting quanta value in send PAUSE frame."]
pub mod pause_quanta_set;
