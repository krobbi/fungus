mod context;
mod graph;
mod step;

use context::Context;
use graph::Graph;

use crate::{common::Playfield, ir::Program, parse::FlowGraph};

/// Optimizes a program with a flow graph and a playfield.
pub fn optimize_program(program: &mut Program, flow_graph: &FlowGraph, playfield: &Playfield) {
    let mut graph = Graph::new(program);
    let mut ctx = Context::new(flow_graph, playfield);

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
    step::replace_jumps_to_exits(graph, ctx);
    step::optimize_branches(graph, ctx);
}
