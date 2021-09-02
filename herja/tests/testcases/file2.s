.intel_syntax noprefix
.global _start
_start:
    call main
    mov rdi, rax
    call exit

.global exit
exit:
    mov rdi, rax
    mov rax, 60
    syscall
