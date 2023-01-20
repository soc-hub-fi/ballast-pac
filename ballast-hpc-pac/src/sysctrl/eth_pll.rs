#[doc = r"Register block"]
#[repr(C)]
pub struct ETH_PLL {
    #[doc = "0x00 - "]
    pub eth_pll_loop_ctrl: ETH_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub eth_pll_div: ETH_PLL_DIV,
    #[doc = "0x08 - "]
    pub eth_pll_debug_ctrl: ETH_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub eth_pll_enable: ETH_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub eth_pll_spare_ctrl: ETH_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub eth_pll_tmux_sel: ETH_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub eth_pll_status1: ETH_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub eth_pll_status2: ETH_PLL_STATUS2,
}
#[doc = "ETH_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<ETH_PLL_LOOP_CTRL_SPEC>`"]
pub type ETH_PLL_LOOP_CTRL = crate::Reg<eth_pll_loop_ctrl::ETH_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod eth_pll_loop_ctrl;
#[doc = "ETH_PLL_DIV (rw) register accessor: an alias for `Reg<ETH_PLL_DIV_SPEC>`"]
pub type ETH_PLL_DIV = crate::Reg<eth_pll_div::ETH_PLL_DIV_SPEC>;
#[doc = ""]
pub mod eth_pll_div;
#[doc = "ETH_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<ETH_PLL_DEBUG_CTRL_SPEC>`"]
pub type ETH_PLL_DEBUG_CTRL = crate::Reg<eth_pll_debug_ctrl::ETH_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod eth_pll_debug_ctrl;
#[doc = "ETH_PLL_ENABLE (rw) register accessor: an alias for `Reg<ETH_PLL_ENABLE_SPEC>`"]
pub type ETH_PLL_ENABLE = crate::Reg<eth_pll_enable::ETH_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod eth_pll_enable;
#[doc = "ETH_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<ETH_PLL_SPARE_CTRL_SPEC>`"]
pub type ETH_PLL_SPARE_CTRL = crate::Reg<eth_pll_spare_ctrl::ETH_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod eth_pll_spare_ctrl;
#[doc = "ETH_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<ETH_PLL_TMUX_SEL_SPEC>`"]
pub type ETH_PLL_TMUX_SEL = crate::Reg<eth_pll_tmux_sel::ETH_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod eth_pll_tmux_sel;
#[doc = "ETH_PLL_STATUS1 (r) register accessor: an alias for `Reg<ETH_PLL_STATUS1_SPEC>`"]
pub type ETH_PLL_STATUS1 = crate::Reg<eth_pll_status1::ETH_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod eth_pll_status1;
#[doc = "ETH_PLL_STATUS2 (r) register accessor: an alias for `Reg<ETH_PLL_STATUS2_SPEC>`"]
pub type ETH_PLL_STATUS2 = crate::Reg<eth_pll_status2::ETH_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod eth_pll_status2;
