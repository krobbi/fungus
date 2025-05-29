use crate::{ir::Instruction, optimize::context::OptimizationContext};

/// Performs peephole optimization to replace instructions with more optimal
/// equivalents.
pub fn replace_instructions(ctx: &mut OptimizationContext) {
    let mut has_changes = false;
    for block in ctx.blocks_mut() {
        has_changes |= optimize_peepholes(&mut block.instructions, 2);
    }

    if has_changes {
        ctx.mark_change();
    }
}

/// Performs peephole optimization on a vector of instructions with a window
/// size and returns whether any changes were made.
fn optimize_peepholes(instructions: &mut Vec<Instruction>, window_size: usize) -> bool {
    let mut has_changes = false;

    let mut index = 0;
    loop {
        let range = index..index + window_size;
        let Some(peephole) = instructions.get(range.clone()) else {
            // Tried to index out of bounds. The end of the block was reached,
            // or the block is too small for the window.
            return has_changes;
        };

        if let Some(peephole) = optimize_peephole(peephole) {
            instructions.splice(range, peephole);
            has_changes = true;

            // Move the window backwards to try using the result of the
            // optimization.
            index = index.saturating_sub(window_size - 1);
        } else {
            index += 1; // No optimization could be made. Try the next window.
        }
    }
}

/// Returns an optimized equivalent of a peephole. Returns `None` if no
/// optimization could be made.
fn optimize_peephole(peephole: &[Instruction]) -> Option<Vec<Instruction>> {
    let peephole = match peephole {
        [Instruction::Duplicate, Instruction::Swap] => vec![Instruction::Duplicate],
        [Instruction::Duplicate, Instruction::Pop] | [Instruction::Swap, Instruction::Swap] => {
            Vec::new()
        }
        _ => return None,
    };
    Some(peephole)
}
