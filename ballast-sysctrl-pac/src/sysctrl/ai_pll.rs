#[doc = r"Register block"]
#[repr(C)]
pub struct AI_PLL {
    #[doc = "0x00 - "]
    pub ai_pll_loop_ctrl: AI_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub ai_pll_div: AI_PLL_DIV,
    #[doc = "0x08 - "]
    pub ai_pll_debug_ctrl: AI_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub ai_pll_enable: AI_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub ai_pll_spare_ctrl: AI_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub ai_pll_tmux_sel: AI_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub ai_pll_status1: AI_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub ai_pll_status2: AI_PLL_STATUS2,
}
#[doc = "AI_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<AI_PLL_LOOP_CTRL_SPEC>`"]
pub type AI_PLL_LOOP_CTRL = crate::Reg<ai_pll_loop_ctrl::AI_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod ai_pll_loop_ctrl;
#[doc = "AI_PLL_DIV (rw) register accessor: an alias for `Reg<AI_PLL_DIV_SPEC>`"]
pub type AI_PLL_DIV = crate::Reg<ai_pll_div::AI_PLL_DIV_SPEC>;
#[doc = ""]
pub mod ai_pll_div;
#[doc = "AI_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<AI_PLL_DEBUG_CTRL_SPEC>`"]
pub type AI_PLL_DEBUG_CTRL = crate::Reg<ai_pll_debug_ctrl::AI_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod ai_pll_debug_ctrl;
#[doc = "AI_PLL_ENABLE (rw) register accessor: an alias for `Reg<AI_PLL_ENABLE_SPEC>`"]
pub type AI_PLL_ENABLE = crate::Reg<ai_pll_enable::AI_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod ai_pll_enable;
#[doc = "AI_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<AI_PLL_SPARE_CTRL_SPEC>`"]
pub type AI_PLL_SPARE_CTRL = crate::Reg<ai_pll_spare_ctrl::AI_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod ai_pll_spare_ctrl;
#[doc = "AI_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<AI_PLL_TMUX_SEL_SPEC>`"]
pub type AI_PLL_TMUX_SEL = crate::Reg<ai_pll_tmux_sel::AI_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod ai_pll_tmux_sel;
#[doc = "AI_PLL_STATUS1 (r) register accessor: an alias for `Reg<AI_PLL_STATUS1_SPEC>`"]
pub type AI_PLL_STATUS1 = crate::Reg<ai_pll_status1::AI_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod ai_pll_status1;
#[doc = "AI_PLL_STATUS2 (r) register accessor: an alias for `Reg<AI_PLL_STATUS2_SPEC>`"]
pub type AI_PLL_STATUS2 = crate::Reg<ai_pll_status2::AI_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod ai_pll_status2;
