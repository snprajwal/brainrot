use std::cmp::Ordering;

use crate::parse::{Jump, Op};

pub fn optimise(ops: &mut Vec<Op>) {
    fold_consecutive_ops(Op::MoveL, Op::MoveR, ops);
    fold_consecutive_ops(Op::Decrement, Op::Increment, ops);
    rewrite_clear_loops(ops);
    remove_empty_ops(ops);
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

/// A loop of the form `[-]` just clears the value of the current memory cell.
/// This can be optimised to directly set the cell value to zero.
fn rewrite_clear_loops(ops: &mut [Op]) {
    let mut i = 0;
    while let Some([op1, op2, op3]) = ops.get_mut(i..i + 3) {
        if matches!(
            (&op1, &op2, &op3),
            (
                Op::Jump(Jump::JumpR(_)),
                Op::Decrement(_),
                Op::Jump(Jump::JumpL(_))
            )
        ) {
            *op1 = Op::Clear;
            *op2 = Op::Empty;
            *op3 = Op::Empty;
            i += 3;
        } else {
            i += 1;
        }
    }
}

fn remove_empty_ops(ops: &mut Vec<Op>) {
    ops.retain(|op| *op != Op::Empty);
}

#[cfg(test)]
mod tests {
    use crate::parse::{Jump, Op};

    #[test]
    fn fold_consecutive_ops_identical() {
        let mut ops = vec![Op::MoveR(1), Op::MoveR(1), Op::MoveR(1), Op::MoveR(1)];
        super::fold_consecutive_ops(Op::MoveL, Op::MoveR, &mut ops);
        assert_eq!(ops, [Op::MoveR(4), Op::Empty, Op::Empty, Op::Empty,]);
    }

    #[test]
    fn fold_consecutive_ops_net_positive() {
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
    fn fold_consecutive_ops_net_negative() {
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
    fn fold_consecutive_ops_net_zero() {
        let mut ops = vec![Op::MoveR(1), Op::MoveR(1), Op::MoveL(1), Op::MoveL(1)];
        super::fold_consecutive_ops(Op::MoveL, Op::MoveR, &mut ops);
        assert_eq!(ops, [Op::Empty, Op::Empty, Op::Empty, Op::Empty,]);
    }

    #[test]
    fn rewrite_clear_loops() {
        let mut ops = vec![
            Op::Jump(Jump::JumpR(0)),
            Op::Decrement(1),
            Op::Jump(Jump::JumpL(0)),
        ];
        super::rewrite_clear_loops(&mut ops);
        assert_eq!(ops, [Op::Clear, Op::Empty, Op::Empty,]);
    }

    #[test]
    fn remove_empty_ops() {
        let mut ops = vec![Op::Empty, Op::Empty, Op::Empty, Op::Empty];
        super::remove_empty_ops(&mut ops);
        assert_eq!(ops, []);
    }
}
