use crate::{
    ir::{Exit, Instruction, ops::UnOp},
    optimize::{context::Context, graph::Graph},
};

/// Optimizes branch exits to more optimal equivalents.
pub fn optimize_branches(graph: &mut Graph, ctx: &mut Context) {
    for block in graph.blocks_mut() {
        if let Exit::Branch(then_label, else_label) = &block.exit {
            let (then_label, else_label) = (then_label.clone(), else_label.clone());

            if then_label == else_label {
                block.instructions.push(Instruction::Pop); // Pop condition.
                block.exit = Exit::Jump(then_label);
                ctx.mark_change();
            } else {
                match block.instructions.last() {
                    Some(Instruction::Push(v)) => {
                        block.exit = Exit::Jump(if v.into_i32() != 0 {
                            then_label
                        } else {
                            else_label
                        });
                        block.instructions.pop(); // Remove condition.
                        ctx.mark_change();
                    }
                    Some(Instruction::Unary(UnOp::Not)) => {
                        block.instructions.pop(); // Remove logical negation.
                        block.exit = Exit::Branch(else_label, then_label);
                        ctx.mark_change();
                    }
                    _ => {}
                }
            }
        }
    }
}
