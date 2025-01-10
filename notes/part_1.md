# Part 1: Hello World!

The `mov` instruction moves the second input into the first;
the below code moves the integer 123 into the register `ebx`

```asm
mov ebx, 123
```

The `add` instruction works the same;
`ebx` is added into `eax`

```asm
add eax, ebx
```

`mul` and `div` work differently, they always work with the `eax` register;
the lines below mulitply/divide into `eax`

```asm
mul ebx
div edx
```

The video presenter uses `int 0x80` for syscalls, this should be avoided.
Instead, use the `syscall` instruction.

[Next Part](part_2)
