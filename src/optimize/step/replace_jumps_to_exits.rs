use crate::{
    ir::{Exit, Instruction, Label, ops::UnOp},
    optimize::{context::Context, graph::Graph},
};

/// Replaces unconditional jumps to block exits with the exit if it is more
/// optimal.
pub fn replace_jumps_to_exits(graph: &mut Graph, ctx: &mut Context) {
    while let Some((label, exit)) = find_replacement(graph) {
        graph.block_mut(&label).exit = exit;
        ctx.mark_change();
    }
}

/// Finds a label with a jump exit and an exit to replace it with. Returns
/// `None` if there are no replacements.
fn find_replacement(graph: &Graph) -> Option<(Label, Exit)> {
    for label in graph.labels() {
        let Some(exit) = graph.foreign_jump_successor(label) else {
            continue;
        };
        let exit = graph.block(exit);

        if !exit.instructions.is_empty() {
            continue;
        }

        let exit = &exit.exit;
        if matches!(
            (exit, graph.block(label).instructions.last()),
            (
                Exit::Branch(_, _),
                Some(Instruction::Push(_) | Instruction::Unary(UnOp::Not))
            ) | (Exit::End, _)
        ) {
            return Some((label.clone(), exit.clone()));
        }
    }

    None
}
