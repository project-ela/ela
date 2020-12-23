.intel_syntax noprefix
.global _start
_start:
  call main
  mov rdi, 42
  xor eax, eax
  mov al, 60
  syscall
.global main
main:
  ret
