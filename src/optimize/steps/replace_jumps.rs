use crate::{
    cfg::{Cfg, Expr, Instruction, Label, Terminator, UnOp},
    optimize::{const_stack::ConstStack, context::Context},
};

/// Replaces jump [`Terminator`]s with a copy of their target
/// [`BasicBlock`][crate::cfg::BasicBlock]s if it would be more optimal.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    while let Some((source_label, target_label)) = find_pair(cfg) {
        let mut target_basic_block = cfg.basic_block(target_label).clone();
        let source_basic_block = cfg.basic_block_mut(source_label);
        source_basic_block
            .positions
            .extend(target_basic_block.positions);

        source_basic_block
            .instructions
            .append(&mut target_basic_block.instructions);

        source_basic_block.terminator = target_basic_block.terminator;
        ctx.mark_change();
    }
}

/// Finds and returns the first pair of [`Label`]s to merge in a [`Cfg`]. This
/// function returns [`None`] if there are no mergeable pairs of [`Label`]s.
fn find_pair(cfg: &Cfg) -> Option<(Label, Label)> {
    for source_label in cfg.labels() {
        let Terminator::Jump(target_label) = cfg.basic_block(source_label).terminator else {
            continue;
        };

        if target_label == source_label {
            continue;
        }

        let source_basic_block = cfg.basic_block(source_label);
        let target_basic_block = cfg.basic_block(target_label);

        if matches!(target_basic_block.terminator, Terminator::Branch(_, _)) {
            let mut const_stack = ConstStack::new();
            const_stack.eval_instructions(&source_basic_block.instructions);
            const_stack.eval_instructions(&target_basic_block.instructions);

            if const_stack.peek().is_some() {
                return Some((source_label, target_label));
            }
        }

        if target_basic_block.instructions.is_empty()
            && should_merge(
                source_basic_block.instructions.last(),
                &target_basic_block.terminator,
            )
        {
            return Some((source_label, target_label));
        }
    }

    None
}

/// Returns [`true`] if a [`Terminator`] should be merged after a previous
/// [`Instruction`].
fn should_merge(instruction: Option<&Instruction>, terminator: &Terminator) -> bool {
    match terminator {
        Terminator::Branch(_, _) => instruction.is_some_and(is_instruction_pre_branch),
        Terminator::Halt | Terminator::InfiniteLoop => true,
        _ => false,
    }
}

/// Returns [`true`] if an [`Instruction`] may be useful before a branch
/// [`Terminator`].
fn is_instruction_pre_branch(instruction: &Instruction) -> bool {
    match instruction {
        Instruction::Push(expr) => is_expr_pre_branch(expr),
        Instruction::Duplicate | Instruction::Unary(UnOp::Negate | UnOp::Bool | UnOp::Not) => true,
        _ => false,
    }
}

/// Returns [`true`] if an [`Expr`] may be useful before a branch
/// [`Terminator`].
fn is_expr_pre_branch(expr: &Expr) -> bool {
    match expr {
        Expr::Unary(UnOp::Negate | UnOp::Bool | UnOp::Not, _) => true,
        Expr::Sequence(_, expr) => is_expr_pre_branch(expr),
        _ => false,
    }
}
