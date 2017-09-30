# thoughtfuck

A brainfuck Interpreter/REPL written in Rust.

## Synposis

    tf program.b

    Execute without arguments to start the REPL.

## Description

There isn't much in the ways of a concrete specification floating around, so this version adheres
to the more common rules of:

- "Machine" is 30,000 cells, all 1 byte in size. All cells are initialized to 0;
- Decrementing the cell pointer below index 0 causes a panic
- Incrementing the cell pointer above index 29,999 causes a panic
- Decrementing a cell value below 0 causes an underflow
- Incrementing a cell value above 255 causes an overflow
- Characters that are not a part of the 8 standard operators are ignored
    (though the REPL obviously has some "fondness" for newline characters)
