#[doc = r"Register block"]
#[repr(C)]
pub struct INTER_PLL {
    #[doc = "0x00 - "]
    pub inter_pll_loop_ctrl: INTER_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub inter_pll_div: INTER_PLL_DIV,
    #[doc = "0x08 - "]
    pub inter_pll_debug_ctrl: INTER_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub inter_pll_enable: INTER_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub inter_pll_spare_ctrl: INTER_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub inter_pll_tmux_sel: INTER_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub inter_pll_status1: INTER_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub inter_pll_status2: INTER_PLL_STATUS2,
}
#[doc = "INTER_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<INTER_PLL_LOOP_CTRL_SPEC>`"]
pub type INTER_PLL_LOOP_CTRL = crate::Reg<inter_pll_loop_ctrl::INTER_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod inter_pll_loop_ctrl;
#[doc = "INTER_PLL_DIV (rw) register accessor: an alias for `Reg<INTER_PLL_DIV_SPEC>`"]
pub type INTER_PLL_DIV = crate::Reg<inter_pll_div::INTER_PLL_DIV_SPEC>;
#[doc = ""]
pub mod inter_pll_div;
#[doc = "INTER_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<INTER_PLL_DEBUG_CTRL_SPEC>`"]
pub type INTER_PLL_DEBUG_CTRL = crate::Reg<inter_pll_debug_ctrl::INTER_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod inter_pll_debug_ctrl;
#[doc = "INTER_PLL_ENABLE (rw) register accessor: an alias for `Reg<INTER_PLL_ENABLE_SPEC>`"]
pub type INTER_PLL_ENABLE = crate::Reg<inter_pll_enable::INTER_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod inter_pll_enable;
#[doc = "INTER_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<INTER_PLL_SPARE_CTRL_SPEC>`"]
pub type INTER_PLL_SPARE_CTRL = crate::Reg<inter_pll_spare_ctrl::INTER_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod inter_pll_spare_ctrl;
#[doc = "INTER_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<INTER_PLL_TMUX_SEL_SPEC>`"]
pub type INTER_PLL_TMUX_SEL = crate::Reg<inter_pll_tmux_sel::INTER_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod inter_pll_tmux_sel;
#[doc = "INTER_PLL_STATUS1 (r) register accessor: an alias for `Reg<INTER_PLL_STATUS1_SPEC>`"]
pub type INTER_PLL_STATUS1 = crate::Reg<inter_pll_status1::INTER_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod inter_pll_status1;
#[doc = "INTER_PLL_STATUS2 (r) register accessor: an alias for `Reg<INTER_PLL_STATUS2_SPEC>`"]
pub type INTER_PLL_STATUS2 = crate::Reg<inter_pll_status2::INTER_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod inter_pll_status2;
