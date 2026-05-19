use crate::{
    cfg::{Cfg, Label, Terminator},
    optimize::context::Context,
};

/// Replaces [`Label`]s leading to chains of jumps with their target [`Label`]s.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    while let Some(chain) = find_chain(cfg) {
        match chain {
            Chain::TightLoop(label) => {
                cfg.basic_block_mut(label).terminator = Terminator::InfiniteLoop;
                ctx.mark_change();
            }
        }
    }
}

/// A chain of jumps.
enum Chain {
    /// A direct jump to the same [`Label`].
    TightLoop(Label),
}

/// Finds and returns the first [`Chain`] in a [`Cfg`]. This function returns
/// [`None`] if the [`Cfg`] has no [`Chain`]s.
fn find_chain(cfg: &Cfg) -> Option<Chain> {
    for source_label in cfg.labels() {
        let Some(target_label) = follow_label(cfg, source_label) else {
            continue;
        };

        if target_label == source_label {
            return Some(Chain::TightLoop(source_label));
        }
    }

    None
}

/// Returns a source [`Label`]'s jump target [`Label`] in a [`Cfg`]. This
/// function returns [`None`] if the source [`Label`] has no jump target
/// [`Label`].
fn follow_label(cfg: &Cfg, source_label: Label) -> Option<Label> {
    let source_basic_block = cfg.basic_block(source_label);

    if source_basic_block.instructions.is_empty()
        && let Terminator::Jump(target_label) = source_basic_block.terminator
    {
        Some(target_label)
    } else {
        None
    }
}
