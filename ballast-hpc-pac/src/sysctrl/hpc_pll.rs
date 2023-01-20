#[doc = r"Register block"]
#[repr(C)]
pub struct HPC_PLL {
    #[doc = "0x00 - "]
    pub hpc_pll_loop_ctrl: HPC_PLL_LOOP_CTRL,
    #[doc = "0x04 - "]
    pub hpc_pll_div: HPC_PLL_DIV,
    #[doc = "0x08 - "]
    pub hpc_pll_debug_ctrl: HPC_PLL_DEBUG_CTRL,
    #[doc = "0x0c - "]
    pub hpc_pll_enable: HPC_PLL_ENABLE,
    #[doc = "0x10 - "]
    pub hpc_pll_spare_ctrl: HPC_PLL_SPARE_CTRL,
    #[doc = "0x14 - "]
    pub hpc_pll_tmux_sel: HPC_PLL_TMUX_SEL,
    #[doc = "0x18 - "]
    pub hpc_pll_status1: HPC_PLL_STATUS1,
    #[doc = "0x1c - "]
    pub hpc_pll_status2: HPC_PLL_STATUS2,
}
#[doc = "HPC_PLL_LOOP_CTRL (rw) register accessor: an alias for `Reg<HPC_PLL_LOOP_CTRL_SPEC>`"]
pub type HPC_PLL_LOOP_CTRL = crate::Reg<hpc_pll_loop_ctrl::HPC_PLL_LOOP_CTRL_SPEC>;
#[doc = ""]
pub mod hpc_pll_loop_ctrl;
#[doc = "HPC_PLL_DIV (rw) register accessor: an alias for `Reg<HPC_PLL_DIV_SPEC>`"]
pub type HPC_PLL_DIV = crate::Reg<hpc_pll_div::HPC_PLL_DIV_SPEC>;
#[doc = ""]
pub mod hpc_pll_div;
#[doc = "HPC_PLL_DEBUG_CTRL (rw) register accessor: an alias for `Reg<HPC_PLL_DEBUG_CTRL_SPEC>`"]
pub type HPC_PLL_DEBUG_CTRL = crate::Reg<hpc_pll_debug_ctrl::HPC_PLL_DEBUG_CTRL_SPEC>;
#[doc = ""]
pub mod hpc_pll_debug_ctrl;
#[doc = "HPC_PLL_ENABLE (rw) register accessor: an alias for `Reg<HPC_PLL_ENABLE_SPEC>`"]
pub type HPC_PLL_ENABLE = crate::Reg<hpc_pll_enable::HPC_PLL_ENABLE_SPEC>;
#[doc = ""]
pub mod hpc_pll_enable;
#[doc = "HPC_PLL_SPARE_CTRL (rw) register accessor: an alias for `Reg<HPC_PLL_SPARE_CTRL_SPEC>`"]
pub type HPC_PLL_SPARE_CTRL = crate::Reg<hpc_pll_spare_ctrl::HPC_PLL_SPARE_CTRL_SPEC>;
#[doc = ""]
pub mod hpc_pll_spare_ctrl;
#[doc = "HPC_PLL_TMUX_SEL (rw) register accessor: an alias for `Reg<HPC_PLL_TMUX_SEL_SPEC>`"]
pub type HPC_PLL_TMUX_SEL = crate::Reg<hpc_pll_tmux_sel::HPC_PLL_TMUX_SEL_SPEC>;
#[doc = ""]
pub mod hpc_pll_tmux_sel;
#[doc = "HPC_PLL_STATUS1 (r) register accessor: an alias for `Reg<HPC_PLL_STATUS1_SPEC>`"]
pub type HPC_PLL_STATUS1 = crate::Reg<hpc_pll_status1::HPC_PLL_STATUS1_SPEC>;
#[doc = ""]
pub mod hpc_pll_status1;
#[doc = "HPC_PLL_STATUS2 (r) register accessor: an alias for `Reg<HPC_PLL_STATUS2_SPEC>`"]
pub type HPC_PLL_STATUS2 = crate::Reg<hpc_pll_status2::HPC_PLL_STATUS2_SPEC>;
#[doc = ""]
pub mod hpc_pll_status2;
