#include "custom_ops.S"

.section .text.__retirq
.global __retirq
__retirq:
    picorv32_retirq_insn()
    ret

.section .text.__maskirq
.global __maskirq
__maskirq:
    mv a1, a0
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
    mv a1, a0
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

.section .text.__setq2
.global __setq2
__setq2:
    mv a1, a0
    picorv32_setq_insn(q2, a1)
    ret

.section .text.__setq3
.global __setq3
__setq3:
    mv a1, a0
    picorv32_setq_insn(q3, a1)
    ret

#endif
