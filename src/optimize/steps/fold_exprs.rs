use std::mem;

use crate::{
    cfg::{AssocOp, Cfg, Expr, Instruction, UnOp},
    optimize::context::Context,
    value::Value,
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
            let unfolded_expr = mem::replace(expr, Expr::Const(Value(0)));

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
        Expr::Assoc(op, terms) => fold_expr_assoc(op, terms, ctx),
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

/// Folds an associative [`Expr`].
fn fold_expr_assoc(op: AssocOp, terms: Vec<Expr>, ctx: &mut Context) -> Expr {
    let identity = assoc_identity(op);
    let mut accum = identity;
    let mut folded_terms = Vec::new();
    let mut const_count = 0_u32;
    let mut is_const_last = false;

    for term in terms {
        let term = fold_expr(term, ctx);
        is_const_last = false;

        match term {
            Expr::Const(value) => {
                accum = eval_assoc(op, accum, value);
                const_count += 1;
                is_const_last = true;
            }
            Expr::Assoc(sub_op, sub_terms) if sub_op == op => {
                for sub_term in sub_terms {
                    folded_terms.push(sub_term);
                }

                // Flattened a sub-expression.
                ctx.mark_change();
            }
            _ => folded_terms.push(term),
        }
    }

    if const_count > 1 || const_count == 1 && !is_const_last {
        // Collected and commuted constant terms.
        ctx.mark_change();
    }

    if accum != identity {
        folded_terms.push(Expr::Const(accum));
    } else if const_count > 0 {
        // Removed constant terms.
        ctx.mark_change();
    }

    match folded_terms.len() {
        0 => {
            // Reduced to identity.
            ctx.mark_change();
            Expr::Const(identity)
        }
        1 => {
            // Reduced to single term.
            ctx.mark_change();
            folded_terms.pop().expect("there should be one term")
        }
        _ => Expr::Assoc(op, folded_terms),
    }
}

/// Returns an [`AssocOp`]'s identity [`Value`].
const fn assoc_identity(op: AssocOp) -> Value {
    match op {
        AssocOp::Sum => Value(0),
        AssocOp::Product => Value(1),
    }
}

/// Evaluates an [`AssocOp`] with two term [`Value`]s.
fn eval_assoc(op: AssocOp, lhs: Value, rhs: Value) -> Value {
    match op {
        AssocOp::Sum => lhs + rhs,
        AssocOp::Product => lhs * rhs,
    }
}
