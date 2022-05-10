    .section .text.entry
    .globl _start
_start:
    # mv t3, a0
    # mv t4, a1
    # mv t5, a7
    # add a7, zero, 1
    # add a0, a0, 48
    # ecall
    # mv a0, t3
    # mv a1, t4
    # mv a7, t5
    # a0 = hartid
    # 1. set sp
    # sp = bootstack + (hartid + 1) * 0x8000(stacksize)
    la  t1, boot_stack
    add     t0, a0, 1
    slli    t0, t0, 15 # hart_id* stacksize
    add  sp, t1, t0
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16 * 5
    .globl boot_stack_top
boot_stack_top:
