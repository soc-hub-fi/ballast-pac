#[doc = r"Register block"]
#[repr(C)]
pub struct GPIO {
    #[doc = "0x00 - GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
    pub dir: DIR,
    #[doc = "0x04 - GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
    pub en: EN,
    #[doc = "0x08 - GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
    pub in_: IN,
    #[doc = "0x0c - GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
    pub out: OUT,
    _reserved4: [u8; 0x18],
    #[doc = "0x28 - "]
    pub pad_cfg_0_3: PAD_CFG_0_3,
    #[doc = "0x2c - "]
    pub pad_cfg_4_7: PAD_CFG_4_7,
    #[doc = "0x30 - "]
    pub pad_cfg_8_11: PAD_CFG_8_11,
    #[doc = "0x34 - "]
    pub pad_cfg_12_15: PAD_CFG_12_15,
}
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "GPIO direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]"]
pub mod dir;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "GPIO clock enable config register. Bit 31 - 0 GPIOEN (R/W) GPIO\\[31:0\\]
clock enable configuration bitfield: - bit\\[i\\]=1b0: disable clock for GPIO\\[i\\]
- bit\\[i\\]=1b1: enable clock for GPIO\\[i\\]
GPIOs are gathered by groups of 4. The clock gating of one group is done only if all 4 GPIOs are disabled. Clock must be enabled for a GPIO if its direction is configured in input mode."]
pub mod en;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO Data IN register. Bit 31 - 0 DATA_IN (R) GPIO\\[31:0\\]
input data read bitfield. DATA_IN\\[i\\]
corresponds to input data of GPIO\\[i\\]."]
pub mod in_;
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO Data out register. Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]."]
pub mod out;
#[doc = "PAD_CFG_0_3 (rw) register accessor: an alias for `Reg<PAD_CFG_0_3_SPEC>`"]
pub type PAD_CFG_0_3 = crate::Reg<pad_cfg_0_3::PAD_CFG_0_3_SPEC>;
#[doc = ""]
pub mod pad_cfg_0_3;
#[doc = "PAD_CFG_4_7 (rw) register accessor: an alias for `Reg<PAD_CFG_4_7_SPEC>`"]
pub type PAD_CFG_4_7 = crate::Reg<pad_cfg_4_7::PAD_CFG_4_7_SPEC>;
#[doc = ""]
pub mod pad_cfg_4_7;
#[doc = "PAD_CFG_8_11 (rw) register accessor: an alias for `Reg<PAD_CFG_8_11_SPEC>`"]
pub type PAD_CFG_8_11 = crate::Reg<pad_cfg_8_11::PAD_CFG_8_11_SPEC>;
#[doc = ""]
pub mod pad_cfg_8_11;
#[doc = "PAD_CFG_12_15 (rw) register accessor: an alias for `Reg<PAD_CFG_12_15_SPEC>`"]
pub type PAD_CFG_12_15 = crate::Reg<pad_cfg_12_15::PAD_CFG_12_15_SPEC>;
#[doc = ""]
pub mod pad_cfg_12_15;
