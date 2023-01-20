#[doc = r"Register block"]
#[repr(C)]
pub struct APB_SOFTWARE_IRQ {
    #[doc = "0x00 - "]
    pub software_irq_read: SOFTWARE_IRQ_READ,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - "]
    pub software_irq_set: SOFTWARE_IRQ_SET,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - "]
    pub software_irq_clear: SOFTWARE_IRQ_CLEAR,
}
#[doc = "software_irq_read (r) register accessor: an alias for `Reg<SOFTWARE_IRQ_READ_SPEC>`"]
pub type SOFTWARE_IRQ_READ = crate::Reg<software_irq_read::SOFTWARE_IRQ_READ_SPEC>;
#[doc = ""]
pub mod software_irq_read;
#[doc = "software_irq_set (w) register accessor: an alias for `Reg<SOFTWARE_IRQ_SET_SPEC>`"]
pub type SOFTWARE_IRQ_SET = crate::Reg<software_irq_set::SOFTWARE_IRQ_SET_SPEC>;
#[doc = ""]
pub mod software_irq_set;
#[doc = "software_irq_clear (w) register accessor: an alias for `Reg<SOFTWARE_IRQ_CLEAR_SPEC>`"]
pub type SOFTWARE_IRQ_CLEAR = crate::Reg<software_irq_clear::SOFTWARE_IRQ_CLEAR_SPEC>;
#[doc = ""]
pub mod software_irq_clear;
