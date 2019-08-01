//! Assembly instructions

/// `maskirq` instruction wrapper
///
/// Original documentation from [https://github.com/cliffordwolf/picorv32/blob/master/README.md#maskirq]:
///
/// The "IRQ Mask" register contains a bitmask of masked (disabled) interrupts.
/// This instruction writes a new value to the irq mask register and reads the old
/// value.
///
///     0000011 ----- XXXXX --- XXXXX 0001011
///     f7      rs2   rs    f3  rd    opcode
///
/// Example:
///
///     maskirq x1, x2
///
/// The processor starts with all interrupts disabled.
///
/// An illegal instruction or bus error while the illegal instruction or bus error
/// interrupt is disabled will cause the processor to halt.
#[inline]
#[allow(unused_variables)]
pub unsafe fn maskirq(mask: u32) -> u32 {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => {
            let ret: u32;
            asm!(
                            ".word (((0b0000011) << 25) | ((0) << 20) | ((11) << 15) | ((0) << 12) | ((10) << 7) | ((0b0001011) << 0))"
            //                ".insn r OPC_OP 0b0001011, 0b000, 0b0000011, $0, $1, 0b00000"
                            : "={x10}"(ret)
                            : "{x11}"(mask)
                            :
                            : "volatile"
                        );
            ret
        }

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __maskirq(mask: u32) -> u32;
            }

            __maskirq(mask)
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `waitirq` instruction wrapper
///
/// Original documentation from [https://github.com/cliffordwolf/picorv32/blob/master/README.md#waitirq]:
///
/// Pause execution until an interrupt becomes pending. The bitmask of pending IRQs
/// is written to `rd`.
///
///     0000100 ----- 00000 --- XXXXX 0001011
///     f7      rs2   rs    f3  rd    opcode
///
/// Example:
///
///     waitirq x1
#[inline]
#[allow(unused_variables)]
pub unsafe fn waitirq() -> u32 {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => {
            let ret: u32;
            asm!(
                ".word (((0b0000100) << 25) | ((0) << 20) | ((0) << 15) | ((0) << 12) | ((10) << 7) | ((0b0001011) << 0))"
                : "={x10}"(ret)
                :
                :
                : "volatile"
            );
            ret
        }

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __waitirq() -> u32;
            }

            __waitirq()
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `timer` instruction wrapper
///
/// Original documentation from [https://github.com/cliffordwolf/picorv32/blob/master/README.md#timer]:
///
/// Reset the timer counter to a new value. The counter counts down clock cycles and
/// triggers the timer interrupt when transitioning from 1 to 0. Setting the
/// counter to zero disables the timer. The old value of the counter is written to
/// `rd`.
///
///     0000101 ----- XXXXX --- XXXXX 0001011
///     f7      rs2   rs    f3  rd    opcode
///
/// Example:
///
///     timer x1, x2
#[inline]
#[allow(unused_variables)]
pub unsafe fn timer(cycles_to_wait: u32) -> u32 {
    match () {
        #[cfg(all(riscv, feature = "inline-asm"))]
        () => {
            let ret: u32;
            asm!(
                            ".word (((0b0000101) << 25) | ((0) << 20) | ((11) << 15) | ((0) << 12) | ((10) << 7) | ((0b0001011) << 0))"
            //                ".insn r OPC_OP 0b0001011, 0b000, 0b0000101, $0, $1, 0b00000"
                            : "={x10}"(ret)
                            : "{x11}"(cycles_to_wait)
                            :
                            : "volatile"
                        );
            ret
        }

        #[cfg(all(riscv, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __timer(cycles_to_wait: u32) -> u32;
            }

            __timer(cycles_to_wait)
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}
