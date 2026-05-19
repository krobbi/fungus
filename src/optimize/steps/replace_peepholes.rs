use crate::{
    cfg::{AssocOp, BinOp, Cfg, Expr, Instruction, UnOp},
    optimize::context::Context,
    value::Value,
};

/// Replaces peepholes of [`Instruction`]s with more optimal equivalents.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    for basic_block in cfg.basic_blocks_mut_unstable() {
        optimize_window(&mut basic_block.instructions, 2, ctx);
        optimize_window(&mut basic_block.instructions, 3, ctx);
    }
}

/// Optimizes a sliding window of [`Instruction`] peepholes.
fn optimize_window(instructions: &mut Vec<Instruction>, window_size: usize, ctx: &mut Context) {
    let mut index = 0;

    loop {
        let range = index..index + window_size;

        let Some(peephole) = instructions.get(range.clone()) else {
            break;
        };

        if let Some(peephole) = optimize_peephole(peephole) {
            instructions.splice(range, peephole);
            ctx.mark_change();
            index = index.saturating_sub(window_size - 1);
        } else {
            index += 1;
        }
    }
}

/// Returns an optimized equivalent of a peephole of [`Instruction`]s. This
/// function returns [`None`] if the peephole could not be optimized.
#[expect(clippy::too_many_lines, reason = "function is a single pattern match")]
fn optimize_peephole(peephole: &[Instruction]) -> Option<Vec<Instruction>> {
    use Instruction::{
        Assoc, Binary, Duplicate, OutputChar, OutputInt, Pop, Print, PrintStack, Push, Swap, Unary,
    };

    let peephole = match peephole {
        // * Dividing or multiplying a value by -1 negates it.
        [Push(expr), Binary(BinOp::Divide) | Assoc(AssocOp::Product)]
            if expr.eval_const() == Some(Value(-1)) =>
        {
            vec![Unary(UnOp::Negate), Push(expr.clone()), Pop]
        }

        // * Dividing or multiplying a value by 0 is always zero.
        [Push(expr), Binary(BinOp::Divide) | Assoc(AssocOp::Product)]
            if expr.eval_const() == Some(Value(0)) =>
        {
            vec![Pop, Push(expr.clone())]
        }

        // * Adding 0 to a value does nothing.
        [Push(expr), Assoc(AssocOp::Sum)] if expr.eval_const() == Some(Value(0)) => {
            vec![Push(expr.clone()), Pop]
        }

        // * Dividing or multiplying a value by 1 does nothing.
        [Push(expr), Binary(BinOp::Divide) | Assoc(AssocOp::Product)]
            if expr.eval_const() == Some(Value(1)) =>
        {
            vec![Push(expr.clone()), Pop]
        }

        // * A value modulo -1, 0, or 1 is always zero.
        [Push(expr), Binary(BinOp::Modulo)] if let Some(Value(-1..=1)) = expr.eval_const() => {
            vec![Pop, Push(expr.clone()), Pop, Push(Expr::Const(Value(0)))]
        }

        // * Expressions can be swapped if at least one expression has no side
        //   effects.
        [Push(a), Push(b), Swap] if a.is_read_only() || b.is_read_only() => {
            vec![Push(b.clone()), Push(a.clone())]
        }

        // * Build binary expressions.
        [Push(lhs), Push(rhs), Binary(op)] => vec![Push(Expr::Binary(
            *op,
            Box::new(lhs.clone()),
            Box::new(rhs.clone()),
        ))],

        // * Build associative expressions.
        [Push(lhs), Push(rhs), Assoc(op)] => {
            vec![Push(Expr::Assoc(*op, vec![lhs.clone(), rhs.clone()]))]
        }

        // * Popping an expression without side effects does nothing.
        [Push(expr), Pop] if expr.is_read_only() => vec![],

        // * Push pop push is a sequence.
        [Push(prefix), Pop, Push(expr)] => vec![Push(Expr::Sequence(
            vec![prefix.clone()],
            Box::new(expr.clone()),
        ))],

        // * Replacing push pop pop with pop push pop unblocks some peephole
        //   optimizations.
        [Push(expr), Pop, Pop] => vec![Pop, Push(expr.clone()), Pop],

        // * A duplicated constant can be replaced with itself.
        [Push(expr), Duplicate] if let Some(value) = expr.eval_const() => {
            vec![Push(expr.clone()), Push(Expr::Const(value))]
        }

        // * Build unary expressions.
        [Push(rhs), Unary(op)] => vec![Push(Expr::Unary(*op, Box::new(rhs.clone())))],

        // * Build print statements.
        [Push(expr), OutputInt] if let Some(value) = expr.eval_const() => {
            vec![Push(expr.clone()), Pop, Print(format!("{value} "))]
        }
        [Push(expr), OutputChar] if let Some(value) = expr.eval_const() => vec![
            Push(expr.clone()),
            Pop,
            Print(value.to_char_lossy().to_string()),
        ],
        [Push(expr), PrintStack] if let Some(value) = expr.eval_const() => {
            if value.is_non_zero() {
                vec![
                    Push(expr.clone()),
                    Pop,
                    Print(value.to_char_lossy().to_string()),
                    PrintStack,
                ]
            } else {
                vec![Push(expr.clone())]
            }
        }

        // * Popping a duplicated value does nothing.
        // * Swapping twice does nothing.
        // * Negating twice does nothing.
        [Duplicate, Pop] | [Swap, Swap] | [Unary(UnOp::Negate), Unary(UnOp::Negate)] => vec![],

        // * Swapping after duplicating is unnecessary.
        [Duplicate, Swap] => vec![Duplicate],

        // * A value subtracted from itself is always zero.
        // * A value remainder itself is always zero.
        // * A value is never greater than itself.
        // * A value is never less than itself.
        [Duplicate, Unary(UnOp::Negate), Assoc(AssocOp::Sum)]
        | [
            Duplicate,
            Binary(BinOp::Modulo | BinOp::Greater | BinOp::Less),
        ] => {
            vec![Pop, Push(Expr::Const(Value(0)))]
        }

        // * A value is always greater than or equal to itself.
        // * A value is always less than or equal to itself.
        [Duplicate, Binary(BinOp::GreaterEqual | BinOp::LessEqual)] => {
            vec![Pop, Push(Expr::Const(Value(1)))]
        }

        // * Adding a value to itself is the same as multiplying by 2.
        [Duplicate, Assoc(AssocOp::Sum)] => {
            vec![Push(Expr::Const(Value(2))), Assoc(AssocOp::Product)]
        }

        // * Swapping before popping twice is unnecessary.
        // * No binary operators have any side effects. Popping the result of a
        //   binary operation can be replaced with popping its operands.
        [Swap, Pop, Pop] | [Binary(_) | Assoc(_), Pop] => vec![Pop, Pop],

        // * Swapping before a comparison reverses it.
        // * Not after a comparison inverts it.
        [Swap, Binary(BinOp::Greater)] | [Binary(BinOp::GreaterEqual), Unary(UnOp::Not)] => {
            vec![Binary(BinOp::Less)]
        }
        [Swap, Binary(BinOp::GreaterEqual)] | [Binary(BinOp::Greater), Unary(UnOp::Not)] => {
            vec![Binary(BinOp::LessEqual)]
        }
        [Swap, Binary(BinOp::Less)] | [Binary(BinOp::LessEqual), Unary(UnOp::Not)] => {
            vec![Binary(BinOp::Greater)]
        }
        [Swap, Binary(BinOp::LessEqual)] | [Binary(BinOp::Less), Unary(UnOp::Not)] => {
            vec![Binary(BinOp::GreaterEqual)]
        }

        // * Swapping before a commutative operation is unnecessary.
        [Swap, Assoc(op)] => vec![Assoc(*op)],

        // * No unary operators have any side effects. Popping the result of a
        //   unary operation can be replaced with popping its operand.
        [Unary(_), Pop] => vec![Pop],

        // * Negation and Boolean casts do not affect whether a value is
        //   non-zero, so they are unnecessary before a Boolean cast.
        // * A double not is a Boolean cast.
        [Unary(UnOp::Negate | UnOp::Bool), Unary(UnOp::Bool)]
        | [Unary(UnOp::Not), Unary(UnOp::Not)] => {
            vec![Unary(UnOp::Bool)]
        }

        // * Negation and Boolean casts do not affect whether a value is
        //   non-zero, so they are unnecessary before a not operation.
        [Unary(UnOp::Negate | UnOp::Bool), Unary(UnOp::Not)] => vec![Unary(UnOp::Not)],

        // * A Boolean cast on a Boolean value is unnecessary.
        [
            instruction @ (Unary(UnOp::Not)
            | Binary(
                BinOp::Greater | BinOp::GreaterEqual | BinOp::Less | BinOp::LessEqual,
            )),
            Unary(UnOp::Bool),
        ] => vec![instruction.clone()],

        // * Concatenate print statements.
        [Print(prefix), Print(suffix)] => vec![Print(format!("{prefix}{suffix}"))],

        // * Bubble print statements.
        [quiet, print @ Print(_)] if !matches!(quiet, Pop) && quiet.is_quiet() => {
            vec![print.clone(), quiet.clone()]
        }

        _ => return None,
    };

    Some(peephole)
}
