//! Assembly instructions

// The picorv32 implements custom interrupt instructions. We use the `.insn`
// directive to create the numeric representation of these instructions. For
// the `.insn` directive to work we have to specify the instruction format.
// The picorv32 uses the R format for the interrupt instructions.
//
// The syntax for an `.insn` directive with R type format is:
//
// ```
// .insn r opcode, func3, func7, rd, rs1, rs2
// ```
//
// From the [picorv32 manual](https://github.com/YosysHQ/picorv32):
//
//     opcode = 0b0001011 [CUSTOM_0]  (Opcode names are not supported)
//     func3  = ---                   (Don't care)
//     func7  = Specifies the instruction
//     rd     = Compiler allocated destination register
//     rs1    = Compiler allocated source register
//     rs2    = -----                 (Don't care so set to zero [x0])
//
// The compiler will pad and offset these values correctly for an R type
// instruction.
//
// This method frees the compiler to allocate registers and specify them in
// the assembly.
//
// Each instruction wrapper will set func7, rd and rs1 correctly for their
// instruction.

use core::arch::asm;

/// `getq` instruction wrapper (`getq __, q2`)
///
/// This function returns the value from the `q2` q-register.
///
/// Note: this function is only available when q-registers are enabled.
#[inline]
#[allow(unused_variables)]
#[cfg(feature = "interrupts-qregs")]
pub unsafe fn getq2() -> u32 {
    match () {
        #[cfg(riscv)]
        () => {
            let ret: u32;

            // The picorv32 getq2 specific values:
            //
            //     func7 = 0b0000000
            //     rd    = ret register
            //     rs1   = q2           (x2 used in place)
            //
            // NOTE: The `.insn` requires a register name for `rs1` but the
            // compiler is not aware of the `q` registers as they are picorv32
            // specific. To work around this we use the `x` register
            // equivalents. In this case we want to read from `q2` which is
            // offset 2, therefore we use `x2` to achieve this.
            asm!(
                ".insn r 0b0001011, 0, 0b0000000, {0}, x2, zero",
                out(reg) ret,
                );

            ret
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `getq` instruction wrapper (`getq __, q3`)
///
/// This function returns the value from the `q3` q-register.
///
/// Note: this function is only available when q-registers are enabled.
#[inline]
#[allow(unused_variables)]
#[cfg(feature = "interrupts-qregs")]
pub unsafe fn getq3() -> u32 {
    match () {
        #[cfg(riscv)]
        () => {
            let ret: u32;

            // The picorv32 getq3 specific values:
            //
            //     func7 = 0b0000000
            //     rd    = ret register
            //     rs1   = q3           (x3 used in place)
            //
            // NOTE: The `.insn` requires a register name for `rs1` but the
            // compiler is not aware of the `q` registers as they are picorv32
            // specific. To work around this we use the `x` register
            // equivalents. In this case we want to read from `q3` which is
            // offset 3, therefore we use `x3` to achieve this.
            asm!(
                ".insn r 0b0001011, 0, 0b0000000, {0}, x3, zero",
                out(reg) ret,
                );

            ret
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `setq` instruction wrapper (`setq __, q2`)
///
/// This function writes val to the `q2` q-register.
///
/// Note: this function is only available when q-registers are enabled.
#[inline]
#[allow(unused_variables)]
#[cfg(feature = "interrupts-qregs")]
pub unsafe fn setq2(val: u32) -> () {
    match () {
        #[cfg(riscv)]
        () => {
            // The picorv32 setq2 specific values:
            //
            //     func7 = 0b0000001
            //     rd    = q2           (x2 used in place)
            //     rs1   = val register
            //
            // NOTE: The `.insn` requires a register name for `rd` but the
            // compiler is not aware of the `q` registers as they are picorv32
            // specific. To work around this we use the `x` register
            // equivalents. In this case we want to write to `q2` which is
            // offset 2, therefore we use `x2` to achieve this.
            asm!(
                ".insn r 0b0001011, 0, 0b0000001, x2, {0}, zero",
                in(reg) val,
                );
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `setq` instruction wrapper (`setq __, q3`)
///
/// This function writes val to the `q3` q-register.
///
/// Note: this function is only available when q-registers are enabled.
#[inline]
#[allow(unused_variables)]
#[cfg(feature = "interrupts-qregs")]
pub unsafe fn setq3(val: u32) -> () {
    match () {
        #[cfg(riscv)]
        () => {
            // The picorv32 setq3 specific values:
            //
            //     func7 = 0b0000001
            //     rd    = q3           (x3 used in place)
            //     rs1   = val register
            //
            // NOTE: The `.insn` requires a register name for `rd` but the
            // compiler is not aware of the `q` registers as they are picorv32
            // specific. To work around this we use the `x` register
            // equivalents. In this case we want to write to `q3` which is
            // offset 3, therefore we use `x3` to achieve this.
            asm!(
                ".insn r 0b0001011, 0, 0b0000001, x3, {0}, zero",
                in(reg) val,
                );
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `retirq` instruction wrapper
///
/// Return from interrupt. This function resets the program counter to the
/// last value before the interrupt and re-enables interrupts.
#[inline]
#[allow(unused_variables)]
pub unsafe fn retirq() -> () {
    match () {
        #[cfg(riscv)]
        () => {
            // The picorv32 retirq specific values:
            //
            //     func7 = 0b0000010
            //     rd    = -----        (Don't care so set to zero [x0])
            //     rs1   = -----        (Don't care so set to zero [x0])
            asm!(
                ".insn r 0b0001011, 0, 0b0000010, zero, zero, zero",
                );
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `maskirq` instruction wrapper
///
/// This function writes a new value to the irq mask register and returns the
/// old value. The "IRQ Mask" register contains a bitmask of masked (disabled)
/// interrupts.
///
/// The mask argument is 32 bits. A `0b1` at any offset in `mask` disables
/// that interrupt.
#[inline]
#[allow(unused_variables)]
pub unsafe fn maskirq(mask: u32) -> u32 {
    match () {
        #[cfg(riscv)]
        () => {
            let ret: u32;

            // The picorv32 maskirq specific values:
            //
            //     func7 = 0b0000011
            //     rd    = ret register
            //     rs1   = mask register
            asm!(
                ".insn r 0b0001011, 0, 0b0000011, {0}, {1}, zero",
                out(reg) ret,
                in(reg) mask,
                );

            ret
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `waitirq` instruction wrapper
///
/// This function blocks until an interrupt becomes pending. The bitmask of
/// pending IRQs is returned.
///
/// A `0b1` at any offset in the return value indicates an interrupt is
/// pending on that interrupt line.
#[inline]
#[allow(unused_variables)]
pub unsafe fn waitirq() -> u32 {
    match () {
        #[cfg(riscv)]
        () => {
            let ret: u32;

            // The picorv32 waitirq specific values:
            //
            //     func7 = 0b0000100
            //     rd    = ret register
            //     rs1   = -----        (Don't care so set to zero [x0])
            asm!(
                ".insn r 0b0001011, 0, 0b0000100, {0}, zero, zero",
                out(reg) ret,
                );

            ret
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}

/// `timer` instruction wrapper
///
/// Reset the timer counter to a new value. The counter counts down clock
/// cycles and triggers the timer interrupt when transitioning from 1 to 0.
/// Setting the counter to zero disables the timer. The old value of the
/// counter is returned.
///
/// The timer interrupt is at offset 0 in the interrupt bitmask.
#[inline]
#[allow(unused_variables)]
pub unsafe fn timer(cycles_to_wait: u32) -> u32 {
    match () {
        #[cfg(riscv)]
        () => {
            let ret: u32;

            // The picorv32 timer specific values:
            //
            //     func7 = 0b0000101
            //     rd    = ret register
            //     rs1   = cycles_to_wait register
            asm!(
                ".insn r 0b0001011, 0, 0b0000101, {0}, {1}, zero",
                out(reg) ret,
                in(reg) cycles_to_wait,
                );

            ret
        }

        #[cfg(not(riscv))]
        () => unimplemented!(),
    }
}
