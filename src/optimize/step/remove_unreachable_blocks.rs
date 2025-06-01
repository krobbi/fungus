use std::collections::BTreeSet;

use crate::{
    ir::Label,
    optimize::{context::Context, graph::Graph},
};

/// Finds unreachable blocks and removes them.
pub fn remove_unreachable_blocks(graph: &mut Graph, ctx: &mut Context) {
    let mut pending_labels = BTreeSet::new();
    let mut reachable_labels = BTreeSet::new();
    pending_labels.insert(&Label::Main);

    while let Some(label) = pending_labels.pop_first() {
        if !reachable_labels.contains(label) {
            reachable_labels.insert(label.clone());
            pending_labels.extend(graph.exit_labels(label));
        }
    }

    for label in &graph.labels_cloned() {
        if !reachable_labels.contains(label) {
            graph.remove_block(label);
            ctx.mark_change();
        }
    }
}
