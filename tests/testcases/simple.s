.intel_syntax noprefix
.global main
main:
  mov rax, 1
  call hoge
  ret

.global hoge
hoge:
  call exit
  ret
