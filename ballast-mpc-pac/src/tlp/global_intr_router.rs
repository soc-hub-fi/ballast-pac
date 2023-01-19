#[doc = r"Register block"]
#[repr(C)]
pub struct GLOBAL_INTR_ROUTER {
    #[doc = "0x00 - SysCtrl route register 0"]
    pub irq_0_route_reg_0: IRQ_0_ROUTE_REG_0,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - MPC route register 0"]
    pub irq_1_route_reg_0: IRQ_1_ROUTE_REG_0,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - HPC route register 0"]
    pub irq_2_route_reg_0: IRQ_2_ROUTE_REG_0,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - SysCtrl route register 1"]
    pub irq_0_route_reg_1: IRQ_0_ROUTE_REG_1,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - MPC route register 1"]
    pub irq_1_route_reg_1: IRQ_1_ROUTE_REG_1,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - HPC route register 1"]
    pub irq_2_route_reg_1: IRQ_2_ROUTE_REG_1,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - SysCtrl route register 2"]
    pub irq_0_route_reg_2: IRQ_0_ROUTE_REG_2,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - MPC route register 2"]
    pub irq_1_route_reg_2: IRQ_1_ROUTE_REG_2,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - HPC route register 2"]
    pub irq_2_route_reg_2: IRQ_2_ROUTE_REG_2,
    _reserved9: [u8; 0x07],
    #[doc = "0x28 - MPC route register 3"]
    pub irq_1_route_reg_3: IRQ_1_ROUTE_REG_3,
    _reserved10: [u8; 0x03],
    #[doc = "0x2c - HPC route register 3"]
    pub irq_2_route_reg_3: IRQ_2_ROUTE_REG_3,
    _reserved11: [u8; 0x07],
    #[doc = "0x34 - MPC route register 4"]
    pub irq_1_route_reg_4: IRQ_1_ROUTE_REG_4,
    _reserved12: [u8; 0x0b],
    #[doc = "0x40 - MPC route register 5"]
    pub irq_1_route_reg_5: IRQ_1_ROUTE_REG_5,
    _reserved13: [u8; 0x0b],
    #[doc = "0x4c - MPC route register 6"]
    pub irq_1_route_reg_6: IRQ_1_ROUTE_REG_6,
}
#[doc = "irq_0_route_reg_0 (rw) register accessor: an alias for `Reg<IRQ_0_ROUTE_REG_0_SPEC>`"]
pub type IRQ_0_ROUTE_REG_0 = crate::Reg<irq_0_route_reg_0::IRQ_0_ROUTE_REG_0_SPEC>;
#[doc = "SysCtrl route register 0"]
pub mod irq_0_route_reg_0;
#[doc = "irq_0_route_reg_1 (rw) register accessor: an alias for `Reg<IRQ_0_ROUTE_REG_1_SPEC>`"]
pub type IRQ_0_ROUTE_REG_1 = crate::Reg<irq_0_route_reg_1::IRQ_0_ROUTE_REG_1_SPEC>;
#[doc = "SysCtrl route register 1"]
pub mod irq_0_route_reg_1;
#[doc = "irq_0_route_reg_2 (rw) register accessor: an alias for `Reg<IRQ_0_ROUTE_REG_2_SPEC>`"]
pub type IRQ_0_ROUTE_REG_2 = crate::Reg<irq_0_route_reg_2::IRQ_0_ROUTE_REG_2_SPEC>;
#[doc = "SysCtrl route register 2"]
pub mod irq_0_route_reg_2;
#[doc = "irq_1_route_reg_0 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_0_SPEC>`"]
pub type IRQ_1_ROUTE_REG_0 = crate::Reg<irq_1_route_reg_0::IRQ_1_ROUTE_REG_0_SPEC>;
#[doc = "MPC route register 0"]
pub mod irq_1_route_reg_0;
#[doc = "irq_1_route_reg_6 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_6_SPEC>`"]
pub type IRQ_1_ROUTE_REG_6 = crate::Reg<irq_1_route_reg_6::IRQ_1_ROUTE_REG_6_SPEC>;
#[doc = "MPC route register 6"]
pub mod irq_1_route_reg_6;
#[doc = "irq_1_route_reg_4 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_4_SPEC>`"]
pub type IRQ_1_ROUTE_REG_4 = crate::Reg<irq_1_route_reg_4::IRQ_1_ROUTE_REG_4_SPEC>;
#[doc = "MPC route register 4"]
pub mod irq_1_route_reg_4;
#[doc = "irq_2_route_reg_1 (rw) register accessor: an alias for `Reg<IRQ_2_ROUTE_REG_1_SPEC>`"]
pub type IRQ_2_ROUTE_REG_1 = crate::Reg<irq_2_route_reg_1::IRQ_2_ROUTE_REG_1_SPEC>;
#[doc = "HPC route register 1"]
pub mod irq_2_route_reg_1;
#[doc = "irq_2_route_reg_0 (rw) register accessor: an alias for `Reg<IRQ_2_ROUTE_REG_0_SPEC>`"]
pub type IRQ_2_ROUTE_REG_0 = crate::Reg<irq_2_route_reg_0::IRQ_2_ROUTE_REG_0_SPEC>;
#[doc = "HPC route register 0"]
pub mod irq_2_route_reg_0;
#[doc = "irq_2_route_reg_2 (rw) register accessor: an alias for `Reg<IRQ_2_ROUTE_REG_2_SPEC>`"]
pub type IRQ_2_ROUTE_REG_2 = crate::Reg<irq_2_route_reg_2::IRQ_2_ROUTE_REG_2_SPEC>;
#[doc = "HPC route register 2"]
pub mod irq_2_route_reg_2;
#[doc = "irq_1_route_reg_1 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_1_SPEC>`"]
pub type IRQ_1_ROUTE_REG_1 = crate::Reg<irq_1_route_reg_1::IRQ_1_ROUTE_REG_1_SPEC>;
#[doc = "MPC route register 1"]
pub mod irq_1_route_reg_1;
#[doc = "irq_1_route_reg_5 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_5_SPEC>`"]
pub type IRQ_1_ROUTE_REG_5 = crate::Reg<irq_1_route_reg_5::IRQ_1_ROUTE_REG_5_SPEC>;
#[doc = "MPC route register 5"]
pub mod irq_1_route_reg_5;
#[doc = "irq_1_route_reg_2 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_2_SPEC>`"]
pub type IRQ_1_ROUTE_REG_2 = crate::Reg<irq_1_route_reg_2::IRQ_1_ROUTE_REG_2_SPEC>;
#[doc = "MPC route register 2"]
pub mod irq_1_route_reg_2;
#[doc = "irq_1_route_reg_3 (rw) register accessor: an alias for `Reg<IRQ_1_ROUTE_REG_3_SPEC>`"]
pub type IRQ_1_ROUTE_REG_3 = crate::Reg<irq_1_route_reg_3::IRQ_1_ROUTE_REG_3_SPEC>;
#[doc = "MPC route register 3"]
pub mod irq_1_route_reg_3;
#[doc = "irq_2_route_reg_3 (rw) register accessor: an alias for `Reg<IRQ_2_ROUTE_REG_3_SPEC>`"]
pub type IRQ_2_ROUTE_REG_3 = crate::Reg<irq_2_route_reg_3::IRQ_2_ROUTE_REG_3_SPEC>;
#[doc = "HPC route register 3"]
pub mod irq_2_route_reg_3;
