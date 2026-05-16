use std::mem;

use crate::{
    cfg::{Cfg, Expr, Instruction, UnOp},
    optimize::context::Context,
};

/// Performs constant folding on [`Expr`]s to replace them with more optimal
/// equivalents.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    for basic_block in cfg.basic_blocks_mut_unstable() {
        for instruction in &mut basic_block.instructions {
            let Instruction::Push(expr) = instruction else {
                // Instruction does not contain an expression.
                continue;
            };

            if matches!(expr, Expr::Const(_) | Expr::InputInt | Expr::InputChar) {
                // Expression cannot be folded further.
                continue;
            }

            // HACK: Replacing the expression with a placeholder allows constant
            // folding to take ownership of it without cloning.
            let unfolded_expr = mem::replace(expr, Expr::InputInt);

            *expr = fold_expr(unfolded_expr, ctx);
        }
    }
}

/// Folds an [`Expr`].
fn fold_expr(expr: Expr, ctx: &mut Context) -> Expr {
    match expr {
        Expr::Const(_) | Expr::InputInt | Expr::InputChar => expr,
        Expr::Unary(UnOp::Negate, rhs) => fold_expr_negate(*rhs, ctx),
        Expr::Unary(UnOp::Not, rhs) => fold_expr_not(*rhs, ctx),
        Expr::Binary(op, lhs, rhs) => {
            let lhs = fold_expr(*lhs, ctx);
            let rhs = fold_expr(*rhs, ctx);
            Expr::Binary(op, Box::new(lhs), Box::new(rhs))
        }
        Expr::Assoc(op, mut args) => {
            for arg in &mut args {
                // HACK: See above.
                let unfolded_arg = mem::replace(arg, Expr::InputInt);

                *arg = fold_expr(unfolded_arg, ctx);
            }

            Expr::Assoc(op, args)
        }
    }
}

/// Folds a unary arithmetic negation [`Expr`].
fn fold_expr_negate(rhs: Expr, ctx: &mut Context) -> Expr {
    let rhs = fold_expr(rhs, ctx);

    let folded_expr = match rhs {
        Expr::Const(value) => Expr::Const(-value),
        Expr::Unary(UnOp::Negate, double_negated_expr) => *double_negated_expr,
        _ => return Expr::Unary(UnOp::Negate, Box::new(rhs)),
    };

    ctx.mark_change();
    folded_expr
}

/// Folds a unary logical negation [`Expr`].
fn fold_expr_not(rhs: Expr, ctx: &mut Context) -> Expr {
    let rhs = fold_expr(rhs, ctx);

    let folded_expr = match rhs {
        Expr::Const(value) => Expr::Const(!value),
        Expr::Unary(UnOp::Negate, negated_expr) => Expr::Unary(UnOp::Not, negated_expr),
        _ => return Expr::Unary(UnOp::Not, Box::new(rhs)),
    };

    ctx.mark_change();
    folded_expr
}
