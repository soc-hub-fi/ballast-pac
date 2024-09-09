extern crate riscv_slic;

riscv_slic::codegen!(
    pac = self::clint,
    swi = [Soft1, Soft2, Soft3], //are these enough?
    backend = [hart_id = HART0]
);
pub use slic::Interrupt as SoftwareInterrupts; // Re-export of automatically generated enum of interrupts in previous macro
