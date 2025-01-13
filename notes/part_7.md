# Part 6: Macros

A macro is a predefined instruction that expands into a set of extrunctions.
A macro's defining feature is that it happens in compile-time, the resulting
code will not see a macro.

## Examples

Below is a macro that exits the program. The `0` after exit defines the
number of inputs, which is 0 in this case.

```asm
%macro exit 0
    mov rax, 60
    mov rdi, 0
    syscall
%endmacro
```

Next, a macro the prints a digit using the previously mentioned `printRaxDigit`

```asm
%macro print_digit 1
    mov rax, %1
    call printRaxDigit
%endmacro

_start:
    print_digit 3
    print_digit 4

    exit
```

## Macro Arguments

When inputting multiple arguments into a macro, you use commas to divide them.

```asm
%macro print_digit 2
    mov rax, %1
    add rax, %2
    call printRaxDigit
%endmacro

_start:
    print_digit 3, 2

    exit
```

## Local Labels

Special syntax is used for label within macros to avoid redefining the same
label in the resulting assembly.

```asm
%macro freeze 0
%%loop:
    jmp %%loop
%endmacro
```


## Constants use `equ`

You can use `equ` to define compile time constants

```asm
SYS_EXIT        equ 60
EXIT_SUCCESS    equ 0

_start:
    mov rax, SYS_EXIT
    mov rdi, EXIT_SUCCESS
    syscall
```

### Inclucing external files

To include external files, use the `include` macro

```asm
%include "file_name.asm"
```

The above code effectively replaces itself with the code from that file.

[Next Part](part_8)
