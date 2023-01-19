#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - "]
    pub timer0_timer: TIMER0_TIMER,
    #[doc = "0x04 - "]
    pub timer0_ctrl: TIMER0_CTRL,
    #[doc = "0x08 - "]
    pub timer0_cmp: TIMER0_CMP,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - "]
    pub timer1_timer: TIMER1_TIMER,
    #[doc = "0x14 - "]
    pub timer1_ctrl: TIMER1_CTRL,
    #[doc = "0x18 - "]
    pub timer1_cmp: TIMER1_CMP,
}
#[doc = "timer0_timer (rw) register accessor: an alias for `Reg<TIMER0_TIMER_SPEC>`"]
pub type TIMER0_TIMER = crate::Reg<timer0_timer::TIMER0_TIMER_SPEC>;
#[doc = ""]
pub mod timer0_timer;
#[doc = "timer1_timer (rw) register accessor: an alias for `Reg<TIMER1_TIMER_SPEC>`"]
pub type TIMER1_TIMER = crate::Reg<timer1_timer::TIMER1_TIMER_SPEC>;
#[doc = ""]
pub mod timer1_timer;
#[doc = "timer0_ctrl (rw) register accessor: an alias for `Reg<TIMER0_CTRL_SPEC>`"]
pub type TIMER0_CTRL = crate::Reg<timer0_ctrl::TIMER0_CTRL_SPEC>;
#[doc = ""]
pub mod timer0_ctrl;
#[doc = "timer0_cmp (rw) register accessor: an alias for `Reg<TIMER0_CMP_SPEC>`"]
pub type TIMER0_CMP = crate::Reg<timer0_cmp::TIMER0_CMP_SPEC>;
#[doc = ""]
pub mod timer0_cmp;
#[doc = "timer1_ctrl (rw) register accessor: an alias for `Reg<TIMER1_CTRL_SPEC>`"]
pub type TIMER1_CTRL = crate::Reg<timer1_ctrl::TIMER1_CTRL_SPEC>;
#[doc = ""]
pub mod timer1_ctrl;
#[doc = "timer1_cmp (rw) register accessor: an alias for `Reg<TIMER1_CMP_SPEC>`"]
pub type TIMER1_CMP = crate::Reg<timer1_cmp::TIMER1_CMP_SPEC>;
#[doc = ""]
pub mod timer1_cmp;
