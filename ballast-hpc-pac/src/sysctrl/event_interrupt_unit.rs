#[doc = "MASK_read register accessor: an alias for `Reg<MASK_READ_SPEC>`"]
pub type MASK_READ = crate::Reg<mask_read::MASK_READ_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_read;
#[doc = "MASK_set register accessor: an alias for `Reg<MASK_SET_SPEC>`"]
pub type MASK_SET = crate::Reg<mask_set::MASK_SET_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_set;
#[doc = "MASK_clear register accessor: an alias for `Reg<MASK_CLEAR_SPEC>`"]
pub type MASK_CLEAR = crate::Reg<mask_clear::MASK_CLEAR_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_clear;
#[doc = "INT_read register accessor: an alias for `Reg<INT_READ_SPEC>`"]
pub type INT_READ = crate::Reg<int_read::INT_READ_SPEC>;
#[doc = "This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
pub mod int_read;
#[doc = "INT_set register accessor: an alias for `Reg<INT_SET_SPEC>`"]
pub type INT_SET = crate::Reg<int_set::INT_SET_SPEC>;
#[doc = "INT_read"]
pub mod int_set;
#[doc = "INT_clear register accessor: an alias for `Reg<INT_CLEAR_SPEC>`"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = ""]
pub mod int_clear;
#[doc = "ACK_read register accessor: an alias for `Reg<ACK_READ_SPEC>`"]
pub type ACK_READ = crate::Reg<ack_read::ACK_READ_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_read;
#[doc = "ACK_set register accessor: an alias for `Reg<ACK_SET_SPEC>`"]
pub type ACK_SET = crate::Reg<ack_set::ACK_SET_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_set;
#[doc = "ACK_clear register accessor: an alias for `Reg<ACK_CLEAR_SPEC>`"]
pub type ACK_CLEAR = crate::Reg<ack_clear::ACK_CLEAR_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_clear;
#[doc = "FIFO_DATA register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
pub mod fifo_data;
