#[doc = "status register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status: Shows why the TTA core is in a break state Bit 0: single-stepping breakpoint Bit 1: Breakpoint 1 Bit 2: Breakpoint 2 Bit 3: Manual breakpoint"]
pub mod status;
#[doc = "program_counter register accessor: an alias for `Reg<PROGRAM_COUNTER_SPEC>`"]
pub type PROGRAM_COUNTER = crate::Reg<program_counter::PROGRAM_COUNTER_SPEC>;
#[doc = "Program counter"]
pub mod program_counter;
#[doc = "cycle_count_low register accessor: an alias for `Reg<CYCLE_COUNT_LOW_SPEC>`"]
pub type CYCLE_COUNT_LOW = crate::Reg<cycle_count_low::CYCLE_COUNT_LOW_SPEC>;
#[doc = "Low part of Cycle count register"]
pub mod cycle_count_low;
#[doc = "cycle_count_high register accessor: an alias for `Reg<CYCLE_COUNT_HIGH_SPEC>`"]
pub type CYCLE_COUNT_HIGH = crate::Reg<cycle_count_high::CYCLE_COUNT_HIGH_SPEC>;
#[doc = "High part of Cycle count register"]
pub mod cycle_count_high;
#[doc = "stall_count_low register accessor: an alias for `Reg<STALL_COUNT_LOW_SPEC>`"]
pub type STALL_COUNT_LOW = crate::Reg<stall_count_low::STALL_COUNT_LOW_SPEC>;
#[doc = "Low part of Stall count"]
pub mod stall_count_low;
#[doc = "stall_count_high register accessor: an alias for `Reg<STALL_COUNT_HIGH_SPEC>`"]
pub type STALL_COUNT_HIGH = crate::Reg<stall_count_high::STALL_COUNT_HIGH_SPEC>;
#[doc = "High part of Stall count"]
pub mod stall_count_high;
