# Part 5: Math Operatorions and the Stack

## Math Operations

You can do math on registers, for example:

```asm
add rax, 5      ; rax = rax + rbx
sub rbx, rax    ; rax = rax - rbx
mul 4           ; rax = rax * 4
div rbx         ; rax = rax / rbx
imul 4          ; rax = rax * 4     (signed)
div rbx         ; rax = rax / rbx   (signed)
neg rax         ; rax = -rax
inc rax         ; rax = rax + 1
dec rax         ; rax = rax - 1
adc rax, rbx    ; rax = rax+rbx+CF
sbb rax, rbx    ; rax = rax-rbx-CF
```

## Displaying digits

Displaying a single digit

```asm
section .data
    digit db 0,10

printRAXDigit:
    add rax, 48
    mov [digit], al
    mov rax, 1
    mov rdi, 1
    mov rsi, digit
    mov rdx, 2
    syscall
    ret
```

## The stack

Same as the stack concept in higher level languages; pop, push, etc.

```asm
push rax        ; push rax onto the stack
push 10         ; push 10 onto the stack
pop rax         ; pop 10 into rax
pop rax         ; pop previous rax into rax
mov rax, [rsp]  ; stores peeked value into rax
```

[Next Part](part_6)
