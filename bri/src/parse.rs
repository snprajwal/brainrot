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

impl TryFrom<char> for Op {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '+' => Self::Increment,
            '-' => Self::Decrement,
            '>' => Self::MoveR,
            '<' => Self::MoveL,
            // Jumps are initialised with the jump location to 0 by default. The jump resolution
            // pass will then set the actual locations for both the right and left jumps.
            '[' => Self::Jump(Jump::JumpR(0)),
            ']' => Self::Jump(Jump::JumpL(0)),
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
        let mut ops = src.chars().flat_map(Op::try_from).collect::<Vec<_>>();
        Self::resolve_jumps(&mut ops);
        ops
    }

    fn resolve_jumps(ops: &mut [Op]) {
        let mut stack = Vec::default();
        for (i, op) in ops.iter_mut().enumerate() {
            if let Op::Jump(jump) = op {
                match jump {
                    Jump::JumpR(r) => {
                        // Set the current position in the value pushed on the stack. This will be
                        // swapped with the position of the matching instruction ahead.
                        *r = i;
                        stack.push(jump);
                    }
                    Jump::JumpL(l) => {
                        let r = stack
                            .pop()
                            .map(|j| match j {
                                Jump::JumpR(r) => r,
                                Jump::JumpL(_) => {
                                    unreachable!("left jumps cannot be present on the stack");
                                }
                            })
                            .expect(&format!("unmatched `]` at position {}", i + 1));
                        // Insert the jump positions into the right and left jump instructions
                        (*r, *l) = (i + 1, *r + 1);
                    }
                }
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
