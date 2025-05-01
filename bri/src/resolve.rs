use crate::parse::{Jump, Op};

/// Resolves jump instructions to the actual jump location, and stores it.
pub fn resolve_jumps(ops: &mut [Op]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_loop() {
        let mut ops = vec![
            Op::Jump(Jump::JumpR(0)),
            Op::Increment(1),
            Op::Decrement(1),
            Op::Increment(1),
            Op::Decrement(1),
            Op::Jump(Jump::JumpL(0)),
        ];
        resolve_jumps(&mut ops);
        assert_eq!(
            ops,
            [
                Op::Jump(Jump::JumpR(6)),
                Op::Increment(1),
                Op::Decrement(1),
                Op::Increment(1),
                Op::Decrement(1),
                Op::Jump(Jump::JumpL(1))
            ]
        );
    }

    #[test]
    #[should_panic]
    fn mismatched_jump_r() {
        resolve_jumps(&mut vec![Op::Jump(Jump::JumpR(0))]);
    }

    #[test]
    #[should_panic]
    fn mismatched_jump_l() {
        resolve_jumps(&mut vec![Op::Jump(Jump::JumpL(0))]);
    }
}
