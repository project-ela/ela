BITS 32
org 0x7C00
mov eax, 42
mov ecx, 16
mov eax, ecx
mov ebx, eax
mov ebp, esp
mov dword [ebp-4], 10
mov dword [ebp-4], eax
mov eax, [ebp-4]
