use crate::{
    ir::Instruction,
    optimize::{context::Context, graph::Graph},
};

/// Performs peephole optimization to replace instructions with more optimal
/// equivalents.
pub fn replace_instructions(graph: &mut Graph, ctx: &mut Context) {
    for block in graph.blocks_mut() {
        optimize_peepholes(&mut block.instructions, 3, ctx);
        optimize_peepholes(&mut block.instructions, 2, ctx);
    }
}

/// Performs peephole optimization on a vector of instructions with a window
/// size and returns whether any changes were made.
fn optimize_peepholes(instructions: &mut Vec<Instruction>, window_size: usize, ctx: &mut Context) {
    let mut index = 0;
    loop {
        let range = index..index + window_size;
        let Some(peephole) = instructions.get(range.clone()) else {
            return;
        };

        if let Some(peephole) = optimize_peephole(peephole) {
            instructions.splice(range, peephole);
            ctx.mark_change();

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
        [
            Instruction::Push(l),
            Instruction::Push(r),
            Instruction::Binary(o),
        ] => vec![Instruction::Push(o.eval(*l, *r))],
        [Instruction::Push(r), Instruction::Unary(o)] => vec![Instruction::Push(o.eval(*r))],
        [Instruction::Push(r), Instruction::Divide(o)] if r.into_i32() != 0 => {
            vec![Instruction::Push(*r), Instruction::Binary((*o).into())]
        }
        [
            Instruction::Push(_) | Instruction::Duplicate,
            Instruction::Pop,
        ]
        | [Instruction::Swap, Instruction::Swap] => Vec::new(),
        [Instruction::Unary(_), Instruction::Pop] => vec![Instruction::Pop],
        [Instruction::Binary(_) | Instruction::Get, Instruction::Pop] => {
            vec![Instruction::Pop, Instruction::Pop]
        }
        [Instruction::Duplicate, Instruction::Swap] => vec![Instruction::Duplicate],
        _ => return None,
    };
    Some(peephole)
}
