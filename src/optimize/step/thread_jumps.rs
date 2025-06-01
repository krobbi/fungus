use std::collections::{BTreeMap, BTreeSet};

use crate::{
    ir::{Exit, Label},
    optimize::{context::Context, graph::Graph},
};

/// Redirects labels targeting an unconditional jump to the jump's target.
pub fn thread_jumps(graph: &mut Graph, ctx: &mut Context) {
    let mut redirects = BTreeMap::new();
    for label in graph.labels() {
        map_redirects(label, graph, &mut redirects);
    }

    for block in graph.blocks_mut() {
        match &mut block.exit {
            Exit::Jump(l) => redirect_label(l, &redirects, ctx),
            Exit::Random(r, d, l, u) => {
                redirect_label(r, &redirects, ctx);
                redirect_label(d, &redirects, ctx);
                redirect_label(l, &redirects, ctx);
                redirect_label(u, &redirects, ctx);
            }
            Exit::Branch(t, e) => {
                redirect_label(t, &redirects, ctx);
                redirect_label(e, &redirects, ctx);
            }
            Exit::End => {}
        }
    }
}

/// Maps the chain of redirects from a source label.
fn map_redirects<'a>(label: &'a Label, graph: &'a Graph, redirects: &mut BTreeMap<Label, Label>) {
    if redirects.contains_key(label) {
        return; // Already mapped a redirect from the source label.
    }

    if let Some(mut target) = follow_label(label, graph) {
        let mut sources = BTreeSet::new();
        sources.insert(label);

        while let Some(next_target) = follow_label(target, graph) {
            if sources.contains(next_target) {
                return; // Infinite loop.
            }

            sources.insert(target);
            target = next_target;
        }

        for source in sources.into_iter().cloned() {
            redirects.insert(source, target.clone());
        }
    }
}

/// Follows a source label to a target label. Returns `None` is there is no
/// target label.
fn follow_label<'a>(label: &'a Label, graph: &'a Graph) -> Option<&'a Label> {
    if graph.block(label).instructions.is_empty() {
        graph.foreign_jump_successor(label)
    } else {
        None
    }
}

/// Redirects a label using a map of redirects.
fn redirect_label(label: &mut Label, redirects: &BTreeMap<Label, Label>, ctx: &mut Context) {
    if let Some(target) = redirects.get(label) {
        *label = target.clone();
        ctx.mark_change();
    }
}
