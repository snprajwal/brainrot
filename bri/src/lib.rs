mod optimise;
mod parse;
mod resolve;

use std::io::Read;

use parse::{Jump, Op};

const RAM_SIZE: usize = 30_000;
const DEFAULT_DEBUG_RANGE: usize = 5;

#[derive(Debug)]
pub struct Cpu {
    pc: usize,
    ram: [u8; RAM_SIZE],
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            pc: 0,
            ram: [0; RAM_SIZE],
        }
    }
}

impl Cpu {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn exec(&mut self, ops: Vec<Op>) {
        let mut i = 0;
        while i < ops.len() {
            match ops[i] {
                Op::Increment(i) => {
                    self.ram[self.pc] =
                        self.ram[self.pc].wrapping_add((i % u8::MAX as usize) as u8);
                }
                Op::Decrement(i) => {
                    self.ram[self.pc] =
                        self.ram[self.pc].wrapping_sub((i % u8::MAX as usize) as u8);
                }
                Op::MoveR(i) => {
                    self.pc += i;
                    if self.pc >= RAM_SIZE {
                        panic!("attempting to move past the last memory cell");
                    }
                }
                Op::MoveL(i) => {
                    self.pc = self
                        .pc
                        .checked_sub(i)
                        .expect("attempting to move behind the first memory cell");
                }
                Op::Jump(Jump::JumpR(r)) => {
                    if self.ram[self.pc] == 0 {
                        i = r;
                        continue;
                    }
                }
                Op::Jump(Jump::JumpL(l)) => {
                    if self.ram[self.pc] != 0 {
                        i = l;
                        continue;
                    }
                }
                Op::Set => {
                    let mut buf = [0u8; 1];
                    std::io::stdin()
                        .read(&mut buf)
                        .expect("failed to read input");
                    self.ram[self.pc] = buf[0];
                }
                Op::Get => {
                    print!("{}", self.ram[self.pc] as char);
                }
                Op::Debug => {
                    self.debug();
                }
                Op::Clear => {
                    self.ram[self.pc] = 0;
                }
                Op::Empty => {
                    unreachable!("this should never have made it past the optimisations")
                }
            }
            i += 1;
        }
    }

    #[inline]
    fn debug(&self) {
        let debug_range = std::env::var("DEBUG_RANGE")
            .ok()
            .and_then(|r| r.parse().ok())
            .unwrap_or(DEFAULT_DEBUG_RANGE);
        let (start, end) = (
            self.pc.saturating_sub(debug_range),
            (self.pc + debug_range + 1).min(RAM_SIZE),
        );
        println!(
            "MEM: [{}{} ({}) {}{}]",
            if start > 0 { "..." } else { "" },
            self.ram[start..self.pc]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            self.ram[self.pc],
            self.ram[(self.pc + 1).min(RAM_SIZE)..end]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            if end < RAM_SIZE { "..." } else { "" },
        );
    }
}

pub fn run(src: &str, cpu: &mut Cpu) {
    let mut ops = parse::parse(src);
    if std::env::var("NO_OPT") == Err(std::env::VarError::NotPresent) {
        optimise::optimise(&mut ops);
    }
    resolve::resolve_jumps(&mut ops);
    cpu.exec(ops);
}
