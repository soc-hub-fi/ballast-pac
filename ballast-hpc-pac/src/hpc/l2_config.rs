#[doc = "l2_enable register accessor: an alias for `Reg<L2_ENABLE_SPEC>`"]
pub type L2_ENABLE = crate::Reg<l2_enable::L2_ENABLE_SPEC>;
#[doc = "Enable L2 cache"]
pub mod l2_enable;
#[doc = "core_enable register accessor: an alias for `Reg<CORE_ENABLE_SPEC>`"]
pub type CORE_ENABLE = crate::Reg<core_enable::CORE_ENABLE_SPEC>;
#[doc = "Enable shadow L1 caches in L2;NI"]
pub mod core_enable;
#[doc = "l2_arbitration_priority register accessor: an alias for `Reg<L2_ARBITRATION_PRIORITY_SPEC>`"]
pub type L2_ARBITRATION_PRIORITY =
    crate::Reg<l2_arbitration_priority::L2_ARBITRATION_PRIORITY_SPEC>;
#[doc = "Select L2 replacement algorithm; NI"]
pub mod l2_arbitration_priority;
#[doc = "l2_replacement_policy register accessor: an alias for `Reg<L2_REPLACEMENT_POLICY_SPEC>`"]
pub type L2_REPLACEMENT_POLICY = crate::Reg<l2_replacement_policy::L2_REPLACEMENT_POLICY_SPEC>;
#[doc = "Select L2 core priority algorithm; NI"]
pub mod l2_replacement_policy;
#[doc = "l2_arbitration_policy register accessor: an alias for `Reg<L2_ARBITRATION_POLICY_SPEC>`"]
pub type L2_ARBITRATION_POLICY = crate::Reg<l2_arbitration_policy::L2_ARBITRATION_POLICY_SPEC>;
#[doc = "Select L2 core priorities; NI"]
pub mod l2_arbitration_policy;
