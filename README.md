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

The interpreter performs a variety of optimisations before execution:

- Folding consecutive increment or decrement instructions
- Folding consecutive move instructions
- Rewriting loops that clear a memory cell (`[-]`) into a single instruction

Significant improvement in execution time is seen for the larger programs, with a **24x** speed-up for `hanoi.b`:

| Example        | Unoptimised | Optimised  | Improvement factor |
|----------------|-------------|------------|--------------------|
| `beer.b`       | 0.0062s     | 0.0031s    | 2x                 |
| `bootstrap.b`  | 20.3662s    | 14.8928s   | 1.37x              |
| `factor.b`     | 13.4522s    | 4.6786s    | 2.88x              |
| `golden.b`     | 0.1758s     | 0.1356s    | 1.3x               |
| `hanoi.b`      | 13.5071s    | 0.5665s    | 23.84x             |
| `mandelbrot.b` | 21.9095s    | 6.2807s    | 3.49x              |
| `numwarp.b`    | 0.0022s     | 0.0021s    | 1.05x              |
| `sierpinski.b` | 0.0028s     | 0.0017s    | 1.65x              |

Optimisations can be disabled by setting the `NO_OPT` environment variable (the value does not matter).

# Benchmark

The example programs can be benchmarked with the [`bench/bench.sh`](bench/bench.sh) script. It requires the path to the interpreter or compiler binary as an argument. The script also conveniently prints the table present above :)

# License

This project is licensed under the [MIT License](/LICENSE). Do whatever you want with it.
