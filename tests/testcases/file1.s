.intel_syntax noprefix
.global main
main:
    mov rdi, 1
    call exit
    mov rax, 1
    ret
