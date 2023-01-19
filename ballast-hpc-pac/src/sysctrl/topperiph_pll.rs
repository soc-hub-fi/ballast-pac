#[doc = r"Register block"]
#[repr(C)]
pub struct TOPPERIPH_PLL {
    #[doc = "0x00 - "]
    pub topperiph_pll_loop_ctrl: TOPPERIPH_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub topperiph_pll_div: TOPPERIPH_PLL_DIV,
    #[doc = "0x08 - "]
    pub topperiph_pll_debug_ctrl: TOPPERIPH_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub topperiph_pll_enable: TOPPERIPH_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub topperiph_pll_spare_ctrl: TOPPERIPH_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub topperiph_pll_tmux_sel: TOPPERIPH_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub topperiph_pll_status1: TOPPERIPH_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub topperiph_pll_status2: TOPPERIPH_PLL_STATUS2,
}
#[doc = "TOPPERIPH_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<TOPPERIPH_PLL_LOOP_CTRL_SPEC>`"]
pub type TOPPERIPH_PLL_LOOP_CTRL =
    crate::Reg<topperiph_pll_loop_ctrl::TOPPERIPH_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod topperiph_pll_loop_ctrl;
#[doc = "TOPPERIPH_PLL_DIV (rw) register accessor: an alias for `Reg<TOPPERIPH_PLL_DIV_SPEC>`"]
pub type TOPPERIPH_PLL_DIV = crate::Reg<topperiph_pll_div::TOPPERIPH_PLL_DIV_SPEC>;
#[doc = ""]
pub mod topperiph_pll_div;
#[doc = "TOPPERIPH_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<TOPPERIPH_PLL_DEBUG_CTRL_SPEC>`"]
pub type TOPPERIPH_PLL_DEBUG_CTRL =
    crate::Reg<topperiph_pll_debug_ctrl::TOPPERIPH_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod topperiph_pll_debug_ctrl;
#[doc = "TOPPERIPH_PLL_ENABLE (rw) register accessor: an alias for `Reg<TOPPERIPH_PLL_ENABLE_SPEC>`"]
pub type TOPPERIPH_PLL_ENABLE = crate::Reg<topperiph_pll_enable::TOPPERIPH_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod topperiph_pll_enable;
#[doc = "TOPPERIPH_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<TOPPERIPH_PLL_SPARE_CTRL_SPEC>`"]
pub type TOPPERIPH_PLL_SPARE_CTRL =
    crate::Reg<topperiph_pll_spare_ctrl::TOPPERIPH_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod topperiph_pll_spare_ctrl;
#[doc = "TOPPERIPH_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<TOPPERIPH_PLL_TMUX_SEL_SPEC>`"]
pub type TOPPERIPH_PLL_TMUX_SEL = crate::Reg<topperiph_pll_tmux_sel::TOPPERIPH_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod topperiph_pll_tmux_sel;
#[doc = "TOPPERIPH_PLL_STATUS1 (r) register accessor: an alias for `Reg<TOPPERIPH_PLL_STATUS1_SPEC>`"]
pub type TOPPERIPH_PLL_STATUS1 = crate::Reg<topperiph_pll_status1::TOPPERIPH_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod topperiph_pll_status1;
#[doc = "TOPPERIPH_PLL_STATUS2 (r) register accessor: an alias for `Reg<TOPPERIPH_PLL_STATUS2_SPEC>`"]
pub type TOPPERIPH_PLL_STATUS2 = crate::Reg<topperiph_pll_status2::TOPPERIPH_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod topperiph_pll_status2;
