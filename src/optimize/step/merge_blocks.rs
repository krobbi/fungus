use crate::{
    ir::Label,
    optimize::{context::Context, graph::Graph},
};

/// Merges blocks with a single foreign predecessor from an unconditional jump
/// into their predecessor.
pub fn merge_blocks(graph: &mut Graph, ctx: &mut Context) {
    while let Some((predecessor, successor)) = find_edge(graph) {
        let mut successor = graph.remove_block(&successor);
        let predecessor = graph.block_mut(&predecessor);

        predecessor.instructions.append(&mut successor.instructions);
        predecessor.exit = successor.exit;
        ctx.mark_change();
    }
}

/// Finds the first edge eligable for block merging. Returns `None` if there are
/// no eligable edges.
fn find_edge(graph: &Graph) -> Option<(Label, Label)> {
    for predecessor in graph.labels() {
        let Some(successor) = graph.foreign_jump_successor(predecessor) else {
            continue; // The predecessor does not jump to a different successor.
        };

        if graph
            .labels()
            .filter(|l| *l != predecessor)
            .any(|l| graph.has_edge(l, successor))
        {
            continue; // The successor has another predecessor.
        }

        return Some((predecessor.clone(), successor.clone()));
    }

    None
}
