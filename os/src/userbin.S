    .section .data
    .global initproc_start
    .global initproc_end
    .global usershell_start
    .global usershell_end

    .align 12
initproc_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/initproc"
initproc_end:

    .section .data
.align 12
usershell_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/user_shell"
usershell_end:
