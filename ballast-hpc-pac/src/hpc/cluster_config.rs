#[doc = "core0_bootaddr register accessor: an alias for `Reg<CORE0_BOOTADDR_SPEC>`"]
pub type CORE0_BOOTADDR = crate::Reg<core0_bootaddr::CORE0_BOOTADDR_SPEC>;
#[doc = "Core#0 boot address"]
pub mod core0_bootaddr;
#[doc = "core1_bootaddr register accessor: an alias for `Reg<CORE1_BOOTADDR_SPEC>`"]
pub type CORE1_BOOTADDR = crate::Reg<core1_bootaddr::CORE1_BOOTADDR_SPEC>;
#[doc = "Core#1 boot address"]
pub mod core1_bootaddr;
#[doc = "core0_hart_id register accessor: an alias for `Reg<CORE0_HART_ID_SPEC>`"]
pub type CORE0_HART_ID = crate::Reg<core0_hart_id::CORE0_HART_ID_SPEC>;
#[doc = "Core#0 hart ID"]
pub mod core0_hart_id;
#[doc = "core1_hart_id register accessor: an alias for `Reg<CORE1_HART_ID_SPEC>`"]
pub type CORE1_HART_ID = crate::Reg<core1_hart_id::CORE1_HART_ID_SPEC>;
#[doc = "Core#1 hart ID"]
pub mod core1_hart_id;
#[doc = "rtc_cfg0 register accessor: an alias for `Reg<RTC_CFG0_SPEC>`"]
pub type RTC_CFG0 = crate::Reg<rtc_cfg0::RTC_CFG0_SPEC>;
#[doc = "real time clock generator clock high count"]
pub mod rtc_cfg0;
#[doc = "rtc_cfg1 register accessor: an alias for `Reg<RTC_CFG1_SPEC>`"]
pub type RTC_CFG1 = crate::Reg<rtc_cfg1::RTC_CFG1_SPEC>;
#[doc = "real time clock generator clock low count"]
pub mod rtc_cfg1;
#[doc = "rtc_cfg2 register accessor: an alias for `Reg<RTC_CFG2_SPEC>`"]
pub type RTC_CFG2 = crate::Reg<rtc_cfg2::RTC_CFG2_SPEC>;
#[doc = "real time clock generator clock fine tune"]
pub mod rtc_cfg2;
#[doc = "irq_status register accessor: an alias for `Reg<IRQ_STATUS_SPEC>`"]
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUS_SPEC>;
#[doc = "interrupt flag statuses; NI"]
pub mod irq_status;
#[doc = "force_irq register accessor: an alias for `Reg<FORCE_IRQ_SPEC>`"]
pub type FORCE_IRQ = crate::Reg<force_irq::FORCE_IRQ_SPEC>;
#[doc = "force interrupt line active; NI"]
pub mod force_irq;
#[doc = "timer_base_addr register accessor: an alias for `Reg<TIMER_BASE_ADDR_SPEC>`"]
pub type TIMER_BASE_ADDR = crate::Reg<timer_base_addr::TIMER_BASE_ADDR_SPEC>;
#[doc = "Timer base address"]
pub mod timer_base_addr;
#[doc = "timer_addr_length register accessor: an alias for `Reg<TIMER_ADDR_LENGTH_SPEC>`"]
pub type TIMER_ADDR_LENGTH = crate::Reg<timer_addr_length::TIMER_ADDR_LENGTH_SPEC>;
#[doc = "Timer address space length"]
pub mod timer_addr_length;
#[doc = "plic_base_addr register accessor: an alias for `Reg<PLIC_BASE_ADDR_SPEC>`"]
pub type PLIC_BASE_ADDR = crate::Reg<plic_base_addr::PLIC_BASE_ADDR_SPEC>;
#[doc = "PLIC base address"]
pub mod plic_base_addr;
#[doc = "clint_addr_length register accessor: an alias for `Reg<CLINT_ADDR_LENGTH_SPEC>`"]
pub type CLINT_ADDR_LENGTH = crate::Reg<clint_addr_length::CLINT_ADDR_LENGTH_SPEC>;
#[doc = "CLINT address space length"]
pub mod clint_addr_length;
#[doc = "cached_region_addr_base2 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE2_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE2 =
    crate::Reg<cached_region_addr_base2::CACHED_REGION_ADDR_BASE2_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base2;
#[doc = "non_idempotent_region_addr_base3 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_ADDR_BASE3_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_ADDR_BASE3 =
    crate::Reg<non_idempotent_region_addr_base3::NON_IDEMPOTENT_REGION_ADDR_BASE3_SPEC>;
#[doc = ""]
pub mod non_idempotent_region_addr_base3;
#[doc = "non_idempotent_region_addr_base2 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_ADDR_BASE2_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_ADDR_BASE2 =
    crate::Reg<non_idempotent_region_addr_base2::NON_IDEMPOTENT_REGION_ADDR_BASE2_SPEC>;
#[doc = ""]
pub mod non_idempotent_region_addr_base2;
#[doc = "non_idempotent_region_addr_base1 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_ADDR_BASE1 =
    crate::Reg<non_idempotent_region_addr_base1::NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC>;
#[doc = ""]
pub mod non_idempotent_region_addr_base1;
#[doc = "non_idempotent_region_addr_base0 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_ADDR_BASE0_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_ADDR_BASE0 =
    crate::Reg<non_idempotent_region_addr_base0::NON_IDEMPOTENT_REGION_ADDR_BASE0_SPEC>;
#[doc = "Idempotent region base address"]
pub mod non_idempotent_region_addr_base0;
#[doc = "cached_region_length7 register accessor: an alias for `Reg<CACHED_REGION_LENGTH7_SPEC>`"]
pub type CACHED_REGION_LENGTH7 = crate::Reg<cached_region_length7::CACHED_REGION_LENGTH7_SPEC>;
#[doc = ""]
pub mod cached_region_length7;
#[doc = "cached_region_length6 register accessor: an alias for `Reg<CACHED_REGION_LENGTH6_SPEC>`"]
pub type CACHED_REGION_LENGTH6 = crate::Reg<cached_region_length6::CACHED_REGION_LENGTH6_SPEC>;
#[doc = ""]
pub mod cached_region_length6;
#[doc = "cached_region_length5 register accessor: an alias for `Reg<CACHED_REGION_LENGTH5_SPEC>`"]
pub type CACHED_REGION_LENGTH5 = crate::Reg<cached_region_length5::CACHED_REGION_LENGTH5_SPEC>;
#[doc = ""]
pub mod cached_region_length5;
#[doc = "cached_region_length4 register accessor: an alias for `Reg<CACHED_REGION_LENGTH4_SPEC>`"]
pub type CACHED_REGION_LENGTH4 = crate::Reg<cached_region_length4::CACHED_REGION_LENGTH4_SPEC>;
#[doc = ""]
pub mod cached_region_length4;
#[doc = "cached_region_length3 register accessor: an alias for `Reg<CACHED_REGION_LENGTH3_SPEC>`"]
pub type CACHED_REGION_LENGTH3 = crate::Reg<cached_region_length3::CACHED_REGION_LENGTH3_SPEC>;
#[doc = ""]
pub mod cached_region_length3;
#[doc = "cached_region_length2 register accessor: an alias for `Reg<CACHED_REGION_LENGTH2_SPEC>`"]
pub type CACHED_REGION_LENGTH2 = crate::Reg<cached_region_length2::CACHED_REGION_LENGTH2_SPEC>;
#[doc = ""]
pub mod cached_region_length2;
#[doc = "cached_region_addr_base1 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE1_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE1 =
    crate::Reg<cached_region_addr_base1::CACHED_REGION_ADDR_BASE1_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base1;
#[doc = "cached_region_addr_base0 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE0_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE0 =
    crate::Reg<cached_region_addr_base0::CACHED_REGION_ADDR_BASE0_SPEC>;
#[doc = "Cached region base address"]
pub mod cached_region_addr_base0;
#[doc = "execute_region_length7 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH7_SPEC>`"]
pub type EXECUTE_REGION_LENGTH7 = crate::Reg<execute_region_length7::EXECUTE_REGION_LENGTH7_SPEC>;
#[doc = ""]
pub mod execute_region_length7;
#[doc = "execute_region_length6 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH6_SPEC>`"]
pub type EXECUTE_REGION_LENGTH6 = crate::Reg<execute_region_length6::EXECUTE_REGION_LENGTH6_SPEC>;
#[doc = ""]
pub mod execute_region_length6;
#[doc = "execute_region_length5 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH5_SPEC>`"]
pub type EXECUTE_REGION_LENGTH5 = crate::Reg<execute_region_length5::EXECUTE_REGION_LENGTH5_SPEC>;
#[doc = ""]
pub mod execute_region_length5;
#[doc = "execute_region_length4 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH4_SPEC>`"]
pub type EXECUTE_REGION_LENGTH4 = crate::Reg<execute_region_length4::EXECUTE_REGION_LENGTH4_SPEC>;
#[doc = ""]
pub mod execute_region_length4;
#[doc = "execute_region_length3 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH3_SPEC>`"]
pub type EXECUTE_REGION_LENGTH3 = crate::Reg<execute_region_length3::EXECUTE_REGION_LENGTH3_SPEC>;
#[doc = ""]
pub mod execute_region_length3;
#[doc = "execute_region_addr_base2 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE2_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE2 =
    crate::Reg<execute_region_addr_base2::EXECUTE_REGION_ADDR_BASE2_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base2;
#[doc = "execute_region_addr_base1 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE1_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE1 =
    crate::Reg<execute_region_addr_base1::EXECUTE_REGION_ADDR_BASE1_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base1;
#[doc = "execute_region_addrbase0 register accessor: an alias for `Reg<EXECUTE_REGION_ADDRBASE0_SPEC>`"]
pub type EXECUTE_REGION_ADDRBASE0 =
    crate::Reg<execute_region_addrbase0::EXECUTE_REGION_ADDRBASE0_SPEC>;
#[doc = "Execution region base address"]
pub mod execute_region_addrbase0;
#[doc = "execute_region_length2 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH2_SPEC>`"]
pub type EXECUTE_REGION_LENGTH2 = crate::Reg<execute_region_length2::EXECUTE_REGION_LENGTH2_SPEC>;
#[doc = ""]
pub mod execute_region_length2;
#[doc = "non_idempotent_region_length3 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_LENGTH3_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_LENGTH3 =
    crate::Reg<non_idempotent_region_length3::NON_IDEMPOTENT_REGION_LENGTH3_SPEC>;
#[doc = ""]
pub mod non_idempotent_region_length3;
#[doc = "non_idempotent_region_length2 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_LENGTH2_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_LENGTH2 =
    crate::Reg<non_idempotent_region_length2::NON_IDEMPOTENT_REGION_LENGTH2_SPEC>;
#[doc = ""]
pub mod non_idempotent_region_length2;
#[doc = "non_idempotent_region_length1 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_LENGTH1_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_LENGTH1 =
    crate::Reg<non_idempotent_region_length1::NON_IDEMPOTENT_REGION_LENGTH1_SPEC>;
#[doc = ""]
pub mod non_idempotent_region_length1;
#[doc = "non_idempotent_region_length0 register accessor: an alias for `Reg<NON_IDEMPOTENT_REGION_LENGTH0_SPEC>`"]
pub type NON_IDEMPOTENT_REGION_LENGTH0 =
    crate::Reg<non_idempotent_region_length0::NON_IDEMPOTENT_REGION_LENGTH0_SPEC>;
#[doc = "Idempotent region address space length"]
pub mod non_idempotent_region_length0;
#[doc = "nr_nonidempotent_region_rules register accessor: an alias for `Reg<NR_NONIDEMPOTENT_REGION_RULES_SPEC>`"]
pub type NR_NONIDEMPOTENT_REGION_RULES =
    crate::Reg<nr_nonidempotent_region_rules::NR_NONIDEMPOTENT_REGION_RULES_SPEC>;
#[doc = "Number of idempotent regions"]
pub mod nr_nonidempotent_region_rules;
#[doc = "cached_region_addr_base7 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE7_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE7 =
    crate::Reg<cached_region_addr_base7::CACHED_REGION_ADDR_BASE7_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base7;
#[doc = "cached_region_addr_base6 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE6_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE6 =
    crate::Reg<cached_region_addr_base6::CACHED_REGION_ADDR_BASE6_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base6;
#[doc = "cached_region_addr_base5 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE5_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE5 =
    crate::Reg<cached_region_addr_base5::CACHED_REGION_ADDR_BASE5_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base5;
#[doc = "cached_region_addr_base4 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE4_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE4 =
    crate::Reg<cached_region_addr_base4::CACHED_REGION_ADDR_BASE4_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base4;
#[doc = "cached_region_addr_base3 register accessor: an alias for `Reg<CACHED_REGION_ADDR_BASE3_SPEC>`"]
pub type CACHED_REGION_ADDR_BASE3 =
    crate::Reg<cached_region_addr_base3::CACHED_REGION_ADDR_BASE3_SPEC>;
#[doc = ""]
pub mod cached_region_addr_base3;
#[doc = "cached_region_length1 register accessor: an alias for `Reg<CACHED_REGION_LENGTH1_SPEC>`"]
pub type CACHED_REGION_LENGTH1 = crate::Reg<cached_region_length1::CACHED_REGION_LENGTH1_SPEC>;
#[doc = ""]
pub mod cached_region_length1;
#[doc = "cached_region_length0 register accessor: an alias for `Reg<CACHED_REGION_LENGTH0_SPEC>`"]
pub type CACHED_REGION_LENGTH0 = crate::Reg<cached_region_length0::CACHED_REGION_LENGTH0_SPEC>;
#[doc = ""]
pub mod cached_region_length0;
#[doc = "nr_cached_region_rules register accessor: an alias for `Reg<NR_CACHED_REGION_RULES_SPEC>`"]
pub type NR_CACHED_REGION_RULES = crate::Reg<nr_cached_region_rules::NR_CACHED_REGION_RULES_SPEC>;
#[doc = "Number of cached regions"]
pub mod nr_cached_region_rules;
#[doc = "execute_region_addr_base7 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE7_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE7 =
    crate::Reg<execute_region_addr_base7::EXECUTE_REGION_ADDR_BASE7_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base7;
#[doc = "execute_region_addr_base6 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE6_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE6 =
    crate::Reg<execute_region_addr_base6::EXECUTE_REGION_ADDR_BASE6_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base6;
#[doc = "execute_region_addr_base5 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE5_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE5 =
    crate::Reg<execute_region_addr_base5::EXECUTE_REGION_ADDR_BASE5_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base5;
#[doc = "execute_region_addr_base4 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE4_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE4 =
    crate::Reg<execute_region_addr_base4::EXECUTE_REGION_ADDR_BASE4_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base4;
#[doc = "execute_region_addr_base3 register accessor: an alias for `Reg<EXECUTE_REGION_ADDR_BASE3_SPEC>`"]
pub type EXECUTE_REGION_ADDR_BASE3 =
    crate::Reg<execute_region_addr_base3::EXECUTE_REGION_ADDR_BASE3_SPEC>;
#[doc = ""]
pub mod execute_region_addr_base3;
#[doc = "execute_region_length1 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH1_SPEC>`"]
pub type EXECUTE_REGION_LENGTH1 = crate::Reg<execute_region_length1::EXECUTE_REGION_LENGTH1_SPEC>;
#[doc = ""]
pub mod execute_region_length1;
#[doc = "execute_region_length0 register accessor: an alias for `Reg<EXECUTE_REGION_LENGTH0_SPEC>`"]
pub type EXECUTE_REGION_LENGTH0 = crate::Reg<execute_region_length0::EXECUTE_REGION_LENGTH0_SPEC>;
#[doc = "Execution region address space length"]
pub mod execute_region_length0;
#[doc = "addr_valid_rule register accessor: an alias for `Reg<ADDR_VALID_RULE_SPEC>`"]
pub type ADDR_VALID_RULE = crate::Reg<addr_valid_rule::ADDR_VALID_RULE_SPEC>;
#[doc = "Valid address space flags"]
pub mod addr_valid_rule;
#[doc = "dram_base_addr register accessor: an alias for `Reg<DRAM_BASE_ADDR_SPEC>`"]
pub type DRAM_BASE_ADDR = crate::Reg<dram_base_addr::DRAM_BASE_ADDR_SPEC>;
#[doc = "External memory base address"]
pub mod dram_base_addr;
#[doc = "clustercfg_base_addr register accessor: an alias for `Reg<CLUSTERCFG_BASE_ADDR_SPEC>`"]
pub type CLUSTERCFG_BASE_ADDR = crate::Reg<clustercfg_base_addr::CLUSTERCFG_BASE_ADDR_SPEC>;
#[doc = "Cluster config base address"]
pub mod clustercfg_base_addr;
#[doc = "l2cfg_base_addr register accessor: an alias for `Reg<L2CFG_BASE_ADDR_SPEC>`"]
pub type L2CFG_BASE_ADDR = crate::Reg<l2cfg_base_addr::L2CFG_BASE_ADDR_SPEC>;
#[doc = "L2 cache config base address"]
pub mod l2cfg_base_addr;
#[doc = "debug_base_addr register accessor: an alias for `Reg<DEBUG_BASE_ADDR_SPEC>`"]
pub type DEBUG_BASE_ADDR = crate::Reg<debug_base_addr::DEBUG_BASE_ADDR_SPEC>;
#[doc = "Debugger base address"]
pub mod debug_base_addr;
#[doc = "plic_addr_length register accessor: an alias for `Reg<PLIC_ADDR_LENGTH_SPEC>`"]
pub type PLIC_ADDR_LENGTH = crate::Reg<plic_addr_length::PLIC_ADDR_LENGTH_SPEC>;
#[doc = "PLIC address space length"]
pub mod plic_addr_length;
#[doc = "rom_base_addr register accessor: an alias for `Reg<ROM_BASE_ADDR_SPEC>`"]
pub type ROM_BASE_ADDR = crate::Reg<rom_base_addr::ROM_BASE_ADDR_SPEC>;
#[doc = "Internal RAM base address"]
pub mod rom_base_addr;
#[doc = "clint_base_addr register accessor: an alias for `Reg<CLINT_BASE_ADDR_SPEC>`"]
pub type CLINT_BASE_ADDR = crate::Reg<clint_base_addr::CLINT_BASE_ADDR_SPEC>;
#[doc = "CLINT base address"]
pub mod clint_base_addr;
#[doc = "nr_execute_region_rules register accessor: an alias for `Reg<NR_EXECUTE_REGION_RULES_SPEC>`"]
pub type NR_EXECUTE_REGION_RULES =
    crate::Reg<nr_execute_region_rules::NR_EXECUTE_REGION_RULES_SPEC>;
#[doc = "Number of execute regions"]
pub mod nr_execute_region_rules;
#[doc = "dram_addr_length register accessor: an alias for `Reg<DRAM_ADDR_LENGTH_SPEC>`"]
pub type DRAM_ADDR_LENGTH = crate::Reg<dram_addr_length::DRAM_ADDR_LENGTH_SPEC>;
#[doc = "External memory address space length"]
pub mod dram_addr_length;
#[doc = "clustercfg_addr_length register accessor: an alias for `Reg<CLUSTERCFG_ADDR_LENGTH_SPEC>`"]
pub type CLUSTERCFG_ADDR_LENGTH = crate::Reg<clustercfg_addr_length::CLUSTERCFG_ADDR_LENGTH_SPEC>;
#[doc = "Cluster config address space length"]
pub mod clustercfg_addr_length;
#[doc = "l2cfg_addr_length register accessor: an alias for `Reg<L2CFG_ADDR_LENGTH_SPEC>`"]
pub type L2CFG_ADDR_LENGTH = crate::Reg<l2cfg_addr_length::L2CFG_ADDR_LENGTH_SPEC>;
#[doc = "L2 cache config address space length"]
pub mod l2cfg_addr_length;
#[doc = "debug_addr_length register accessor: an alias for `Reg<DEBUG_ADDR_LENGTH_SPEC>`"]
pub type DEBUG_ADDR_LENGTH = crate::Reg<debug_addr_length::DEBUG_ADDR_LENGTH_SPEC>;
#[doc = "Debugger address space length"]
pub mod debug_addr_length;
#[doc = "rom_addr_length register accessor: an alias for `Reg<ROM_ADDR_LENGTH_SPEC>`"]
pub type ROM_ADDR_LENGTH = crate::Reg<rom_addr_length::ROM_ADDR_LENGTH_SPEC>;
#[doc = "Internal RAM address space length"]
pub mod rom_addr_length;
