# Part 9: Command Line Arguments

Parsing command line strings.

## How it works

When a program starts, command line arguments are loaded into the stack.
The top-most (last-pushed, first-popped) denotes the number of arguments.
Next comes the path to the executable that was called. Then follows the 
paths to the 0-terminated strings.

The second item in the stack is technically the first argument, but it is
system-provided. The first user-provided arguments is the third item on the
stack. This means that argc is always at-least 1.

For example:

```sh
./arg-test arg1 arg2 arg3
```

Leads to:

```py
argc = 4
path = './arg-test'
arg[1] = 'arg1'
arg[2] = 'arg2'
arg[3] = 'arg3'
```

[Next Part](part_10)
