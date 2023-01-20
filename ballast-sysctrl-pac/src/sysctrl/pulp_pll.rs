#[doc = r"Register block"]
#[repr(C)]
pub struct PULP_PLL {
    #[doc = "0x00 - "]
    pub pulp_pll_loop_ctrl: PULP_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub pulp_pll_div: PULP_PLL_DIV,
    #[doc = "0x08 - "]
    pub pulp_pll_debug_ctrl: PULP_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub pulp_pll_enable: PULP_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub pulp_pll_spare_ctrl: PULP_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub pulp_pll_tmux_sel: PULP_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub pulp_pll_status1: PULP_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub pulp_pll_status2: PULP_PLL_STATUS2,
}
#[doc = "PULP_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<PULP_PLL_LOOP_CTRL_SPEC>`"]
pub type PULP_PLL_LOOP_CTRL = crate::Reg<pulp_pll_loop_ctrl::PULP_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod pulp_pll_loop_ctrl;
#[doc = "PULP_PLL_DIV (rw) register accessor: an alias for `Reg<PULP_PLL_DIV_SPEC>`"]
pub type PULP_PLL_DIV = crate::Reg<pulp_pll_div::PULP_PLL_DIV_SPEC>;
#[doc = ""]
pub mod pulp_pll_div;
#[doc = "PULP_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<PULP_PLL_DEBUG_CTRL_SPEC>`"]
pub type PULP_PLL_DEBUG_CTRL = crate::Reg<pulp_pll_debug_ctrl::PULP_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod pulp_pll_debug_ctrl;
#[doc = "PULP_PLL_ENABLE (rw) register accessor: an alias for `Reg<PULP_PLL_ENABLE_SPEC>`"]
pub type PULP_PLL_ENABLE = crate::Reg<pulp_pll_enable::PULP_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod pulp_pll_enable;
#[doc = "PULP_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<PULP_PLL_SPARE_CTRL_SPEC>`"]
pub type PULP_PLL_SPARE_CTRL = crate::Reg<pulp_pll_spare_ctrl::PULP_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod pulp_pll_spare_ctrl;
#[doc = "PULP_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<PULP_PLL_TMUX_SEL_SPEC>`"]
pub type PULP_PLL_TMUX_SEL = crate::Reg<pulp_pll_tmux_sel::PULP_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod pulp_pll_tmux_sel;
#[doc = "PULP_PLL_STATUS1 (r) register accessor: an alias for `Reg<PULP_PLL_STATUS1_SPEC>`"]
pub type PULP_PLL_STATUS1 = crate::Reg<pulp_pll_status1::PULP_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod pulp_pll_status1;
#[doc = "PULP_PLL_STATUS2 (r) register accessor: an alias for `Reg<PULP_PLL_STATUS2_SPEC>`"]
pub type PULP_PLL_STATUS2 = crate::Reg<pulp_pll_status2::PULP_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod pulp_pll_status2;
