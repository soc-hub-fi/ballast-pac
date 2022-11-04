#[doc = "INFO register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "This register holds the number of clusters and the number of cores in the each cluster. It is a read-only register."]
pub mod info;
#[doc = "BOOT_ADR register accessor: an alias for `Reg<BOOT_ADR_SPEC>`"]
pub type BOOT_ADR = crate::Reg<boot_adr::BOOT_ADR_SPEC>;
#[doc = "This register holds the boot address."]
pub mod boot_adr;
#[doc = "FETCH_ENABLE register accessor: an alias for `Reg<FETCH_ENABLE_SPEC>`"]
pub type FETCH_ENABLE = crate::Reg<fetch_enable::FETCH_ENABLE_SPEC>;
#[doc = "This register contains the value of the fetch enable signal of the core."]
pub mod fetch_enable;
#[doc = "PAD_MUX_0 register accessor: an alias for `Reg<PAD_MUX_0_SPEC>`"]
pub type PAD_MUX_0 = crate::Reg<pad_mux_0::PAD_MUX_0_SPEC>;
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\])."]
pub mod pad_mux_0;
#[doc = "PAD_MUX_1 register accessor: an alias for `Reg<PAD_MUX_1_SPEC>`"]
pub type PAD_MUX_1 = crate::Reg<pad_mux_1::PAD_MUX_1_SPEC>;
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The second register (0x1A10_4014) can be used to sets the mux (2 bit select) from pin 16 (bits \\[1:0\\]) to 31 (bits \\[31:30\\])."]
pub mod pad_mux_1;
#[doc = "PAD_MUX_2 register accessor: an alias for `Reg<PAD_MUX_2_SPEC>`"]
pub type PAD_MUX_2 = crate::Reg<pad_mux_2::PAD_MUX_2_SPEC>;
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The third register (0x1A10_4018) can be used to sets the mux (2 bit select) from pin 32 (bits \\[1:0\\]) to 47 (bits \\[31:30\\])."]
pub mod pad_mux_2;
#[doc = "PAD_MUX_3 register accessor: an alias for `Reg<PAD_MUX_3_SPEC>`"]
pub type PAD_MUX_3 = crate::Reg<pad_mux_3::PAD_MUX_3_SPEC>;
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The forth register (0x1A10_401C) can be used to sets the mux (2 bit select) from pin 48 (bits \\[1:0\\]) to 63 (bits \\[31:30\\])."]
pub mod pad_mux_3;
#[doc = "PAD_CFG_0 register accessor: an alias for `Reg<PAD_CFG_0_SPEC>`"]
pub type PAD_CFG_0 = crate::Reg<pad_cfg_0::PAD_CFG_0_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_0;
#[doc = "PAD_CFG_1 register accessor: an alias for `Reg<PAD_CFG_1_SPEC>`"]
pub type PAD_CFG_1 = crate::Reg<pad_cfg_1::PAD_CFG_1_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_1;
#[doc = "PAD_CFG_2 register accessor: an alias for `Reg<PAD_CFG_2_SPEC>`"]
pub type PAD_CFG_2 = crate::Reg<pad_cfg_2::PAD_CFG_2_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_2;
#[doc = "PAD_CFG_3 register accessor: an alias for `Reg<PAD_CFG_3_SPEC>`"]
pub type PAD_CFG_3 = crate::Reg<pad_cfg_3::PAD_CFG_3_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_3;
#[doc = "PAD_CFG_4 register accessor: an alias for `Reg<PAD_CFG_4_SPEC>`"]
pub type PAD_CFG_4 = crate::Reg<pad_cfg_4::PAD_CFG_4_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_4;
#[doc = "PAD_CFG_5 register accessor: an alias for `Reg<PAD_CFG_5_SPEC>`"]
pub type PAD_CFG_5 = crate::Reg<pad_cfg_5::PAD_CFG_5_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_5;
#[doc = "PAD_CFG_6 register accessor: an alias for `Reg<PAD_CFG_6_SPEC>`"]
pub type PAD_CFG_6 = crate::Reg<pad_cfg_6::PAD_CFG_6_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_6;
#[doc = "PAD_CFG_7 register accessor: an alias for `Reg<PAD_CFG_7_SPEC>`"]
pub type PAD_CFG_7 = crate::Reg<pad_cfg_7::PAD_CFG_7_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_7;
#[doc = "PAD_CFG_8 register accessor: an alias for `Reg<PAD_CFG_8_SPEC>`"]
pub type PAD_CFG_8 = crate::Reg<pad_cfg_8::PAD_CFG_8_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_8;
#[doc = "PAD_CFG_9 register accessor: an alias for `Reg<PAD_CFG_9_SPEC>`"]
pub type PAD_CFG_9 = crate::Reg<pad_cfg_9::PAD_CFG_9_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_9;
#[doc = "PAD_CFG_10 register accessor: an alias for `Reg<PAD_CFG_10_SPEC>`"]
pub type PAD_CFG_10 = crate::Reg<pad_cfg_10::PAD_CFG_10_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_10;
#[doc = "PAD_CFG_11 register accessor: an alias for `Reg<PAD_CFG_11_SPEC>`"]
pub type PAD_CFG_11 = crate::Reg<pad_cfg_11::PAD_CFG_11_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_11;
#[doc = "PAD_CFG_12 register accessor: an alias for `Reg<PAD_CFG_12_SPEC>`"]
pub type PAD_CFG_12 = crate::Reg<pad_cfg_12::PAD_CFG_12_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_12;
#[doc = "PAD_CFG_13 register accessor: an alias for `Reg<PAD_CFG_13_SPEC>`"]
pub type PAD_CFG_13 = crate::Reg<pad_cfg_13::PAD_CFG_13_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_13;
#[doc = "PAD_CFG_14 register accessor: an alias for `Reg<PAD_CFG_14_SPEC>`"]
pub type PAD_CFG_14 = crate::Reg<pad_cfg_14::PAD_CFG_14_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_14;
#[doc = "FLL_CLOCK_SELECT register accessor: an alias for `Reg<FLL_CLOCK_SELECT_SPEC>`"]
pub type FLL_CLOCK_SELECT = crate::Reg<fll_clock_select::FLL_CLOCK_SELECT_SPEC>;
#[doc = "This register contains whether the system clock is coming from the FLL or the FLL is bypassed. It is a read-only register by the core but it can be written via JTAG."]
pub mod fll_clock_select;
#[doc = "CORE_STATUS_0 register accessor: an alias for `Reg<CORE_STATUS_0_SPEC>`"]
pub type CORE_STATUS_0 = crate::Reg<core_status_0::CORE_STATUS_0_SPEC>;
#[doc = "2 Core Status registers contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
pub mod core_status_0;
#[doc = "PAD_CFG_15 register accessor: an alias for `Reg<PAD_CFG_15_SPEC>`"]
pub type PAD_CFG_15 = crate::Reg<pad_cfg_15::PAD_CFG_15_SPEC>;
#[doc = "Pad configuration registers can be used for ASIC targets to configure pads, e.g. pull up, pull down values."]
pub mod pad_cfg_15;
#[doc = "CORE_STATUS_1 register accessor: an alias for `Reg<CORE_STATUS_1_SPEC>`"]
pub type CORE_STATUS_1 = crate::Reg<core_status_1::CORE_STATUS_1_SPEC>;
#[doc = "2 Core Status registers contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only."]
pub mod core_status_1;
#[doc = "JTAG_REG register accessor: an alias for `Reg<JTAG_REG_SPEC>`"]
pub type JTAG_REG = crate::Reg<jtag_reg::JTAG_REG_SPEC>;
#[doc = "This register contains the value of the input from the JTAG and can be used to write 8bit in the JTAG output register for system-to-JTAG communications."]
pub mod jtag_reg;
#[doc = "PERIPH_CLK_DIV register accessor: an alias for `Reg<PERIPH_CLK_DIV_SPEC>`"]
pub type PERIPH_CLK_DIV = crate::Reg<periph_clk_div::PERIPH_CLK_DIV_SPEC>;
#[doc = ""]
pub mod periph_clk_div;
