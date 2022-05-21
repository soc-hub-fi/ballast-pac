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
#[doc = r"Register block"]
#[repr(C)]
pub struct STATUS_REGISTERS {
    #[doc = "0x00 - Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint"]
    pub status: crate::Reg<self::status_registers::status::STATUS_SPEC>,
    #[doc = "0x04 - Program counter"]
    pub program_counter: crate::Reg<self::status_registers::program_counter::PROGRAM_COUNTER_SPEC>,
    #[doc = "0x08 - Low part of Cycle count register"]
    pub cycle_count_low: crate::Reg<self::status_registers::cycle_count_low::CYCLE_COUNT_LOW_SPEC>,
    #[doc = "0x0c - High part of Cycle count register"]
    pub cycle_count_high:
        crate::Reg<self::status_registers::cycle_count_high::CYCLE_COUNT_HIGH_SPEC>,
    #[doc = "0x10 - Low part of Stall count"]
    pub stall_count_low: crate::Reg<self::status_registers::stall_count_low::STALL_COUNT_LOW_SPEC>,
    #[doc = "0x14 - High part of Stall count"]
    pub stall_count_high:
        crate::Reg<self::status_registers::stall_count_high::STALL_COUNT_HIGH_SPEC>,
}
#[doc = r"Register block"]
#[doc = "status_registers"]
pub mod status_registers;
#[doc = r"Register block"]
#[repr(C)]
pub struct CONTROL_REGISTERS {
    #[doc = "0x00 - Command (1 = reset, 2 = run, 4 = break)"]
    pub command: crate::Reg<self::control_registers::command::COMMAND_SPEC>,
    #[doc = "0x04 - "]
    pub start_address: crate::Reg<self::control_registers::start_address::START_ADDRESS_SPEC>,
    #[doc = "0x08 - Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2"]
    pub breakpoint_enable:
        crate::Reg<self::control_registers::breakpoint_enable::BREAKPOINT_ENABLE_SPEC>,
    #[doc = "0x0c - "]
    pub cycle_count_breakpoint:
        crate::Reg<self::control_registers::cycle_count_breakpoint::CYCLE_COUNT_BREAKPOINT_SPEC>,
    #[doc = "0x10 - "]
    pub breakpoint_2_addr:
        crate::Reg<self::control_registers::breakpoint_2_addr::BREAKPOINT_2_ADDR_SPEC>,
    #[doc = "0x14 - "]
    pub breakpoint_1_addr:
        crate::Reg<self::control_registers::breakpoint_1_addr::BREAKPOINT_1_ADDR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "control_registers"]
pub mod control_registers;
#[doc = r"Register block"]
#[repr(C)]
pub struct PROCESSOR_INFO_REGISTERS {
    #[doc = "0x00 - Device class (0x774)"]
    pub device_class: crate::Reg<self::processor_info_registers::device_class::DEVICE_CLASS_SPEC>,
    #[doc = "0x04 - Device ID (unimplemented)"]
    pub device_id: crate::Reg<self::processor_info_registers::device_id::DEVICE_ID_SPEC>,
    #[doc = "0x08 - Interface version (0x2)"]
    pub interface_version:
        crate::Reg<self::processor_info_registers::interface_version::INTERFACE_VERSION_SPEC>,
    #[doc = "0x0c - Core count"]
    pub core_count: crate::Reg<self::processor_info_registers::core_count::CORE_COUNT_SPEC>,
    #[doc = "0x10 - CTRL size, per core, in bytes"]
    pub ctrl_size: crate::Reg<self::processor_info_registers::ctrl_size::CTRL_SIZE_SPEC>,
    #[doc = "0x14 - Data memory size, in bytes"]
    pub data_mem_size:
        crate::Reg<self::processor_info_registers::data_mem_size::DATA_MEM_SIZE_SPEC>,
    #[doc = "0x18 - Instruction memory size, in bytes"]
    pub instr_mem_size:
        crate::Reg<self::processor_info_registers::instr_mem_size::INSTR_MEM_SIZE_SPEC>,
    #[doc = "0x1c - Parameter memory size, in bytes"]
    pub param_mem_size:
        crate::Reg<self::processor_info_registers::param_mem_size::PARAM_MEM_SIZE_SPEC>,
    #[doc = "0x20 - Debug feature support (0x1)"]
    pub debug_feature_support: crate::Reg<
        self::processor_info_registers::debug_feature_support::DEBUG_FEATURE_SUPPORT_SPEC,
    >,
    #[doc = "0x24 - Breakpoint count (0x2)"]
    pub breakpoint_count:
        crate::Reg<self::processor_info_registers::breakpoint_count::BREAKPOINT_COUNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "processor_info_registers"]
pub mod processor_info_registers;
#[doc = r"Register block"]
#[repr(C)]
pub struct BUS_TRACE_REGISTERS {
    #[doc = "0x00 - "]
    pub bus_trace_reg_0:
        crate::Reg<self::bus_trace_registers::bus_trace_reg_0::BUS_TRACE_REG_0_SPEC>,
    #[doc = "0x04 - "]
    pub bus_trace_reg_1:
        crate::Reg<self::bus_trace_registers::bus_trace_reg_1::BUS_TRACE_REG_1_SPEC>,
    #[doc = "0x08 - "]
    pub bus_trace_reg_2:
        crate::Reg<self::bus_trace_registers::bus_trace_reg_2::BUS_TRACE_REG_2_SPEC>,
    #[doc = "0x0c - "]
    pub bus_trace_reg_3:
        crate::Reg<self::bus_trace_registers::bus_trace_reg_3::BUS_TRACE_REG_3_SPEC>,
    #[doc = "0x10 - "]
    pub bus_trace_reg_4:
        crate::Reg<self::bus_trace_registers::bus_trace_reg_4::BUS_TRACE_REG_4_SPEC>,
}
#[doc = r"Register block"]
#[doc = "bus_trace_registers"]
pub mod bus_trace_registers;
