# Part 11: Writing files

## `sys_open`

To open files, you can use the `sys_open` syscall(2).
Arg-1 is the filename, arg-2 are the flags, arg-3 is the mode.
The flags denote what action to do(read, write, create, append, etc).

## `sys_write`

To write files, you can use the `sys_open` syscall(1).
Arg-1 is the filename descriptor, arg-2 is the buffer to write,
arg-3 is the length of the buffer to write. This is similar to writing to `stdout`,
but using the file descriptor recieved from `sys_open` instead of `stdout`'s id.

## `sys_close`

You need to close the file after opening it, using the `sys_close` syscall(3).
Arg-1 is the filename-descriptor.

See [write-file](../src/5-write-file.asm)

[Next Part](part_12)
