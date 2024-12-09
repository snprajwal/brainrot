#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Increment(usize),
    Decrement(usize),
    MoveR(usize),
    MoveL(usize),
    Jump(Jump),
    Set,
    Get,
    Debug,
    // Used during optimisation
    Empty,
}

impl TryFrom<char> for Op {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '+' => Self::Increment(1),
            '-' => Self::Decrement(1),
            '>' => Self::MoveR(1),
            '<' => Self::MoveL(1),
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

pub fn parse(src: &str) -> Vec<Op> {
    src.chars().flat_map(Op::try_from).collect()
}

#[cfg(test)]
mod tests {
    use super::{Jump, Op};

    #[test]
    fn trivial() {
        assert_eq!(
            super::parse("+-><[],.#"),
            vec![
                Op::Increment(1),
                Op::Decrement(1),
                Op::MoveR(1),
                Op::MoveL(1),
                Op::Jump(Jump::JumpR(0)),
                Op::Jump(Jump::JumpL(0)),
                Op::Set,
                Op::Get,
                Op::Debug,
            ]
        )
    }
}
