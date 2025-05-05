use std::{
    env,
    io::{self, Write},
    path::Path,
};

use bri::{run, Cpu};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_repl(),
        1 => run_file(&args[0]),
        _ => {
            eprintln!("Multiple input files provided, they will be run in the provided order");
            for arg in &args {
                run_file(arg);
            }
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn run_repl() {
    println!(
        "Brainrot REPL v{} on {} ({}), Copyright (c) {}",
        VERSION,
        env::consts::OS,
        env::consts::ARCH,
        AUTHORS
    );
    let (stdin, mut stdout) = (io::stdin(), io::stdout());
    let mut cpu = Cpu::default();
    loop {
        let mut line = String::default();
        print!(">>> ");
        stdout.flush().expect("failed to flush stdout");
        let n = stdin.read_line(&mut line).expect("failed to read line");
        // If zero bytes are read, then exit (usually triggered by Ctrl-D)
        if n == 0 {
            break;
        }
        if line.eq("\\reset") {
            cpu.reset();
            continue;
        }
        run(&line, &mut cpu);
        print!("\n");
    }
}

fn run_file(path: impl AsRef<Path>) {
    let src = std::fs::read_to_string(path).expect("failed to read program");
    run(&src, &mut Cpu::default());
}
