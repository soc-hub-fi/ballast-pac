#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "GPIO direction configuration bitfield: - bit\\[i\\]=1’b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1’b1: Output mode for GPIO\\[i\\]"]
pub mod dir;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1’b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1’b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if it’s direction is configured in input mode."]
pub mod en;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
pub mod in_;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
pub mod out;
#[doc = "PAD_CFG_0_3 register accessor: an alias for `Reg<PAD_CFG_0_3_SPEC>`"]
pub type PAD_CFG_0_3 = crate::Reg<pad_cfg_0_3::PAD_CFG_0_3_SPEC>;
#[doc = ""]
pub mod pad_cfg_0_3;
#[doc = "PAD_CFG_4_7 register accessor: an alias for `Reg<PAD_CFG_4_7_SPEC>`"]
pub type PAD_CFG_4_7 = crate::Reg<pad_cfg_4_7::PAD_CFG_4_7_SPEC>;
#[doc = ""]
pub mod pad_cfg_4_7;
#[doc = "PAD_CFG_8_11 register accessor: an alias for `Reg<PAD_CFG_8_11_SPEC>`"]
pub type PAD_CFG_8_11 = crate::Reg<pad_cfg_8_11::PAD_CFG_8_11_SPEC>;
#[doc = ""]
pub mod pad_cfg_8_11;
#[doc = "PAD_CFG_12_15 register accessor: an alias for `Reg<PAD_CFG_12_15_SPEC>`"]
pub type PAD_CFG_12_15 = crate::Reg<pad_cfg_12_15::PAD_CFG_12_15_SPEC>;
#[doc = ""]
pub mod pad_cfg_12_15;
