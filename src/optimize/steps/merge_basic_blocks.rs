use crate::{
    cfg::{Cfg, Label, Terminator},
    optimize::context::Context,
};

/// Merges [`BasicBlock`][crate::cfg::BasicBlock]s with a single unconditional
/// predecessor into their predecessors.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    while let Some((source, target)) = find_mergeable_edge(cfg) {
        let mut target = cfg.remove_basic_block(target);
        let source = cfg.basic_block_mut(source);

        source.instructions.append(&mut target.instructions);
        source.terminator = target.terminator;
        ctx.mark_change();
    }
}

/// Finds and returns an arbitrary mergeable edge in a [`Cfg`]. This function
/// returns [`None`] if the [`Cfg`] has no mergeable edges.
fn find_mergeable_edge(cfg: &Cfg) -> Option<(Label, Label)> {
    for source in cfg.labels_unstable() {
        let Terminator::Jump(target) = cfg.basic_block(source).terminator else {
            continue;
        };

        debug_assert_ne!(target, Label::Main, "main entry point is a target");

        if target != source && is_edge_target_unique(cfg, source, target) {
            return Some((source, target));
        }
    }

    None
}

/// Returns [`true`] if an unconditional edge in a [`Cfg`] is the only edge with
/// its target [`Label`].
fn is_edge_target_unique(cfg: &Cfg, source: Label, target: Label) -> bool {
    for other_source in cfg.labels_unstable() {
        if other_source == source {
            continue;
        }

        if cfg
            .basic_block(other_source)
            .terminator
            .labels()
            .contains(&target)
        {
            return false;
        }
    }

    true
}
