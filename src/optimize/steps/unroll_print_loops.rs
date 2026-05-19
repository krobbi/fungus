use crate::{
    cfg::{Cfg, Instruction, Label, Terminator},
    optimize::context::Context,
};

/// Replaces print loops with print stack [`Instruction`]s.
pub fn run_step(cfg: &mut Cfg, ctx: &mut Context) {
    while let Some((cond_label, body_label, exit_label)) = find_print_loop(cfg) {
        let mut positions = Vec::new();
        positions.extend(cfg.basic_block(body_label).positions.iter());
        positions.extend(cfg.basic_block(cond_label).positions.iter());

        let cond_block = cfg.basic_block_mut(cond_label);
        cond_block.positions.extend(positions.iter());
        cond_block.instructions = vec![Instruction::PrintStack];
        cond_block.terminator = Terminator::Jump(exit_label);

        for basic_block in cfg.basic_blocks_mut_unstable() {
            if let Terminator::Jump(next_label) = basic_block.terminator
                && next_label == cond_label
            {
                basic_block.positions.extend(positions.iter());
                basic_block.instructions.push(Instruction::PrintStack);
                basic_block.terminator = Terminator::Jump(exit_label);
            }
        }

        ctx.mark_change();
    }
}

/// Finds and returns an aribtrary print loop's condition, body, and exit
/// [`Label`]s in a [`Cfg`]. This function returns [`None`] if the [`Cfg`] has
/// no print loops.
fn find_print_loop(cfg: &Cfg) -> Option<(Label, Label, Label)> {
    for body_label in cfg.labels_unstable() {
        let Some(cond_label) = find_cond_label(cfg, body_label) else {
            continue;
        };

        let Some(exit_label) = find_exit_label(cfg, cond_label, body_label) else {
            continue;
        };

        return Some((cond_label, body_label, exit_label));
    }

    None
}

/// Finds and returns a print loop's condition [`Label`] from its body [`Label`]
/// in a [`Cfg`]. This function returns [`None`] if the [`Label`] is not a print
/// loop's body [`Label`].
fn find_cond_label(cfg: &Cfg, body_label: Label) -> Option<Label> {
    let body_block = cfg.basic_block(body_label);

    match (&body_block.instructions[..], &body_block.terminator) {
        ([Instruction::OutputChar], Terminator::Jump(cond_label)) => Some(*cond_label),
        (_, _) => None,
    }
}

/// Finds and returns a print loop's exit [`Label`] from its condition and body
/// [`Label`] in a [`Cfg`]. This function returns [`None`] if the condition and
/// body [`Label`]s are not part of a print loop.
fn find_exit_label(cfg: &Cfg, cond_label: Label, body_label: Label) -> Option<Label> {
    let cond_block = cfg.basic_block(cond_label);

    match (&cond_block.instructions[..], &cond_block.terminator) {
        ([Instruction::Duplicate], Terminator::Branch(then_label, exit_label))
            if *then_label == body_label
                && *exit_label != cond_label
                && *exit_label != body_label =>
        {
            Some(*exit_label)
        }
        (_, _) => None,
    }
}
