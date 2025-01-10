# Part 2: Hello World! Breakdown

Different register sizes use different naming schemes.
When you use smaller sized registers you're still using the normal(64-bit)
register.

## System calls

System calls run using the first register `rax` to determine what call to use.
For example `sys_write` using the ID `1`. Each syscall uses the following
bytes differently, for example `sys_write` uses the 1st argument as a
file descriptor, the 2nd as a buffer, and the 3rd as a count.

Each input is also treated differently, the 1st & 3rd argument take the real
values while the 2nd argument takes a buffer address.

### Examples

```asm
mov rax, 1      ; sys_write
mov rdi, 1      ; stdout
mov rsi, msg    ; msg address
mov rdx, len    ; msg length
syscall
```

```asm
mov rax, 60     ; sys_exit
mov rdi, 0      ; success error code
syscall
```

## Sections

There are different sections of each asm file:

- `.data` : static compile-time defined data.
- `.bss`  : runtime defined data.
- `.text` : the runnable code.

## Labels & Globals

Labels are use to name blocks of code.

```asm
    global _start
_start: 
```

The above labels the code appearing after to be called `_start`, until another
label is found. The `global start` enables `_start`, to be found by the linker.

The `_start` label is a bit special as the linker will look for it, throwing an
error if it is not found since it is the program entry.

## Putting it all together

```asm
section   .data
    message: db "Hello, World", 10  ; note the newline at the end

section   .text
    global    _start

_start: 
    mov       rax, 1            ; system call for write
    mov       rdi, 1            ; file handle 1 is stdout
    mov       rsi, message      ; address of string to output
    mov       rdx, 13           ; number of bytes
    syscall                     ; invoke operating system to do the write
    mov       rax, 60           ; system call for exit
    xor       rdi, rdi          ; exit code 0
    syscall                     ; invoke operating system to exit
```

[Next Part](part_3)
