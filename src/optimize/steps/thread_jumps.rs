use std::collections::HashSet;

use crate::{
    cfg::{Cfg, Label, Terminator},
    optimize::context::Context,
};

/// Replaces [`Label`]s leading to chains of jumps with their target [`Label`]s.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    let mut threaded_labels = HashSet::new();
    threaded_labels.insert(Label::Main);

    for basic_block in cfg.basic_blocks_unstable() {
        if let Terminator::PutChecked(state) = basic_block.terminator {
            threaded_labels.insert(Label::State(state));
        }
    }

    while let Some(chain) = find_chain(cfg, &threaded_labels) {
        match chain {
            Chain::TightLoop(label) => {
                cfg.basic_block_mut(label).terminator = Terminator::InfiniteLoop;
            }
            Chain::LooseLoop(source_labels, target_label) => {
                let positions = collect_positions(cfg, &source_labels);
                let target_basic_block = cfg.basic_block_mut(target_label);
                target_basic_block.positions.extend(positions);
                target_basic_block.terminator = Terminator::InfiniteLoop;
            }
            Chain::Thread(source_labels, target_label) => {
                let positions = collect_positions(cfg, &source_labels);
                let target_basic_block = cfg.basic_block_mut(target_label);
                target_basic_block.positions.extend(positions);

                for basic_block in cfg.basic_blocks_mut_unstable() {
                    redirect_terminator(&mut basic_block.terminator, &source_labels, target_label);
                }

                threaded_labels.extend(source_labels);
            }
        }

        ctx.mark_change();
    }
}

/// A chain of jumps.
enum Chain {
    /// A direct jump to the same [`Label`].
    TightLoop(Label),

    /// An infinite loop between multiple [`Label`]s.
    LooseLoop(Vec<Label>, Label),

    /// A linear thread of [`Label`]s.
    Thread(Vec<Label>, Label),
}

/// Finds and returns the first [`Chain`] in a [`Cfg`] with a set of already
/// threaded [`Label`]s. This function returns [`None`] if the [`Cfg`] has no
/// [`Chain`]s.
fn find_chain(cfg: &Cfg, threaded_labels: &HashSet<Label>) -> Option<Chain> {
    for source_label in cfg.labels() {
        if threaded_labels.contains(&source_label) {
            continue;
        }

        let Some(mut target_label) = follow_label(cfg, source_label) else {
            continue;
        };

        if target_label == source_label {
            return Some(Chain::TightLoop(source_label));
        }

        let mut source_labels = vec![source_label];

        while let Some(next_target_label) = follow_label(cfg, target_label) {
            if source_labels.contains(&next_target_label) {
                return Some(Chain::LooseLoop(source_labels, target_label));
            }

            source_labels.push(target_label);
            target_label = next_target_label;
        }

        return Some(Chain::Thread(source_labels, target_label));
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

/// Collects and returns the positions of a slice of [`Label`]s in a [`Cfg`].
fn collect_positions(cfg: &Cfg, labels: &[Label]) -> Vec<(u16, u16)> {
    let mut positions = Vec::new();

    for label in labels {
        positions.extend(&cfg.basic_block(*label).positions);
    }

    positions
}

/// Redirects a [`Terminator`] from a slice of source [`Label`]s to a target
/// [`Label`].
fn redirect_terminator(terminator: &mut Terminator, source_labels: &[Label], target_label: Label) {
    match terminator {
        Terminator::Halt | Terminator::InfiniteLoop | Terminator::PutChecked(_) => (),
        Terminator::Jump(label) => redirect_label(label, source_labels, target_label),
        Terminator::Branch(then_label, else_label) => {
            redirect_label(then_label, source_labels, target_label);
            redirect_label(else_label, source_labels, target_label);
        }
        Terminator::Random(right_label, down_label, left_label, up_label) => {
            redirect_label(right_label, source_labels, target_label);
            redirect_label(down_label, source_labels, target_label);
            redirect_label(left_label, source_labels, target_label);
            redirect_label(up_label, source_labels, target_label);
        }
    }
}

/// Redirects a [`Label`] from a slice of source [`Label`]s to a target
/// [`Label`].
fn redirect_label(label: &mut Label, source_labels: &[Label], target_label: Label) {
    if source_labels.contains(label) {
        *label = target_label;
    }
}
