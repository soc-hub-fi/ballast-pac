#[doc = r"Register block"]
#[repr(C)]
pub struct EMA {
    #[doc = "0x00 - "]
    pub ai_mem_timing_mode: AI_MEM_TIMING_MODE,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - "]
    pub c2c_mem_timing_mode: C2C_MEM_TIMING_MODE,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - "]
    pub dsp_mem_timing_mode: DSP_MEM_TIMING_MODE,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - "]
    pub eth_mem_timing_mode: ETH_MEM_TIMING_MODE,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - "]
    pub hpc_mem_timing_mode: HPC_MEM_TIMING_MODE,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - "]
    pub mpc_mem_timing_mode: MPC_MEM_TIMING_MODE,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - "]
    pub sysctrl_mem_timing_mode: SYSCTRL_MEM_TIMING_MODE,
}
#[doc = "AI_mem_timing_mode (rw) register accessor: an alias for `Reg<AI_MEM_TIMING_MODE_SPEC>`"]
pub type AI_MEM_TIMING_MODE = crate::Reg<ai_mem_timing_mode::AI_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod ai_mem_timing_mode;
#[doc = "C2C_mem_timing_mode (rw) register accessor: an alias for `Reg<C2C_MEM_TIMING_MODE_SPEC>`"]
pub type C2C_MEM_TIMING_MODE = crate::Reg<c2c_mem_timing_mode::C2C_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod c2c_mem_timing_mode;
#[doc = "dsp_mem_timing_mode (rw) register accessor: an alias for `Reg<DSP_MEM_TIMING_MODE_SPEC>`"]
pub type DSP_MEM_TIMING_MODE = crate::Reg<dsp_mem_timing_mode::DSP_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod dsp_mem_timing_mode;
#[doc = "eth_mem_timing_mode (rw) register accessor: an alias for `Reg<ETH_MEM_TIMING_MODE_SPEC>`"]
pub type ETH_MEM_TIMING_MODE = crate::Reg<eth_mem_timing_mode::ETH_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod eth_mem_timing_mode;
#[doc = "hpc_mem_timing_mode (rw) register accessor: an alias for `Reg<HPC_MEM_TIMING_MODE_SPEC>`"]
pub type HPC_MEM_TIMING_MODE = crate::Reg<hpc_mem_timing_mode::HPC_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod hpc_mem_timing_mode;
#[doc = "mpc_mem_timing_mode (w) register accessor: an alias for `Reg<MPC_MEM_TIMING_MODE_SPEC>`"]
pub type MPC_MEM_TIMING_MODE = crate::Reg<mpc_mem_timing_mode::MPC_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod mpc_mem_timing_mode;
#[doc = "sysctrl_mem_timing_mode (w) register accessor: an alias for `Reg<SYSCTRL_MEM_TIMING_MODE_SPEC>`"]
pub type SYSCTRL_MEM_TIMING_MODE =
    crate::Reg<sysctrl_mem_timing_mode::SYSCTRL_MEM_TIMING_MODE_SPEC>;
#[doc = ""]
pub mod sysctrl_mem_timing_mode;
