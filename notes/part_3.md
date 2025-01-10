# Part 3: Jumps, Calls, and Comparisons

## Flags

Flags hold data like registers, but each flag hold only each one bit.
Flags are parts of larger registers.

## Flags

Pointers, same  thing as other languages: they hold data.
There are many types of pointers, for example: `rip`.

## Control Flow

The `rip` pointer points to the next instruction to execute, everytime
an instruction is execture `rip` is incremented by one.

## Jump

Jump sets the next instruction to be whatever label you give to it.
This means that `jmp` loads the given value into `rip`.

```asm
_start:
    jmp _start
```

The above code loops infinitly.

## Comparisons

Comparisons are if statements that work on registers & values,
and the output is setting a flag.

```asm
cmp rax, 23
cmp rax, rbx
```

If `cmp` find the values to be equal, `ZF` is set to `1`, else 
`ZF` is set to `0`.

Then, regardless of the comparison output, `SF` is set to
the most significant bit of `a - b`.

## Conditional jumping

After doing comparison `cmp`, a conditional jump can be used.
There are a multitude of conditional jump instructions, 
some work on signed values, others work on unsigned. Each conditional jump
instruction works off the output of `cmp a,b`:

- je `a = b`
- jne `a != b`
- etc...

### Conditiona Jump examples

```asm
cmp rax, 23     ; compare register rax and integer value 23
je _doThis      ; jump to '_doThis' if rax = 23

cmp rax, rbx    ; compare register rax and register rbx
jg _doThis      ; jump to '_doThis' if rax > rbx
```

## Registers as pointers

The below code moves the register `rbx` into `rax`.
To be more clear, it moves the address of `rbx` into rax, not `rbx`'s value.

```asm
mov rax, rbx
```

The below moves the value `rbx` is pointing to instead of `rbx`'s address.
This is done with the syntax of `[` `<addr>` `]`

```asm
mov rax, [rbx]
```

## Calls

A call is an extended form of `jmp`; the called label can use `ret` to
go back to the caller.

```asm
...
_start
    call _print     ; go to _print
    mov rax, 60     ; now back from _print due to ret
    mov rdi, 0
    syscall
_print:
    mov rax, 1
    mov rdi, 1
    mov rsi, text
    mov rdx, 14
    syscall         ; print things
    ret             ; return back to _start
```

[Next Part](part_4)
