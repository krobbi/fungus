use crate::{
    common::Value,
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

        if let Some(peephole) = optimize_peephole(peephole, ctx) {
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
fn optimize_peephole(peephole: &[Instruction], ctx: &Context) -> Option<Vec<Instruction>> {
    use Instruction::{
        Binary, Divide, Duplicate, Get, GetAt, OutputChar, OutputInt, Pop, Print, Push, Swap, Unary,
    };

    let peephole = match peephole {
        [Push(x), Push(y), Get] => {
            if let (Ok(x), Ok(y)) = (usize::try_from(x.into_i32()), usize::try_from(y.into_i32())) {
                if ctx.is_in_bounds(x, y) {
                    return Some(vec![GetAt(x, y)]);
                }
            }
            vec![Push(Value::default())]
        }
        [Push(l), Push(r), Binary(o)] => vec![Push(o.eval(*l, *r))],
        [Push(a), Push(b), Swap] => vec![Push(*b), Push(*a)],
        [Push(r), Unary(o)] => vec![Push(o.eval(*r))],
        [Push(r), Divide(o)] if r.into_i32() != 0 => vec![Push(*r), Binary((*o).into())],
        [Push(v), Duplicate] => vec![Push(*v), Push(*v)],
        [Push(_) | Duplicate | GetAt(_, _), Pop] | [Swap, Swap] => Vec::new(),
        [Push(v), OutputInt] => vec![Print(v.into_i32().to_string() + " ")],
        [Push(v), OutputChar] => vec![Print(v.into_char_lossy().into())],
        [Unary(_), Pop] => vec![Pop],
        [Binary(_) | Get, Pop] => vec![Pop, Pop],
        [Duplicate, Swap] => vec![Duplicate],
        [Print(a), Print(b)] => vec![Print(a.clone() + b)],
        [a, b] if a.is_stack_operation() && b.is_statement() => vec![b.clone(), a.clone()],
        _ => return None,
    };
    Some(peephole)
}

impl Instruction {
    /// Returns whether the instruction has stack effects but no side effects.
    fn is_stack_operation(&self) -> bool {
        matches!(
            self,
            Self::Push(_)
                | Self::Unary(_)
                | Self::Binary(_)
                | Self::Duplicate
                | Self::Swap
                | Self::Pop
                | Self::Get
                | Self::GetAt(_, _)
        )
    }

    /// Returns whether the instruction has side effects but no stack effects.
    fn is_statement(&self) -> bool {
        matches!(self, Self::Print(_))
    }
}
