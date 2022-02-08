.intel_syntax noprefix
.global _start
_start:
  call main
  mov rdi, rax
  call exit

.global read
read:
  mov rax, 0
  syscall
  ret

.global write
write:
  mov rax, 1
  syscall
  ret

.global exit
exit:
  mov rax, 60
  syscall

