#[doc = "command register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command (1 = reset, 2 = run, 4 = break)"]
pub mod command;
#[doc = "start_address register accessor: an alias for `Reg<START_ADDRESS_SPEC>`"]
pub type START_ADDRESS = crate::Reg<start_address::START_ADDRESS_SPEC>;
#[doc = ""]
pub mod start_address;
#[doc = "breakpoint_enable register accessor: an alias for `Reg<BREAKPOINT_ENABLE_SPEC>`"]
pub type BREAKPOINT_ENABLE = crate::Reg<breakpoint_enable::BREAKPOINT_ENABLE_SPEC>;
#[doc = "Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2"]
pub mod breakpoint_enable;
#[doc = "breakpoint_1_addr register accessor: an alias for `Reg<BREAKPOINT_1_ADDR_SPEC>`"]
pub type BREAKPOINT_1_ADDR = crate::Reg<breakpoint_1_addr::BREAKPOINT_1_ADDR_SPEC>;
#[doc = ""]
pub mod breakpoint_1_addr;
#[doc = "breakpoint_2_addr register accessor: an alias for `Reg<BREAKPOINT_2_ADDR_SPEC>`"]
pub type BREAKPOINT_2_ADDR = crate::Reg<breakpoint_2_addr::BREAKPOINT_2_ADDR_SPEC>;
#[doc = ""]
pub mod breakpoint_2_addr;
#[doc = "cycle_count_breakpoint register accessor: an alias for `Reg<CYCLE_COUNT_BREAKPOINT_SPEC>`"]
pub type CYCLE_COUNT_BREAKPOINT = crate::Reg<cycle_count_breakpoint::CYCLE_COUNT_BREAKPOINT_SPEC>;
#[doc = ""]
pub mod cycle_count_breakpoint;
