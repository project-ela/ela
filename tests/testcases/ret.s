.intel_syntax noprefix
.global _start
_start:
  mov rdi, 42
  xor eax, eax
  mov al, 60
  syscall
