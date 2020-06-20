BITS 32
org 0x7C00

start:
  jmp test1
  jmp test2

test1:
  jmp test2

test2:
