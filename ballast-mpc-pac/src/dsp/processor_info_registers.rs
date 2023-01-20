#[doc = r"Register block"]
#[repr(C)]
pub struct PROCESSOR_INFO_REGISTERS {
    #[doc = "0x00 - Device class (0x774)"]
    pub device_class: DEVICE_CLASS,
    #[doc = "0x04 - Device ID (unimplemented)"]
    pub device_id: DEVICE_ID,
    #[doc = "0x08 - Interface version (0x2)"]
    pub interface_version: INTERFACE_VERSION,
    #[doc = "0x0c - Core count"]
    pub core_count: CORE_COUNT,
    #[doc = "0x10 - CTRL size, per core, in bytes"]
    pub ctrl_size: CTRL_SIZE,
    #[doc = "0x14 - Data memory size, in bytes"]
    pub data_mem_size: DATA_MEM_SIZE,
    #[doc = "0x18 - Instruction memory size, in bytes"]
    pub instr_mem_size: INSTR_MEM_SIZE,
    #[doc = "0x1c - Parameter memory size, in bytes"]
    pub param_mem_size: PARAM_MEM_SIZE,
    #[doc = "0x20 - Debug feature support (0x1)"]
    pub debug_feature_support: DEBUG_FEATURE_SUPPORT,
    #[doc = "0x24 - Breakpoint count (0x2)"]
    pub breakpoint_count: BREAKPOINT_COUNT,
}
#[doc = "device_class (r) register accessor: an alias for `Reg<DEVICE_CLASS_SPEC>`"]
pub type DEVICE_CLASS = crate::Reg<device_class::DEVICE_CLASS_SPEC>;
#[doc = "Device class (0x774)"]
pub mod device_class;
#[doc = "device_id (r) register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Device ID (unimplemented)"]
pub mod device_id;
#[doc = "interface_version (r) register accessor: an alias for `Reg<INTERFACE_VERSION_SPEC>`"]
pub type INTERFACE_VERSION = crate::Reg<interface_version::INTERFACE_VERSION_SPEC>;
#[doc = "Interface version (0x2)"]
pub mod interface_version;
#[doc = "core_count (r) register accessor: an alias for `Reg<CORE_COUNT_SPEC>`"]
pub type CORE_COUNT = crate::Reg<core_count::CORE_COUNT_SPEC>;
#[doc = "Core count"]
pub mod core_count;
#[doc = "ctrl_size (r) register accessor: an alias for `Reg<CTRL_SIZE_SPEC>`"]
pub type CTRL_SIZE = crate::Reg<ctrl_size::CTRL_SIZE_SPEC>;
#[doc = "CTRL size, per core, in bytes"]
pub mod ctrl_size;
#[doc = "data_mem_size (r) register accessor: an alias for `Reg<DATA_MEM_SIZE_SPEC>`"]
pub type DATA_MEM_SIZE = crate::Reg<data_mem_size::DATA_MEM_SIZE_SPEC>;
#[doc = "Data memory size, in bytes"]
pub mod data_mem_size;
#[doc = "instr_mem_size (r) register accessor: an alias for `Reg<INSTR_MEM_SIZE_SPEC>`"]
pub type INSTR_MEM_SIZE = crate::Reg<instr_mem_size::INSTR_MEM_SIZE_SPEC>;
#[doc = "Instruction memory size, in bytes"]
pub mod instr_mem_size;
#[doc = "breakpoint_count (r) register accessor: an alias for `Reg<BREAKPOINT_COUNT_SPEC>`"]
pub type BREAKPOINT_COUNT = crate::Reg<breakpoint_count::BREAKPOINT_COUNT_SPEC>;
#[doc = "Breakpoint count (0x2)"]
pub mod breakpoint_count;
#[doc = "param_mem_size (r) register accessor: an alias for `Reg<PARAM_MEM_SIZE_SPEC>`"]
pub type PARAM_MEM_SIZE = crate::Reg<param_mem_size::PARAM_MEM_SIZE_SPEC>;
#[doc = "Parameter memory size, in bytes"]
pub mod param_mem_size;
#[doc = "debug_feature_support (r) register accessor: an alias for `Reg<DEBUG_FEATURE_SUPPORT_SPEC>`"]
pub type DEBUG_FEATURE_SUPPORT = crate::Reg<debug_feature_support::DEBUG_FEATURE_SUPPORT_SPEC>;
#[doc = "Debug feature support (0x1)"]
pub mod debug_feature_support;
