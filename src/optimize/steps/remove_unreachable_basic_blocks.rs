use std::collections::HashSet;

use crate::{
    cfg::{Cfg, Label},
    optimize::context::Context,
};

/// Removes unreachable [`BasicBlock`][crate::cfg::BasicBlock]s from a [`Cfg`].
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    let mut pending_labels = vec![Label::Main];
    let mut reachable_labels = HashSet::new();

    while let Some(label) = pending_labels.pop() {
        if reachable_labels.contains(&label) {
            continue;
        }

        reachable_labels.insert(label);
        pending_labels.extend(cfg.basic_block(label).terminator.labels());
    }

    let all_labels: Vec<_> = cfg.labels_unstable().collect();

    for label in all_labels {
        if !reachable_labels.contains(&label) {
            cfg.remove_basic_block(label);
            ctx.mark_change();
        }
    }
}
