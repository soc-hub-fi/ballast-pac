#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x18 - status_registers"]
    pub status_registers: STATUS_REGISTERS,
    _reserved1: [u8; 0x28],
    #[doc = "0x40..0x54 - bus_trace_registers"]
    pub bus_trace_registers: BUS_TRACE_REGISTERS,
    _reserved2: [u8; 0x01ac],
    #[doc = "0x200..0x218 - control_registers"]
    pub control_registers: CONTROL_REGISTERS,
    _reserved3: [u8; 0xe8],
    #[doc = "0x300..0x328 - processor_info_registers"]
    pub processor_info_registers: PROCESSOR_INFO_REGISTERS,
}
#[doc = "status_registers"]
pub use self::status_registers::STATUS_REGISTERS;
#[doc = r"Cluster"]
#[doc = "status_registers"]
pub mod status_registers;
#[doc = "control_registers"]
pub use self::control_registers::CONTROL_REGISTERS;
#[doc = r"Cluster"]
#[doc = "control_registers"]
pub mod control_registers;
#[doc = "processor_info_registers"]
pub use self::processor_info_registers::PROCESSOR_INFO_REGISTERS;
#[doc = r"Cluster"]
#[doc = "processor_info_registers"]
pub mod processor_info_registers;
#[doc = "bus_trace_registers"]
pub use self::bus_trace_registers::BUS_TRACE_REGISTERS;
#[doc = r"Cluster"]
#[doc = "bus_trace_registers"]
pub mod bus_trace_registers;
