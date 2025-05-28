use crate::{ir::Label, optimize::context::OptimizationContext};

/// Merges blocks with a single foreign predecessor from an unconditional jump
/// into their predecessor.
pub fn merge_blocks(ctx: &mut OptimizationContext) {
    while let Some((predecessor, successor)) = find_edge(ctx) {
        let mut successor = ctx.remove_block(&successor);
        let predecessor = ctx.block_mut(&predecessor);

        predecessor.instructions.append(&mut successor.instructions);
        predecessor.exit = successor.exit;
        ctx.mark_change();
    }
}

/// Finds the first edge eligable for block merging. Returns `None` if there are
/// no eligable edges.
fn find_edge(ctx: &OptimizationContext) -> Option<(Label, Label)> {
    for predecessor in ctx.labels() {
        let Some(successor) = ctx.foreign_jump_successor(predecessor) else {
            continue; // The predecessor does not jump to a different successor.
        };

        if ctx
            .labels_except(predecessor)
            .any(|l| ctx.has_edge(l, successor))
        {
            continue; // The successor has another predecessor.
        }

        return Some((predecessor.clone(), successor.clone()));
    }

    None
}
