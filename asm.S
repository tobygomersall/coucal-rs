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
