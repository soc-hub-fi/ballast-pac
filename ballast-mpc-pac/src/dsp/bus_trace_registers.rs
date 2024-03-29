#[doc = r"Register block"]
#[repr(C)]
pub struct BUS_TRACE_REGISTERS {
    #[doc = "0x00 - "]
    pub bus_trace_reg_0: BUS_TRACE_REG_0,
    #[doc = "0x04 - "]
    pub bus_trace_reg_1: BUS_TRACE_REG_1,
    #[doc = "0x08 - "]
    pub bus_trace_reg_2: BUS_TRACE_REG_2,
    #[doc = "0x0c - "]
    pub bus_trace_reg_3: BUS_TRACE_REG_3,
    #[doc = "0x10 - "]
    pub bus_trace_reg_4: BUS_TRACE_REG_4,
}
#[doc = "bus_trace_reg_0 (r) register accessor: an alias for `Reg<BUS_TRACE_REG_0_SPEC>`"]
pub type BUS_TRACE_REG_0 = crate::Reg<bus_trace_reg_0::BUS_TRACE_REG_0_SPEC>;
#[doc = ""]
pub mod bus_trace_reg_0;
#[doc = "bus_trace_reg_1 (r) register accessor: an alias for `Reg<BUS_TRACE_REG_1_SPEC>`"]
pub type BUS_TRACE_REG_1 = crate::Reg<bus_trace_reg_1::BUS_TRACE_REG_1_SPEC>;
#[doc = ""]
pub mod bus_trace_reg_1;
#[doc = "bus_trace_reg_2 (r) register accessor: an alias for `Reg<BUS_TRACE_REG_2_SPEC>`"]
pub type BUS_TRACE_REG_2 = crate::Reg<bus_trace_reg_2::BUS_TRACE_REG_2_SPEC>;
#[doc = ""]
pub mod bus_trace_reg_2;
#[doc = "bus_trace_reg_3 (r) register accessor: an alias for `Reg<BUS_TRACE_REG_3_SPEC>`"]
pub type BUS_TRACE_REG_3 = crate::Reg<bus_trace_reg_3::BUS_TRACE_REG_3_SPEC>;
#[doc = ""]
pub mod bus_trace_reg_3;
#[doc = "bus_trace_reg_4 (r) register accessor: an alias for `Reg<BUS_TRACE_REG_4_SPEC>`"]
pub type BUS_TRACE_REG_4 = crate::Reg<bus_trace_reg_4::BUS_TRACE_REG_4_SPEC>;
#[doc = ""]
pub mod bus_trace_reg_4;
