#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Increment,
    Decrement,
    MoveR,
    MoveL,
    Jump(Jump),
    Set,
    Get,
    Debug,
}

impl TryFrom<(usize, char)> for Op {
    type Error = ();
    fn try_from(value: (usize, char)) -> Result<Self, Self::Error> {
        Ok(match value.1 {
            '+' => Self::Increment,
            '-' => Self::Decrement,
            '>' => Self::MoveR,
            '<' => Self::MoveL,
            '[' => Self::Jump(Jump::JumpR(value.0)),
            ']' => Self::Jump(Jump::JumpL(value.0)),
            ',' => Self::Set,
            '.' => Self::Get,
            '#' => Self::Debug,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Jump {
    JumpR(usize),
    JumpL(usize),
}

#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn parse(src: &str) -> Vec<Op> {
        let mut ops = src
            .chars()
            .enumerate()
            .flat_map(Op::try_from)
            .collect::<Vec<_>>();
        Self::resolve_jumps(&mut ops);
        ops
    }

    fn resolve_jumps(ops: &mut [Op]) {
        let mut stack = Vec::default();
        for op in ops {
            match op {
                Op::Jump(jump @ Jump::JumpR(_)) => stack.push(jump),
                Op::Jump(Jump::JumpL(l)) => {
                    let r = stack
                        .pop()
                        .ok_or_else(|| panic!("unmatched `]` at position {}", *l + 1))
                        .and_then(|j| match j {
                            Jump::JumpR(r) => Ok(r),
                            Jump::JumpL(_) => {
                                unreachable!("left jumps cannot be present on the stack");
                            }
                        })
                        .unwrap();
                    // Insert the jump positions into the right and left jump instructions
                    (*r, *l) = (*l + 1, *r + 1);
                }
                _ => (),
            }
        }
        if let Some(Jump::JumpR(j)) = stack.pop() {
            panic!("unmatched `[` at position {}", *j + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Jump, Op, Parser};

    #[test]
    fn trivial() {
        assert_eq!(
            Parser::parse("+-><[],.#"),
            vec![
                Op::Increment,
                Op::Decrement,
                Op::MoveR,
                Op::MoveL,
                Op::Jump(Jump::JumpR(6)),
                Op::Jump(Jump::JumpL(5)),
                Op::Set,
                Op::Get,
                Op::Debug,
            ]
        )
    }

    #[test]
    #[should_panic]
    fn mismatched_jump_r() {
        Parser::parse("[");
    }

    #[test]
    #[should_panic]
    fn mismatched_jump_l() {
        Parser::parse("]");
    }
}
