#[doc = r"Register block"]
#[repr(C)]
pub struct C2C_PLL {
    #[doc = "0x00 - "]
    pub c2c_pll_loop_ctrl: C2C_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub c2c_pll_div: C2C_PLL_DIV,
    #[doc = "0x08 - "]
    pub c2c_pll_debug_ctrl: C2C_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub c2c_pll_enable: C2C_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub c2c_pll_spare_ctrl: C2C_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub c2c_pll_tmux_sel: C2C_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub c2c_pll_status1: C2C_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub c2c_pll_status2: C2C_PLL_STATUS2,
}
#[doc = "C2C_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<C2C_PLL_LOOP_CTRL_SPEC>`"]
pub type C2C_PLL_LOOP_CTRL = crate::Reg<c2c_pll_loop_ctrl::C2C_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod c2c_pll_loop_ctrl;
#[doc = "C2C_PLL_DIV (rw) register accessor: an alias for `Reg<C2C_PLL_DIV_SPEC>`"]
pub type C2C_PLL_DIV = crate::Reg<c2c_pll_div::C2C_PLL_DIV_SPEC>;
#[doc = ""]
pub mod c2c_pll_div;
#[doc = "C2C_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<C2C_PLL_DEBUG_CTRL_SPEC>`"]
pub type C2C_PLL_DEBUG_CTRL = crate::Reg<c2c_pll_debug_ctrl::C2C_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod c2c_pll_debug_ctrl;
#[doc = "C2C_PLL_ENABLE (rw) register accessor: an alias for `Reg<C2C_PLL_ENABLE_SPEC>`"]
pub type C2C_PLL_ENABLE = crate::Reg<c2c_pll_enable::C2C_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod c2c_pll_enable;
#[doc = "C2C_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<C2C_PLL_SPARE_CTRL_SPEC>`"]
pub type C2C_PLL_SPARE_CTRL = crate::Reg<c2c_pll_spare_ctrl::C2C_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod c2c_pll_spare_ctrl;
#[doc = "C2C_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<C2C_PLL_TMUX_SEL_SPEC>`"]
pub type C2C_PLL_TMUX_SEL = crate::Reg<c2c_pll_tmux_sel::C2C_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod c2c_pll_tmux_sel;
#[doc = "C2C_PLL_STATUS1 (r) register accessor: an alias for `Reg<C2C_PLL_STATUS1_SPEC>`"]
pub type C2C_PLL_STATUS1 = crate::Reg<c2c_pll_status1::C2C_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod c2c_pll_status1;
#[doc = "C2C_PLL_STATUS2 (r) register accessor: an alias for `Reg<C2C_PLL_STATUS2_SPEC>`"]
pub type C2C_PLL_STATUS2 = crate::Reg<c2c_pll_status2::C2C_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod c2c_pll_status2;
