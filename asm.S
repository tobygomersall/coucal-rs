#include "custom_ops.S"

.section .text.__maskirq
.global __maskirq
__maskirq:
    mv a0, a1
    picorv32_maskirq_insn(a0, a1)
    ret

.section .text.__waitirq
.global __waitirq
__waitirq:
    picorv32_waitirq_insn(a0)
    ret

.section .text.__timer
.global __timer
__timer:
    mv a0, a1
    picorv32_timer_insn(a0, a1)
    ret

#ifdef RV32RT_INTERRUPTS_QREGS

.section .text.__getq2
.global __getq2
__getq2:
    picorv32_getq_insn(a0, q2)
    ret

.section .text.__getq3
.global __getq3
__getq3:
    picorv32_getq_insn(a0, q3)
    ret

#endif
