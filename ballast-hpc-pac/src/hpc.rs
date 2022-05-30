#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0020_0000],
    #[doc = "0x200000..0x20c000 - CLINT"]
    pub clint: CLINT,
    _reserved1: [u8; 0x4000],
    #[doc = "0x210000..0x21001c - Timer"]
    pub timer: TIMER,
    _reserved2: [u8; 0xffe4],
    #[doc = "0x220000..0x220028 - l2_config"]
    pub l2_config: L2_CONFIG,
    _reserved3: [u8; 0xffd8],
    #[doc = "0x230000..0x230228 - cluster_config"]
    pub cluster_config: CLUSTER_CONFIG,
    _reserved4: [u8; 0x00dc_fdd8],
    #[doc = "0x1000000..0x1203008 - PLIC"]
    pub plic: PLIC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CLINT {
    #[doc = "0x00..0x08 - Machine mode software interrupt (IPI)"]
    pub msip: crate::Reg<self::clint::msip::MSIP_SPEC>,
    _reserved1: [u8; 0x3ff8],
    #[doc = "0x4000..0x4008 - Machine mode timer compare register for Hart 0"]
    pub mtimecmp_0: crate::Reg<self::clint::mtimecmp_0::MTIMECMP_0_SPEC>,
    #[doc = "0x4008..0x4010 - Machine mode timer compare register for Hart 0"]
    pub mtimecmp_1: crate::Reg<self::clint::mtimecmp_1::MTIMECMP_1_SPEC>,
    _reserved3: [u8; 0x7fe8],
    #[doc = "0xbff8..0xc000 - Timer register"]
    pub mtime: crate::Reg<self::clint::mtime::MTIME_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CLINT"]
pub mod clint;
#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - "]
    pub timer0_timer: crate::Reg<self::timer::timer0_timer::TIMER0_TIMER_SPEC>,
    #[doc = "0x04 - "]
    pub timer0_ctrl: crate::Reg<self::timer::timer0_ctrl::TIMER0_CTRL_SPEC>,
    #[doc = "0x08 - "]
    pub timer0_cmp: crate::Reg<self::timer::timer0_cmp::TIMER0_CMP_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - "]
    pub timer1_timer: crate::Reg<self::timer::timer1_timer::TIMER1_TIMER_SPEC>,
    #[doc = "0x14 - "]
    pub timer1_ctrl: crate::Reg<self::timer::timer1_ctrl::TIMER1_CTRL_SPEC>,
    #[doc = "0x18 - "]
    pub timer1_cmp: crate::Reg<self::timer::timer1_cmp::TIMER1_CMP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Timer"]
pub mod timer;
#[doc = r"Register block"]
#[repr(C)]
pub struct L2_CONFIG {
    #[doc = "0x00..0x08 - Enable L2 cache"]
    pub l2_enable: crate::Reg<self::l2_config::l2_enable::L2_ENABLE_SPEC>,
    #[doc = "0x08..0x10 - Enable shadow L1 caches in L2;NI"]
    pub core_enable: crate::Reg<self::l2_config::core_enable::CORE_ENABLE_SPEC>,
    #[doc = "0x10..0x18 - Select L2 core priority algorithm; NI"]
    pub l2_replacement_policy:
        crate::Reg<self::l2_config::l2_replacement_policy::L2_REPLACEMENT_POLICY_SPEC>,
    #[doc = "0x18..0x20 - Select L2 core priorities; NI"]
    pub l2_arbitration_policy:
        crate::Reg<self::l2_config::l2_arbitration_policy::L2_ARBITRATION_POLICY_SPEC>,
    #[doc = "0x20..0x28 - Select L2 replacement algorithm; NI"]
    pub l2_arbitration_priority:
        crate::Reg<self::l2_config::l2_arbitration_priority::L2_ARBITRATION_PRIORITY_SPEC>,
}
#[doc = r"Register block"]
#[doc = "l2_config"]
pub mod l2_config;
#[doc = r"Register block"]
#[repr(C)]
pub struct CLUSTER_CONFIG { # [ doc = "0x00..0x08 - Core#0 boot address" ] pub core0_bootaddr : crate :: Reg < self :: cluster_config :: core0_bootaddr :: CORE0_BOOTADDR_SPEC > , # [ doc = "0x08..0x10 - Core#1 boot address" ] pub core1_bootaddr : crate :: Reg < self :: cluster_config :: core1_bootaddr :: CORE1_BOOTADDR_SPEC > , # [ doc = "0x10..0x18 - Core#0 hart ID" ] pub core0_hart_id : crate :: Reg < self :: cluster_config :: core0_hart_id :: CORE0_HART_ID_SPEC > , # [ doc = "0x18..0x20 - Core#1 hart ID" ] pub core1_hart_id : crate :: Reg < self :: cluster_config :: core1_hart_id :: CORE1_HART_ID_SPEC > , # [ doc = "0x20..0x28 - real time clock generator clock high count" ] pub rtc_cfg0 : crate :: Reg < self :: cluster_config :: rtc_cfg0 :: RTC_CFG0_SPEC > , # [ doc = "0x28..0x30 - real time clock generator clock low count" ] pub rtc_cfg1 : crate :: Reg < self :: cluster_config :: rtc_cfg1 :: RTC_CFG1_SPEC > , # [ doc = "0x30..0x38 - real time clock generator clock fine tune" ] pub rtc_cfg2 : crate :: Reg < self :: cluster_config :: rtc_cfg2 :: RTC_CFG2_SPEC > , # [ doc = "0x38..0x40 - interrupt flag statuses; NI" ] pub irq_status : crate :: Reg < self :: cluster_config :: irq_status :: IRQ_STATUS_SPEC > , # [ doc = "0x40..0x48 - force interrupt line active; NI" ] pub force_irq : crate :: Reg < self :: cluster_config :: force_irq :: FORCE_IRQ_SPEC > , # [ doc = "0x48..0x50 - Timer base address" ] pub timer_base_addr : crate :: Reg < self :: cluster_config :: timer_base_addr :: TIMER_BASE_ADDR_SPEC > , # [ doc = "0x50..0x58 - Timer address space length" ] pub timer_addr_length : crate :: Reg < self :: cluster_config :: timer_addr_length :: TIMER_ADDR_LENGTH_SPEC > , # [ doc = "0x58..0x60 - PLIC base address" ] pub plic_base_addr : crate :: Reg < self :: cluster_config :: plic_base_addr :: PLIC_BASE_ADDR_SPEC > , # [ doc = "0x60..0x68 - PLIC address space length" ] pub plic_addr_length : crate :: Reg < self :: cluster_config :: plic_addr_length :: PLIC_ADDR_LENGTH_SPEC > , # [ doc = "0x68..0x70 - CLINT base address" ] pub clint_base_addr : crate :: Reg < self :: cluster_config :: clint_base_addr :: CLINT_BASE_ADDR_SPEC > , # [ doc = "0x70..0x78 - CLINT address space length" ] pub clint_addr_length : crate :: Reg < self :: cluster_config :: clint_addr_length :: CLINT_ADDR_LENGTH_SPEC > , # [ doc = "0x78..0x80 - Internal RAM base address" ] pub rom_base_addr : crate :: Reg < self :: cluster_config :: rom_base_addr :: ROM_BASE_ADDR_SPEC > , # [ doc = "0x80..0x88 - Internal RAM address space length" ] pub rom_addr_length : crate :: Reg < self :: cluster_config :: rom_addr_length :: ROM_ADDR_LENGTH_SPEC > , # [ doc = "0x88..0x90 - Debugger base address" ] pub debug_base_addr : crate :: Reg < self :: cluster_config :: debug_base_addr :: DEBUG_BASE_ADDR_SPEC > , # [ doc = "0x90..0x98 - Debugger address space length" ] pub debug_addr_length : crate :: Reg < self :: cluster_config :: debug_addr_length :: DEBUG_ADDR_LENGTH_SPEC > , # [ doc = "0x98..0xa0 - L2 cache config base address" ] pub l2cfg_base_addr : crate :: Reg < self :: cluster_config :: l2cfg_base_addr :: L2CFG_BASE_ADDR_SPEC > , # [ doc = "0xa0..0xa8 - L2 cache config address space length" ] pub l2cfg_addr_length : crate :: Reg < self :: cluster_config :: l2cfg_addr_length :: L2CFG_ADDR_LENGTH_SPEC > , # [ doc = "0xa8..0xb0 - Cluster config base address" ] pub clustercfg_base_addr : crate :: Reg < self :: cluster_config :: clustercfg_base_addr :: CLUSTERCFG_BASE_ADDR_SPEC > , # [ doc = "0xb0..0xb8 - Cluster config address space length" ] pub clustercfg_addr_length : crate :: Reg < self :: cluster_config :: clustercfg_addr_length :: CLUSTERCFG_ADDR_LENGTH_SPEC > , # [ doc = "0xb8..0xc0 - External memory base address" ] pub dram_base_addr : crate :: Reg < self :: cluster_config :: dram_base_addr :: DRAM_BASE_ADDR_SPEC > , # [ doc = "0xc0..0xc8 - External memory address space length" ] pub dram_addr_length : crate :: Reg < self :: cluster_config :: dram_addr_length :: DRAM_ADDR_LENGTH_SPEC > , # [ doc = "0xc8..0xd0 - Valid address space flags" ] pub addr_valid_rule : crate :: Reg < self :: cluster_config :: addr_valid_rule :: ADDR_VALID_RULE_SPEC > , # [ doc = "0xd0..0xd8 - Number of execute regions" ] pub nr_execute_region_rules : crate :: Reg < self :: cluster_config :: nr_execute_region_rules :: NR_EXECUTE_REGION_RULES_SPEC > , # [ doc = "0xd8..0xe0 - Execution region base address" ] pub execute_region_addrbase0 : crate :: Reg < self :: cluster_config :: execute_region_addrbase0 :: EXECUTE_REGION_ADDRBASE0_SPEC > , # [ doc = "0xe0..0xe8 - Execution region address space length" ] pub execute_region_length0 : crate :: Reg < self :: cluster_config :: execute_region_length0 :: EXECUTE_REGION_LENGTH0_SPEC > , # [ doc = "0xe8..0xf0 - " ] pub execute_region_addr_base1 : crate :: Reg < self :: cluster_config :: execute_region_addr_base1 :: EXECUTE_REGION_ADDR_BASE1_SPEC > , # [ doc = "0xf0..0xf8 - " ] pub execute_region_length1 : crate :: Reg < self :: cluster_config :: execute_region_length1 :: EXECUTE_REGION_LENGTH1_SPEC > , # [ doc = "0xf8..0x100 - " ] pub execute_region_addr_base2 : crate :: Reg < self :: cluster_config :: execute_region_addr_base2 :: EXECUTE_REGION_ADDR_BASE2_SPEC > , # [ doc = "0x100..0x108 - " ] pub execute_region_length2 : crate :: Reg < self :: cluster_config :: execute_region_length2 :: EXECUTE_REGION_LENGTH2_SPEC > , # [ doc = "0x108..0x110 - " ] pub execute_region_addr_base3 : crate :: Reg < self :: cluster_config :: execute_region_addr_base3 :: EXECUTE_REGION_ADDR_BASE3_SPEC > , # [ doc = "0x110..0x118 - " ] pub execute_region_length3 : crate :: Reg < self :: cluster_config :: execute_region_length3 :: EXECUTE_REGION_LENGTH3_SPEC > , # [ doc = "0x118..0x120 - " ] pub execute_region_addr_base4 : crate :: Reg < self :: cluster_config :: execute_region_addr_base4 :: EXECUTE_REGION_ADDR_BASE4_SPEC > , # [ doc = "0x120..0x128 - " ] pub execute_region_length4 : crate :: Reg < self :: cluster_config :: execute_region_length4 :: EXECUTE_REGION_LENGTH4_SPEC > , # [ doc = "0x128..0x130 - " ] pub execute_region_addr_base5 : crate :: Reg < self :: cluster_config :: execute_region_addr_base5 :: EXECUTE_REGION_ADDR_BASE5_SPEC > , # [ doc = "0x130..0x138 - " ] pub execute_region_length5 : crate :: Reg < self :: cluster_config :: execute_region_length5 :: EXECUTE_REGION_LENGTH5_SPEC > , # [ doc = "0x138..0x140 - " ] pub execute_region_addr_base6 : crate :: Reg < self :: cluster_config :: execute_region_addr_base6 :: EXECUTE_REGION_ADDR_BASE6_SPEC > , # [ doc = "0x140..0x148 - " ] pub execute_region_length6 : crate :: Reg < self :: cluster_config :: execute_region_length6 :: EXECUTE_REGION_LENGTH6_SPEC > , # [ doc = "0x148..0x150 - " ] pub execute_region_addr_base7 : crate :: Reg < self :: cluster_config :: execute_region_addr_base7 :: EXECUTE_REGION_ADDR_BASE7_SPEC > , # [ doc = "0x150..0x158 - " ] pub execute_region_length7 : crate :: Reg < self :: cluster_config :: execute_region_length7 :: EXECUTE_REGION_LENGTH7_SPEC > , # [ doc = "0x158..0x160 - Number of cached regions" ] pub nr_cached_region_rules : crate :: Reg < self :: cluster_config :: nr_cached_region_rules :: NR_CACHED_REGION_RULES_SPEC > , # [ doc = "0x160..0x168 - Cached region base address" ] pub cached_region_addr_base0 : crate :: Reg < self :: cluster_config :: cached_region_addr_base0 :: CACHED_REGION_ADDR_BASE0_SPEC > , # [ doc = "0x168..0x170 - " ] pub cached_region_length0 : crate :: Reg < self :: cluster_config :: cached_region_length0 :: CACHED_REGION_LENGTH0_SPEC > , # [ doc = "0x170..0x178 - " ] pub cached_region_addr_base1 : crate :: Reg < self :: cluster_config :: cached_region_addr_base1 :: CACHED_REGION_ADDR_BASE1_SPEC > , # [ doc = "0x178..0x180 - " ] pub cached_region_length1 : crate :: Reg < self :: cluster_config :: cached_region_length1 :: CACHED_REGION_LENGTH1_SPEC > , # [ doc = "0x180..0x188 - " ] pub cached_region_addr_base2 : crate :: Reg < self :: cluster_config :: cached_region_addr_base2 :: CACHED_REGION_ADDR_BASE2_SPEC > , # [ doc = "0x188..0x190 - " ] pub cached_region_length2 : crate :: Reg < self :: cluster_config :: cached_region_length2 :: CACHED_REGION_LENGTH2_SPEC > , # [ doc = "0x190..0x198 - " ] pub cached_region_addr_base3 : crate :: Reg < self :: cluster_config :: cached_region_addr_base3 :: CACHED_REGION_ADDR_BASE3_SPEC > , # [ doc = "0x198..0x1a0 - " ] pub cached_region_length3 : crate :: Reg < self :: cluster_config :: cached_region_length3 :: CACHED_REGION_LENGTH3_SPEC > , # [ doc = "0x1a0..0x1a8 - " ] pub cached_region_addr_base4 : crate :: Reg < self :: cluster_config :: cached_region_addr_base4 :: CACHED_REGION_ADDR_BASE4_SPEC > , # [ doc = "0x1a8..0x1b0 - " ] pub cached_region_length4 : crate :: Reg < self :: cluster_config :: cached_region_length4 :: CACHED_REGION_LENGTH4_SPEC > , # [ doc = "0x1b0..0x1b8 - " ] pub cached_region_addr_base5 : crate :: Reg < self :: cluster_config :: cached_region_addr_base5 :: CACHED_REGION_ADDR_BASE5_SPEC > , # [ doc = "0x1b8..0x1c0 - " ] pub cached_region_length5 : crate :: Reg < self :: cluster_config :: cached_region_length5 :: CACHED_REGION_LENGTH5_SPEC > , # [ doc = "0x1c0..0x1c8 - " ] pub cached_region_addr_base6 : crate :: Reg < self :: cluster_config :: cached_region_addr_base6 :: CACHED_REGION_ADDR_BASE6_SPEC > , # [ doc = "0x1c8..0x1d0 - " ] pub cached_region_length6 : crate :: Reg < self :: cluster_config :: cached_region_length6 :: CACHED_REGION_LENGTH6_SPEC > , # [ doc = "0x1d0..0x1d8 - " ] pub cached_region_addr_base7 : crate :: Reg < self :: cluster_config :: cached_region_addr_base7 :: CACHED_REGION_ADDR_BASE7_SPEC > , # [ doc = "0x1d8..0x1e0 - " ] pub cached_region_length7 : crate :: Reg < self :: cluster_config :: cached_region_length7 :: CACHED_REGION_LENGTH7_SPEC > , # [ doc = "0x1e0..0x1e8 - Number of idempotent regions" ] pub nr_nonidempotent_region_rules : crate :: Reg < self :: cluster_config :: nr_nonidempotent_region_rules :: NR_NONIDEMPOTENT_REGION_RULES_SPEC > , # [ doc = "0x1e8..0x1f0 - Idempotent region base address" ] pub non_idempotent_region_addr_base0 : crate :: Reg < self :: cluster_config :: non_idempotent_region_addr_base0 :: NON_IDEMPOTENT_REGION_ADDR_BASE0_SPEC > , # [ doc = "0x1f0..0x1f8 - Idempotent region address space length" ] pub non_idempotent_region_length0 : crate :: Reg < self :: cluster_config :: non_idempotent_region_length0 :: NON_IDEMPOTENT_REGION_LENGTH0_SPEC > , # [ doc = "0x1f8..0x200 - " ] pub non_idempotent_region_addr_base1 : crate :: Reg < self :: cluster_config :: non_idempotent_region_addr_base1 :: NON_IDEMPOTENT_REGION_ADDR_BASE1_SPEC > , # [ doc = "0x200..0x208 - " ] pub non_idempotent_region_length1 : crate :: Reg < self :: cluster_config :: non_idempotent_region_length1 :: NON_IDEMPOTENT_REGION_LENGTH1_SPEC > , # [ doc = "0x208..0x210 - " ] pub non_idempotent_region_addr_base2 : crate :: Reg < self :: cluster_config :: non_idempotent_region_addr_base2 :: NON_IDEMPOTENT_REGION_ADDR_BASE2_SPEC > , # [ doc = "0x210..0x218 - " ] pub non_idempotent_region_length2 : crate :: Reg < self :: cluster_config :: non_idempotent_region_length2 :: NON_IDEMPOTENT_REGION_LENGTH2_SPEC > , # [ doc = "0x218..0x220 - " ] pub non_idempotent_region_addr_base3 : crate :: Reg < self :: cluster_config :: non_idempotent_region_addr_base3 :: NON_IDEMPOTENT_REGION_ADDR_BASE3_SPEC > , # [ doc = "0x220..0x228 - " ] pub non_idempotent_region_length3 : crate :: Reg < self :: cluster_config :: non_idempotent_region_length3 :: NON_IDEMPOTENT_REGION_LENGTH3_SPEC > , }
#[doc = r"Register block"]
#[doc = "cluster_config"]
pub mod cluster_config;
#[doc = r"Register block"]
#[repr(C)]
pub struct PLIC {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - "]
    pub timer_0_int_priority_0:
        crate::Reg<self::plic::timer_0_int_priority_0::TIMER_0_INT_PRIORITY_0_SPEC>,
    #[doc = "0x08 - "]
    pub timer_0_int_priority_1:
        crate::Reg<self::plic::timer_0_int_priority_1::TIMER_0_INT_PRIORITY_1_SPEC>,
    #[doc = "0x0c - "]
    pub timer_1_int_priority_0:
        crate::Reg<self::plic::timer_1_int_priority_0::TIMER_1_INT_PRIORITY_0_SPEC>,
    #[doc = "0x10 - "]
    pub timer_1_int_priority_1:
        crate::Reg<self::plic::timer_1_int_priority_1::TIMER_1_INT_PRIORITY_1_SPEC>,
    #[doc = "0x14 - "]
    pub external_0_int_priority_0:
        crate::Reg<self::plic::external_0_int_priority_0::EXTERNAL_0_INT_PRIORITY_0_SPEC>,
    #[doc = "0x18 - "]
    pub external_0_int_priority_1:
        crate::Reg<self::plic::external_0_int_priority_1::EXTERNAL_0_INT_PRIORITY_1_SPEC>,
    _reserved6: [u8; 0x1fe4],
    #[doc = "0x2000 - "]
    pub enable_context_0: crate::Reg<self::plic::enable_context_0::ENABLE_CONTEXT_0_SPEC>,
    _reserved7: [u8; 0x7c],
    #[doc = "0x2080 - "]
    pub enable_context_1: crate::Reg<self::plic::enable_context_1::ENABLE_CONTEXT_1_SPEC>,
    _reserved8: [u8; 0x7c],
    #[doc = "0x2100 - "]
    pub enable_context_2: crate::Reg<self::plic::enable_context_2::ENABLE_CONTEXT_2_SPEC>,
    _reserved9: [u8; 0x7c],
    #[doc = "0x2180 - "]
    pub enable_context_3: crate::Reg<self::plic::enable_context_3::ENABLE_CONTEXT_3_SPEC>,
    _reserved10: [u8; 0x001f_de7c],
    #[doc = "0x200000 - "]
    pub priority_threshold_context_0:
        crate::Reg<self::plic::priority_threshold_context_0::PRIORITY_THRESHOLD_CONTEXT_0_SPEC>,
    #[doc = "0x200004 - "]
    pub claim_complete_context_0:
        crate::Reg<self::plic::claim_complete_context_0::CLAIM_COMPLETE_CONTEXT_0_SPEC>,
    _reserved12: [u8; 0x0ff8],
    #[doc = "0x201000 - "]
    pub priority_threshold_context_1:
        crate::Reg<self::plic::priority_threshold_context_1::PRIORITY_THRESHOLD_CONTEXT_1_SPEC>,
    #[doc = "0x201004 - "]
    pub claim_complete_context_1:
        crate::Reg<self::plic::claim_complete_context_1::CLAIM_COMPLETE_CONTEXT_1_SPEC>,
    _reserved14: [u8; 0x0ff8],
    #[doc = "0x202000 - "]
    pub priority_threshold_context_2:
        crate::Reg<self::plic::priority_threshold_context_2::PRIORITY_THRESHOLD_CONTEXT_2_SPEC>,
    #[doc = "0x202004 - "]
    pub claim_complete_context_2:
        crate::Reg<self::plic::claim_complete_context_2::CLAIM_COMPLETE_CONTEXT_2_SPEC>,
    _reserved16: [u8; 0x0ff8],
    #[doc = "0x203000 - "]
    pub priority_threshold_context_3:
        crate::Reg<self::plic::priority_threshold_context_3::PRIORITY_THRESHOLD_CONTEXT_3_SPEC>,
    #[doc = "0x203004 - "]
    pub claim_complete_context_3:
        crate::Reg<self::plic::claim_complete_context_3::CLAIM_COMPLETE_CONTEXT_3_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PLIC"]
pub mod plic;
