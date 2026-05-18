use std::mem;

use crate::{
    cfg::{AssocOp, BinOp, Cfg, Expr, Instruction, UnOp},
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
        Expr::Unary(op, rhs) => fold_expr_unary(op, *rhs, ctx),
        Expr::Binary(op, lhs, rhs) => fold_expr_binary(op, *lhs, *rhs, ctx),
        Expr::Assoc(op, terms) => fold_expr_assoc(op, terms, ctx),
        Expr::Sequence(prefix, expr) => fold_expr_sequence(prefix, *expr, ctx),
    }
}

/// Folds a unary [`Expr`].
fn fold_expr_unary(op: UnOp, rhs: Expr, ctx: &mut Context) -> Expr {
    let rhs = fold_expr(rhs, ctx);

    if let Some(rhs_value) = rhs.eval_const() {
        ctx.mark_change();
        return const_sequence(vec![rhs], op.eval(rhs_value));
    }

    let folded_expr = match (op, rhs) {
        // A double negation does nothing.
        (UnOp::Negate, Expr::Unary(UnOp::Negate, rhs)) => *rhs,

        // Operations which preserve non-zeroness are unnecessary before logical
        // operations.
        (UnOp::Bool | UnOp::Not, Expr::Unary(UnOp::Negate | UnOp::Bool, rhs)) => {
            Expr::Unary(op, rhs)
        }

        // A Boolean cast on a Boolean value does nothing.
        (
            UnOp::Bool,
            rhs @ (Expr::Unary(UnOp::Not, _)
            | Expr::Binary(
                BinOp::Greater | BinOp::GreaterEqual | BinOp::Less | BinOp::LessEqual,
                _,
                _,
            )),
        ) => rhs,

        // A double not is a Boolean cast.
        (UnOp::Not, Expr::Unary(UnOp::Not, rhs)) => Expr::Unary(UnOp::Bool, rhs),

        // A not comparison can be simplified to another comparison.
        (UnOp::Not, Expr::Binary(BinOp::Greater, lhs, rhs)) => {
            Expr::Binary(BinOp::LessEqual, lhs, rhs)
        }
        (UnOp::Not, Expr::Binary(BinOp::GreaterEqual, lhs, rhs)) => {
            Expr::Binary(BinOp::Less, lhs, rhs)
        }
        (UnOp::Not, Expr::Binary(BinOp::Less, lhs, rhs)) => {
            Expr::Binary(BinOp::GreaterEqual, lhs, rhs)
        }
        (UnOp::Not, Expr::Binary(BinOp::LessEqual, lhs, rhs)) => {
            Expr::Binary(BinOp::Greater, lhs, rhs)
        }

        (_, rhs) => return Expr::Unary(op, Box::new(rhs)),
    };

    ctx.mark_change();
    folded_expr
}

/// Folds a binary [`Expr`].
fn fold_expr_binary(op: BinOp, lhs: Expr, rhs: Expr, ctx: &mut Context) -> Expr {
    let lhs = fold_expr(lhs, ctx);
    let rhs = fold_expr(rhs, ctx);

    let folded_expr = match (op, lhs, rhs) {
        (BinOp::Divide, Expr::Const(lhs), Expr::Const(rhs)) => Expr::Const(lhs / rhs),
        (BinOp::Divide, Expr::Const(Value(0)), rhs) => const_sequence(vec![rhs], Value(0)),
        (BinOp::Divide, lhs, Expr::Const(Value(-1))) => Expr::Unary(UnOp::Negate, Box::new(lhs)),
        (BinOp::Divide, lhs, Expr::Const(Value(1))) => lhs,
        (BinOp::Modulo, Expr::Const(lhs), Expr::Const(rhs)) => Expr::Const(lhs % rhs),
        (BinOp::Greater, Expr::Const(lhs), Expr::Const(rhs)) => Expr::Const((lhs > rhs).into()),
        (BinOp::GreaterEqual, Expr::Const(lhs), Expr::Const(rhs)) => {
            Expr::Const((lhs >= rhs).into())
        }
        (BinOp::Less, Expr::Const(lhs), Expr::Const(rhs)) => Expr::Const((lhs < rhs).into()),
        (BinOp::LessEqual, Expr::Const(lhs), Expr::Const(rhs)) => Expr::Const((lhs <= rhs).into()),
        (_, lhs, rhs) => return Expr::Binary(op, Box::new(lhs), Box::new(rhs)),
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

/// Folds a sequence [`Expr`].
fn fold_expr_sequence(prefix: Vec<Expr>, expr: Expr, ctx: &mut Context) -> Expr {
    let mut folded_prefix = Vec::new();

    for prefix_expr in prefix {
        let prefix_expr = fold_expr(prefix_expr, ctx);

        if prefix_expr.is_read_only() {
            // Removed a prefix expression without side effects.
            ctx.mark_change();
            continue;
        }

        match prefix_expr {
            Expr::Unary(_, rhs) => {
                folded_prefix.push(*rhs);

                // Flattened a unary sub-expression.
                ctx.mark_change();
            }
            Expr::Binary(_, lhs, rhs) => {
                folded_prefix.push(*lhs);
                folded_prefix.push(*rhs);

                // Flattened a binary sub-expression.
                ctx.mark_change();
            }
            Expr::Assoc(_, terms) => {
                for term in terms {
                    folded_prefix.push(term);
                }

                // Flattened an associative sub-expression.
                ctx.mark_change();
            }
            Expr::Sequence(sub_prefix, sub_expr) => {
                for sub_prefix_expr in sub_prefix {
                    folded_prefix.push(sub_prefix_expr);
                }

                folded_prefix.push(*sub_expr);

                // Flattened a sequence sub-expression.
                ctx.mark_change();
            }
            _ => folded_prefix.push(prefix_expr),
        }
    }

    let expr = fold_expr(expr, ctx);

    if folded_prefix.is_empty() {
        // Reduced to result expression.
        ctx.mark_change();
        expr
    } else {
        Expr::Sequence(folded_prefix, Box::new(expr))
    }
}

/// Returns a new sequence [`Expr`] from a prefix and a constant [`Value`].
fn const_sequence(prefix: Vec<Expr>, value: Value) -> Expr {
    Expr::Sequence(prefix, Box::new(Expr::Const(value)))
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
