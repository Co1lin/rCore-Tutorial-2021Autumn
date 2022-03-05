/*!
 * some useful functions
 */
#![allow(dead_code)]
use riscv::register::{ sstatus, sstatus::SPP, sscratch };

pub fn get_stack_range(sp: usize, stack_size: usize) -> (usize, usize) {
    let top = (sp + stack_size - 1) & (!(stack_size - 1));
    (top - stack_size, top)
}

pub fn get_used_stack_range(sp: usize, stack_size: usize) -> (usize, usize) {
    let top = (sp + stack_size - 1) & (!(stack_size - 1));
    (sp, top)
}

pub unsafe fn r_sp() -> usize {
    let mut sp: usize;
    core::arch::asm!("mv {}, sp", out(reg) sp);
    sp
}

pub fn r_sstatus_spp() -> SPP {
    let sstatus = sstatus::read();
    sstatus.spp()
}

pub fn r_sscratch() -> usize {
    sscratch::read()
}

pub fn range_contains<T: PartialOrd>(large: (T, T), small: (T, T)) -> bool {
    large.0 <= small.0 && small.1 <= large.1
}