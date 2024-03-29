#![doc = "Peripheral access API for BALLAST microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
extern "C" {}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 0] = [];
#[doc = "MPC"]
pub struct MPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPC {}
impl MPC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const mpc::RegisterBlock = 0x0001_2a00_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MPC {
    type Target = mpc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MPC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPC").finish()
    }
}
#[doc = "MPC"]
pub mod mpc;
#[doc = "ethernet"]
pub struct ETHERNET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET {}
impl ETHERNET {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ethernet::RegisterBlock = 0x0001_0e00_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ethernet::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ETHERNET {
    type Target = ethernet::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ETHERNET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETHERNET").finish()
    }
}
#[doc = "ethernet"]
pub mod ethernet;
#[doc = "Stores different status information about the core"]
pub struct DSP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSP {}
impl DSP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dsp::RegisterBlock = 0x0001_0d00_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DSP {
    type Target = dsp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSP").finish()
    }
}
#[doc = "Stores different status information about the core"]
pub mod dsp;
#[doc = "AI"]
pub struct AI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AI {}
impl AI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ai::RegisterBlock = 0x0001_0f00_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ai::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AI {
    type Target = ai::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AI").finish()
    }
}
#[doc = "AI"]
pub mod ai;
#[doc = "TLP"]
pub struct TLP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TLP {}
impl TLP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tlp::RegisterBlock = 0x0001_1000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tlp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TLP {
    type Target = tlp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TLP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TLP").finish()
    }
}
#[doc = "TLP"]
pub mod tlp;
#[doc = "HPC"]
pub struct HPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HPC {}
impl HPC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hpc::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hpc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HPC {
    type Target = hpc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HPC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPC").finish()
    }
}
#[doc = "HPC"]
pub mod hpc;
#[doc = "SYSCTRL"]
pub struct SYSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTRL {}
impl SYSCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sysctrl::RegisterBlock = 0x0001_1a00_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSCTRL {
    type Target = sysctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCTRL").finish()
    }
}
#[doc = "SYSCTRL"]
pub mod sysctrl;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MPC"]
    pub MPC: MPC,
    #[doc = "ETHERNET"]
    pub ETHERNET: ETHERNET,
    #[doc = "DSP"]
    pub DSP: DSP,
    #[doc = "AI"]
    pub AI: AI,
    #[doc = "TLP"]
    pub TLP: TLP,
    #[doc = "HPC"]
    pub HPC: HPC,
    #[doc = "SYSCTRL"]
    pub SYSCTRL: SYSCTRL,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MPC: MPC {
                _marker: PhantomData,
            },
            ETHERNET: ETHERNET {
                _marker: PhantomData,
            },
            DSP: DSP {
                _marker: PhantomData,
            },
            AI: AI {
                _marker: PhantomData,
            },
            TLP: TLP {
                _marker: PhantomData,
            },
            HPC: HPC {
                _marker: PhantomData,
            },
            SYSCTRL: SYSCTRL {
                _marker: PhantomData,
            },
        }
    }
}
