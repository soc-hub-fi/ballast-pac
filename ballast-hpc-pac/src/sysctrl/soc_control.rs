#[doc = r"Register block"]
#[repr(C)]
pub struct SOC_CONTROL {
    #[doc = "0x00 - This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register."]
    pub info: INFO,
    #[doc = "0x04 - This register holds the boot address."]
    pub fcboot: FCBOOT,
    #[doc = "0x08 - This register contains the value of the fetch enable signal of the core."]
    pub fcfetch: FCFETCH,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\])."]
    pub pad_mux: PAD_MUX,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - All 10 bit fields have reset value of 10'b10_0011_0100: 0 drive strenght 1 drive strenght 2 trigger 3 trigger 4 rate 5 output en(0) 6 hold 7 pull enable 8 pd(0)/pu(1) 9 input en(1)"]
    pub pad_cfg_0: PAD_CFG_0,
    #[doc = "0x24 - "]
    pub pad_cfg_1: PAD_CFG_1,
    #[doc = "0x28 - "]
    pub pad_cfg_2: PAD_CFG_2,
    #[doc = "0x2c - "]
    pub pad_cfg_3: PAD_CFG_3,
    #[doc = "0x30 - "]
    pub pad_cfg_4: PAD_CFG_4,
    #[doc = "0x34 - "]
    pub pad_cfg_5: PAD_CFG_5,
    _reserved10: [u8; 0x28],
    #[doc = "0x60 - "]
    pub tta_pll_loop_ctrl: TTA_PLL_LOOP_CTRL,
    #[doc = "0x64 - "]
    pub tta_pll_div: TTA_PLL_DIV,
    #[doc = "0x68 - "]
    pub tta_pll_debug_ctrl: TTA_PLL_DEBUG_CTRL,
    #[doc = "0x6c - "]
    pub tta_pll_enable: TTA_PLL_ENABLE,
    #[doc = "0x70 - "]
    pub cluster_ctrl: CLUSTER_CTRL,
    #[doc = "0x74 - Register to read or write from JTAG"]
    pub jtagreg: JTAGREG,
    #[doc = "0x78 - "]
    pub ctrl_per: CTRL_PER,
    #[doc = "0x7c - "]
    pub cluster_irq: CLUSTER_IRQ,
    #[doc = "0x80 - "]
    pub cluster_boot_addr0: CLUSTER_BOOT_ADDR0,
    #[doc = "0x84 - "]
    pub cluster_boot_addr1: CLUSTER_BOOT_ADDR1,
    #[doc = "0x88 - "]
    pub tta_pll_spare_ctrl: TTA_PLL_SPARE_CTRL,
    #[doc = "0x8c - "]
    pub tta_pll_tmux_sel: TTA_PLL_TMUX_SEL,
    #[doc = "0x90 - "]
    pub tta_pll_status1: TTA_PLL_STATUS1,
    #[doc = "0x94 - "]
    pub topperiph_clk_div: TOPPERIPH_CLK_DIV,
    #[doc = "0x98 - Subsystem Clock selection. Bit definition for TTA, Ethernet, AI, HPC subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub clk_ctrl1: CLK_CTRL1,
    #[doc = "0x9c - Subsystem Clock selection. Bit definition for MPC, Interconnect, C2C and CoreHW subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub clk_ctrl2: CLK_CTRL2,
    #[doc = "0xa0 - These 2 registers (CORESTATUS, CS_RO) contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
    pub corestatus: CORESTATUS,
    #[doc = "0xa4 - "]
    pub slow_pulse_div: SLOW_PULSE_DIV,
    #[doc = "0xa8 - "]
    pub periph_clk_div: PERIPH_CLK_DIV,
    _reserved29: [u8; 0x04],
    #[doc = "0xb0 - Assertion of the Subsystem bit, deasserts the reset to the corresponding subsystem. Bit definition is mentioned in adjoining table"]
    pub ss_reset_en: SS_RESET_EN,
    #[doc = "0xb4 - Subsystem clock enable register"]
    pub ss_clk_en: SS_CLK_EN,
    #[doc = "0xb8 - Subsystem Clock selection. Bit definition for Top peripheral subsystem. *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
    pub clk_ctrl3: CLK_CTRL3,
    #[doc = "0xbc - "]
    pub tta_pll_status2: TTA_PLL_STATUS2,
    #[doc = "0xc0 - "]
    pub cs_ro: CS_RO,
    #[doc = "0xc4 - Boot Sel value"]
    pub bootsel: BOOTSEL,
    #[doc = "0xc8 - "]
    pub clksel: CLKSEL,
    #[doc = "0xcc - Clock divider ratio for the 3 Interconnect modules"]
    pub inter_clk_div: INTER_CLK_DIV,
}
#[doc = "INFO (rw) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register."]
pub mod info;
#[doc = "FCBOOT (rw) register accessor: an alias for `Reg<FCBOOT_SPEC>`"]
pub type FCBOOT = crate::Reg<fcboot::FCBOOT_SPEC>;
#[doc = "This register holds the boot address."]
pub mod fcboot;
#[doc = "FCFETCH (w) register accessor: an alias for `Reg<FCFETCH_SPEC>`"]
pub type FCFETCH = crate::Reg<fcfetch::FCFETCH_SPEC>;
#[doc = "This register contains the value of the fetch enable signal of the core."]
pub mod fcfetch;
#[doc = "PAD_MUX (rw) register accessor: an alias for `Reg<PAD_MUX_SPEC>`"]
pub type PAD_MUX = crate::Reg<pad_mux::PAD_MUX_SPEC>;
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\])."]
pub mod pad_mux;
#[doc = "CLUSTER_BOOT_ADDR0 (rw) register accessor: an alias for `Reg<CLUSTER_BOOT_ADDR0_SPEC>`"]
pub type CLUSTER_BOOT_ADDR0 = crate::Reg<cluster_boot_addr0::CLUSTER_BOOT_ADDR0_SPEC>;
#[doc = ""]
pub mod cluster_boot_addr0;
#[doc = "CS_RO (r) register accessor: an alias for `Reg<CS_RO_SPEC>`"]
pub type CS_RO = crate::Reg<cs_ro::CS_RO_SPEC>;
#[doc = ""]
pub mod cs_ro;
#[doc = "BOOTSEL (r) register accessor: an alias for `Reg<BOOTSEL_SPEC>`"]
pub type BOOTSEL = crate::Reg<bootsel::BOOTSEL_SPEC>;
#[doc = "Boot Sel value"]
pub mod bootsel;
#[doc = "CLUSTER_BOOT_ADDR1 (rw) register accessor: an alias for `Reg<CLUSTER_BOOT_ADDR1_SPEC>`"]
pub type CLUSTER_BOOT_ADDR1 = crate::Reg<cluster_boot_addr1::CLUSTER_BOOT_ADDR1_SPEC>;
#[doc = ""]
pub mod cluster_boot_addr1;
#[doc = "JTAGREG (rw) register accessor: an alias for `Reg<JTAGREG_SPEC>`"]
pub type JTAGREG = crate::Reg<jtagreg::JTAGREG_SPEC>;
#[doc = "Register to read or write from JTAG"]
pub mod jtagreg;
#[doc = "CTRL_PER (rw) register accessor: an alias for `Reg<CTRL_PER_SPEC>`"]
pub type CTRL_PER = crate::Reg<ctrl_per::CTRL_PER_SPEC>;
#[doc = ""]
pub mod ctrl_per;
#[doc = "CLKSEL (r) register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = ""]
pub mod clksel;
#[doc = "CORESTATUS (rw) register accessor: an alias for `Reg<CORESTATUS_SPEC>`"]
pub type CORESTATUS = crate::Reg<corestatus::CORESTATUS_SPEC>;
#[doc = "These 2 registers (CORESTATUS, CS_RO) contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
pub mod corestatus;
#[doc = "CLUSTER_IRQ (rw) register accessor: an alias for `Reg<CLUSTER_IRQ_SPEC>`"]
pub type CLUSTER_IRQ = crate::Reg<cluster_irq::CLUSTER_IRQ_SPEC>;
#[doc = ""]
pub mod cluster_irq;
#[doc = "PAD_CFG_1 (rw) register accessor: an alias for `Reg<PAD_CFG_1_SPEC>`"]
pub type PAD_CFG_1 = crate::Reg<pad_cfg_1::PAD_CFG_1_SPEC>;
#[doc = ""]
pub mod pad_cfg_1;
#[doc = "CLUSTER_CTRL (rw) register accessor: an alias for `Reg<CLUSTER_CTRL_SPEC>`"]
pub type CLUSTER_CTRL = crate::Reg<cluster_ctrl::CLUSTER_CTRL_SPEC>;
#[doc = ""]
pub mod cluster_ctrl;
#[doc = "TTA_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<TTA_PLL_LOOP_CTRL_SPEC>`"]
pub type TTA_PLL_LOOP_CTRL = crate::Reg<tta_pll_loop_ctrl::TTA_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod tta_pll_loop_ctrl;
#[doc = "TTA_PLL_DIV (rw) register accessor: an alias for `Reg<TTA_PLL_DIV_SPEC>`"]
pub type TTA_PLL_DIV = crate::Reg<tta_pll_div::TTA_PLL_DIV_SPEC>;
#[doc = ""]
pub mod tta_pll_div;
#[doc = "TTA_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<TTA_PLL_DEBUG_CTRL_SPEC>`"]
pub type TTA_PLL_DEBUG_CTRL = crate::Reg<tta_pll_debug_ctrl::TTA_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod tta_pll_debug_ctrl;
#[doc = "TTA_PLL_ENABLE (rw) register accessor: an alias for `Reg<TTA_PLL_ENABLE_SPEC>`"]
pub type TTA_PLL_ENABLE = crate::Reg<tta_pll_enable::TTA_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod tta_pll_enable;
#[doc = "TTA_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<TTA_PLL_SPARE_CTRL_SPEC>`"]
pub type TTA_PLL_SPARE_CTRL = crate::Reg<tta_pll_spare_ctrl::TTA_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod tta_pll_spare_ctrl;
#[doc = "TTA_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<TTA_PLL_TMUX_SEL_SPEC>`"]
pub type TTA_PLL_TMUX_SEL = crate::Reg<tta_pll_tmux_sel::TTA_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod tta_pll_tmux_sel;
#[doc = "TTA_PLL_STATUS1 (r) register accessor: an alias for `Reg<TTA_PLL_STATUS1_SPEC>`"]
pub type TTA_PLL_STATUS1 = crate::Reg<tta_pll_status1::TTA_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod tta_pll_status1;
#[doc = "PAD_CFG_0 (rw) register accessor: an alias for `Reg<PAD_CFG_0_SPEC>`"]
pub type PAD_CFG_0 = crate::Reg<pad_cfg_0::PAD_CFG_0_SPEC>;
#[doc = "All 10 bit fields have reset value of 10'b10_0011_0100: 0 drive strenght 1 drive strenght 2 trigger 3 trigger 4 rate 5 output en(0) 6 hold 7 pull enable 8 pd(0)/pu(1) 9 input en(1)"]
pub mod pad_cfg_0;
#[doc = "INTER_CLK_DIV (rw) register accessor: an alias for `Reg<INTER_CLK_DIV_SPEC>`"]
pub type INTER_CLK_DIV = crate::Reg<inter_clk_div::INTER_CLK_DIV_SPEC>;
#[doc = "Clock divider ratio for the 3 Interconnect modules"]
pub mod inter_clk_div;
#[doc = "PERIPH_CLK_DIV (rw) register accessor: an alias for `Reg<PERIPH_CLK_DIV_SPEC>`"]
pub type PERIPH_CLK_DIV = crate::Reg<periph_clk_div::PERIPH_CLK_DIV_SPEC>;
#[doc = ""]
pub mod periph_clk_div;
#[doc = "TTA_PLL_STATUS2 (r) register accessor: an alias for `Reg<TTA_PLL_STATUS2_SPEC>`"]
pub type TTA_PLL_STATUS2 = crate::Reg<tta_pll_status2::TTA_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod tta_pll_status2;
#[doc = "TOPPERIPH_CLK_DIV (rw) register accessor: an alias for `Reg<TOPPERIPH_CLK_DIV_SPEC>`"]
pub type TOPPERIPH_CLK_DIV = crate::Reg<topperiph_clk_div::TOPPERIPH_CLK_DIV_SPEC>;
#[doc = ""]
pub mod topperiph_clk_div;
#[doc = "CLK_CTRL1 (rw) register accessor: an alias for `Reg<CLK_CTRL1_SPEC>`"]
pub type CLK_CTRL1 = crate::Reg<clk_ctrl1::CLK_CTRL1_SPEC>;
#[doc = "Subsystem Clock selection. Bit definition for TTA, Ethernet, AI, HPC subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
pub mod clk_ctrl1;
#[doc = "CLK_CTRL2 (rw) register accessor: an alias for `Reg<CLK_CTRL2_SPEC>`"]
pub type CLK_CTRL2 = crate::Reg<clk_ctrl2::CLK_CTRL2_SPEC>;
#[doc = "Subsystem Clock selection. Bit definition for MPC, Interconnect, C2C and CoreHW subsystems *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
pub mod clk_ctrl2;
#[doc = "SS_RESET_EN (rw) register accessor: an alias for `Reg<SS_RESET_EN_SPEC>`"]
pub type SS_RESET_EN = crate::Reg<ss_reset_en::SS_RESET_EN_SPEC>;
#[doc = "Assertion of the Subsystem bit, deasserts the reset to the corresponding subsystem. Bit definition is mentioned in adjoining table"]
pub mod ss_reset_en;
#[doc = "SS_CLK_EN (rw) register accessor: an alias for `Reg<SS_CLK_EN_SPEC>`"]
pub type SS_CLK_EN = crate::Reg<ss_clk_en::SS_CLK_EN_SPEC>;
#[doc = "Subsystem clock enable register"]
pub mod ss_clk_en;
#[doc = "CLK_CTRL3 (rw) register accessor: an alias for `Reg<CLK_CTRL3_SPEC>`"]
pub type CLK_CTRL3 = crate::Reg<clk_ctrl3::CLK_CTRL3_SPEC>;
#[doc = "Subsystem Clock selection. Bit definition for Top peripheral subsystem. *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid"]
pub mod clk_ctrl3;
#[doc = "SLOW_PULSE_DIV (rw) register accessor: an alias for `Reg<SLOW_PULSE_DIV_SPEC>`"]
pub type SLOW_PULSE_DIV = crate::Reg<slow_pulse_div::SLOW_PULSE_DIV_SPEC>;
#[doc = ""]
pub mod slow_pulse_div;
#[doc = "PAD_CFG_2 (rw) register accessor: an alias for `Reg<PAD_CFG_2_SPEC>`"]
pub type PAD_CFG_2 = crate::Reg<pad_cfg_2::PAD_CFG_2_SPEC>;
#[doc = ""]
pub mod pad_cfg_2;
#[doc = "PAD_CFG_3 (rw) register accessor: an alias for `Reg<PAD_CFG_3_SPEC>`"]
pub type PAD_CFG_3 = crate::Reg<pad_cfg_3::PAD_CFG_3_SPEC>;
#[doc = ""]
pub mod pad_cfg_3;
#[doc = "PAD_CFG_5 (rw) register accessor: an alias for `Reg<PAD_CFG_5_SPEC>`"]
pub type PAD_CFG_5 = crate::Reg<pad_cfg_5::PAD_CFG_5_SPEC>;
#[doc = ""]
pub mod pad_cfg_5;
#[doc = "PAD_CFG_4 (rw) register accessor: an alias for `Reg<PAD_CFG_4_SPEC>`"]
pub type PAD_CFG_4 = crate::Reg<pad_cfg_4::PAD_CFG_4_SPEC>;
#[doc = ""]
pub mod pad_cfg_4;
