#[doc = r"Register block"]
#[repr(C)]
pub struct EVENT_INTERRUPT_UNIT {
    #[doc = "0x00 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    pub mask_read: MASK_READ,
    #[doc = "0x04 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    pub mask_set: MASK_SET,
    #[doc = "0x08 - This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
    pub mask_clear: MASK_CLEAR,
    #[doc = "0x0c - This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
    pub int_read: INT_READ,
    #[doc = "0x10 - INT_read"]
    pub int_set: INT_SET,
    #[doc = "0x14 - "]
    pub int_clear: INT_CLEAR,
    #[doc = "0x18 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    pub ack_read: ACK_READ,
    #[doc = "0x1c - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    pub ack_set: ACK_SET,
    #[doc = "0x20 - This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
    pub ack_clear: ACK_CLEAR,
    #[doc = "0x24 - Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
    pub fifo_data: FIFO_DATA,
}
#[doc = "MASK_read (rw) register accessor: an alias for `Reg<MASK_READ_SPEC>`"]
pub type MASK_READ = crate::Reg<mask_read::MASK_READ_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_read;
#[doc = "MASK_set (rw) register accessor: an alias for `Reg<MASK_SET_SPEC>`"]
pub type MASK_SET = crate::Reg<mask_set::MASK_SET_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_set;
#[doc = "MASK_clear (rw) register accessor: an alias for `Reg<MASK_CLEAR_SPEC>`"]
pub type MASK_CLEAR = crate::Reg<mask_clear::MASK_CLEAR_SPEC>;
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected."]
pub mod mask_clear;
#[doc = "INT_read (rw) register accessor: an alias for `Reg<INT_READ_SPEC>`"]
pub type INT_READ = crate::Reg<int_read::INT_READ_SPEC>;
#[doc = "This register contains the pending interrupts or events. Writing to 0x1A10_9010 sets the bits of the INT register selected. Writing to 0x1A10_9014 clears the bits of the INT register selected."]
pub mod int_read;
#[doc = "INT_set (rw) register accessor: an alias for `Reg<INT_SET_SPEC>`"]
pub type INT_SET = crate::Reg<int_set::INT_SET_SPEC>;
#[doc = "INT_read"]
pub mod int_set;
#[doc = "INT_clear (rw) register accessor: an alias for `Reg<INT_CLEAR_SPEC>`"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = ""]
pub mod int_clear;
#[doc = "ACK_read (rw) register accessor: an alias for `Reg<ACK_READ_SPEC>`"]
pub type ACK_READ = crate::Reg<ack_read::ACK_READ_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_read;
#[doc = "ACK_set (rw) register accessor: an alias for `Reg<ACK_SET_SPEC>`"]
pub type ACK_SET = crate::Reg<ack_set::ACK_SET_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_set;
#[doc = "ACK_clear (rw) register accessor: an alias for `Reg<ACK_CLEAR_SPEC>`"]
pub type ACK_CLEAR = crate::Reg<ack_clear::ACK_CLEAR_SPEC>;
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected."]
pub mod ack_clear;
#[doc = "FIFO_DATA (r) register accessor: an alias for `Reg<FIFO_DATA_SPEC>`"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Fifo Content. This is a read-only register that contain the first valid value of the FIFO."]
pub mod fifo_data;
