BITS 32

mov eax, 32
push eax
push 10
mov ebp, esp
mov dword [ebp-4], 120
push dword [ebp-4]
pop ebx
pop ecx
pop dword [ebp+4]
