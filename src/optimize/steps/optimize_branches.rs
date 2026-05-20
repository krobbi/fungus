use crate::{
    cfg::{Cfg, Expr, Instruction, Terminator, UnOp},
    optimize::{const_stack::ConstStack, context::Context},
};

/// Optimizes [`BasicBlock`][crate::cfg::BasicBlock]s with branch
/// [`Terminator`]s.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    for basic_block in cfg.basic_blocks_mut_unstable() {
        let Terminator::Branch(then_label, else_label) = basic_block.terminator else {
            continue;
        };

        if then_label == else_label {
            basic_block.instructions.push(Instruction::Pop);
            basic_block.terminator = Terminator::Jump(then_label);
            ctx.mark_change();
            continue;
        }

        let mut const_stack = ConstStack::new();
        const_stack.eval_instructions(&basic_block.instructions);

        if let Some(value) = const_stack.peek() {
            let label = if value.is_non_zero() {
                then_label
            } else {
                else_label
            };

            basic_block.instructions.push(Instruction::Pop);
            basic_block.terminator = Terminator::Jump(label);
            ctx.mark_change();
            continue;
        }

        match basic_block.instructions.last() {
            Some(Instruction::Push(expr))
                if let Some((expr, is_negated)) = fold_condition(expr) =>
            {
                basic_block.instructions.pop();
                basic_block.instructions.push(Instruction::Push(expr));

                if is_negated {
                    basic_block.terminator = Terminator::Branch(else_label, then_label);
                }

                ctx.mark_change();
            }
            Some(Instruction::Unary(UnOp::Negate | UnOp::Bool)) => {
                basic_block.instructions.pop();
                ctx.mark_change();
            }
            Some(Instruction::Unary(UnOp::Not)) => {
                basic_block.instructions.pop();
                basic_block.terminator = Terminator::Branch(else_label, then_label);
                ctx.mark_change();
            }
            _ => (),
        }
    }
}

/// Returns a more optimal equivalent of a condition [`Expr`] and whether it is
/// negated.
fn fold_condition(expr: &Expr) -> Option<(Expr, bool)> {
    match expr {
        Expr::Unary(UnOp::Negate | UnOp::Bool, rhs) => Some((*rhs.clone(), false)),
        Expr::Unary(UnOp::Not, rhs) => Some((*rhs.clone(), true)),
        Expr::Sequence(prefix, expr) if let Some((expr, is_negated)) = fold_condition(expr) => {
            Some((Expr::Sequence(prefix.clone(), Box::new(expr)), is_negated))
        }
        _ => None,
    }
}
