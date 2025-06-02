mod context;
mod graph;
mod step;

use context::Context;
use graph::Graph;

use crate::ir::Program;

/// Optimizes a program.
pub fn optimize_program(program: &mut Program) {
    let mut graph = Graph::new(program);
    let mut ctx = Context::new();

    while ctx.should_run_pass() {
        run_pass(&mut graph, &mut ctx);
    }
}

/// Runs an optimization pass.
fn run_pass(graph: &mut Graph, ctx: &mut Context) {
    step::merge_blocks(graph, ctx);
    step::thread_jumps(graph, ctx);
    step::remove_unreachable_blocks(graph, ctx);
    step::replace_instructions(graph, ctx);
    step::optimize_branches(graph, ctx);
}
