use std::cmp::Ordering;

use crate::parse::{Jump, Op};

pub fn resolve(ops: &mut Vec<Op>) {
    fold_consecutive_ops(Op::MoveL, Op::MoveR, ops);
    fold_consecutive_ops(Op::Decrement, Op::Increment, ops);
    remove_empty_ops(ops);
    resolve_jumps(ops);
}

/// A pair of operations that move in opposite directions when visualised in a 2D
/// space can be considered a complementary pair, e.g.:
///
/// - decrement and increment on the number line
/// - move left and move right on a memory tape
///
/// This function accepts such a pair, and folds consecutive occurences of the operations
/// into a single "left" or "right" operation.
fn fold_consecutive_ops<L, R>(left: L, right: R, ops: &mut [Op])
where
    L: Fn(usize) -> Op,
    R: Fn(usize) -> Op,
{
    let mut i = 0;
    while i < ops.len() {
        if matches!(&ops[i], op if *op == left(1) || *op == right(1)) {
            let mut net = 0_isize;
            let start = i;

            // Accumulate consecutive ops
            while i < ops.len() {
                match &ops[i] {
                    op if op == &left(1) => net -= 1,
                    op if op == &right(1) => net += 1,
                    _ => break,
                }
                i += 1;
            }

            ops[start] = match net.cmp(&0) {
                Ordering::Less => left(net.abs() as usize),
                Ordering::Greater => right(net as usize),
                Ordering::Equal => Op::Empty,
            };

            // Replace the remaining moves with Op::Empty
            (start + 1..i).for_each(|i| {
                ops[i] = Op::Empty;
            });
        } else {
            i += 1;
        }
    }
}

fn remove_empty_ops(ops: &mut Vec<Op>) {
    ops.retain(|op| *op != Op::Empty);
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

#[cfg(test)]
mod tests {
    use crate::parse::{Jump, Op};

    #[test]
    fn fold_consecutive_moves_unidirectional() {
        let mut ops = vec![Op::MoveR(1), Op::MoveR(1), Op::MoveR(1), Op::MoveR(1)];
        super::fold_consecutive_ops(Op::MoveL, Op::MoveR, &mut ops);
        assert_eq!(ops, [Op::MoveR(4), Op::Empty, Op::Empty, Op::Empty,]);
    }

    #[test]
    fn fold_consecutive_moves_net_left() {
        let mut ops = vec![
            Op::MoveR(1),
            Op::MoveR(1),
            Op::MoveL(1),
            Op::MoveL(1),
            Op::MoveL(1),
            Op::MoveL(1),
        ];
        super::fold_consecutive_ops(Op::MoveL, Op::MoveR, &mut ops);
        assert_eq!(
            ops,
            [
                Op::MoveL(2),
                Op::Empty,
                Op::Empty,
                Op::Empty,
                Op::Empty,
                Op::Empty,
            ]
        );
    }

    #[test]
    fn fold_consecutive_moves_net_right() {
        let mut ops = vec![
            Op::MoveR(1),
            Op::MoveR(1),
            Op::MoveR(1),
            Op::MoveR(1),
            Op::MoveL(1),
            Op::MoveL(1),
        ];
        super::fold_consecutive_ops(Op::MoveL, Op::MoveR, &mut ops);
        assert_eq!(
            ops,
            [
                Op::MoveR(2),
                Op::Empty,
                Op::Empty,
                Op::Empty,
                Op::Empty,
                Op::Empty,
            ]
        );
    }

    #[test]
    fn fold_consecutive_moves_net_zero() {
        let mut ops = vec![Op::MoveR(1), Op::MoveR(1), Op::MoveL(1), Op::MoveL(1)];
        super::fold_consecutive_ops(Op::MoveL, Op::MoveR, &mut ops);
        assert_eq!(ops, [Op::Empty, Op::Empty, Op::Empty, Op::Empty,]);
    }

    #[test]
    fn remove_empty_ops() {
        let mut ops = vec![Op::Empty, Op::Empty, Op::Empty, Op::Empty];
        super::resolve(&mut ops);
        assert_eq!(ops, []);
    }

    #[test]
    fn resolve_jumps() {
        let mut ops = vec![
            Op::Jump(Jump::JumpR(0)),
            Op::Increment(1),
            Op::Decrement(1),
            Op::Increment(1),
            Op::Decrement(1),
            Op::Jump(Jump::JumpL(0)),
        ];
        super::resolve_jumps(&mut ops);
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
        super::resolve_jumps(&mut vec![Op::Jump(Jump::JumpR(0))]);
    }

    #[test]
    #[should_panic]
    fn mismatched_jump_l() {
        super::resolve_jumps(&mut vec![Op::Jump(Jump::JumpL(0))]);
    }
}
