//! Interrupts

pub use bare_metal::{CriticalSection, Mutex, Nr};
use crate::asm::*;

/// Disables all interrupts
#[inline]
pub unsafe fn disable() {
    maskirq(0xffff_ffff);
}

/// Enables all the interrupts
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() {
    maskirq(0);
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
{
    // disable interrupts
    let old_mask = unsafe { maskirq(0xffff_ffff) };

    let r = f(unsafe { &CriticalSection::new() });

    unsafe { maskirq(old_mask); }

    r
}
