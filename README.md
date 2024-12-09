# Brainrot

An interpreter and (coming soon) optimising compiler for [brainfuck](https://esolangs.org/wiki/Brainfuck), aptly named after the Oxford Word of the Year 2024.

# Usage

To run the Brainrot REPL, simply execute `cargo run`.

To invoke the interpreter on a `.b` file, pass the file as an argument to the above command, i.e. `cargo run file.b`.

# Getting started

Brainfuck is an extremely simple Turing-complete language which operates on an array of memory cells. The language uses just eight instructions (and an unofficial debug instruction):

| Instruction | Description                                                                 |
|-------------|-----------------------------------------------------------------------------|
| `+`         | Increment the current cell value by 1                                       |
| `-`         | Decrement the current cell value by 1                                       |
| `>`         | Move to the next cell to the right                                          |
| `<`         | Move to the previous cell to the left                                       |
| `[`         | Jump past the corresponding `]` if the current cell value is zero           |
| `]`         | Jump back to the corresponding `[` if the current cell value is non-zero    |
| `.`         | Display the current cell value                                              |
| `,`         | Read an input character and set it as the current cell value                |
| `#`         | Display the current cell, with `DEBUG_RANGE` preceding and succeeding cells |

The number of preceding and succeeding cells displayed with the debug instruction can be set with the `DEBUG_RANGE` environment variable (5 by default).

Here's an example program that echoes your input back:

```brainfuck
,[.,]
```

And one that prints `Hello world!`:

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

# Optimisations

The interpreter performs three key optimisations prior to runtime:

- Folding consecutive increment or decrement instructions
- Folding consecutive move instructions
- Resolving jump locations

This is enough to provide a ~3.5x boost in performance, as seen with [`examples/mandelbrot.b`](/examples/mandelbrot.b):

- Without optimisations: 21.62s
- With optimisations: 6.1s

# License

This project is licensed under the [MIT License](/LICENSE). Do whatever you want with it.
