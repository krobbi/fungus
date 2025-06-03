use crate::{common::Playfield, parse::FlowGraph};

/// Context for optimizing a program.
pub struct Context<'a> {
    /// Whether an optimization pass should be run.
    should_run_pass: bool,

    /// The flow graph.
    _flow_graph: &'a FlowGraph,

    /// The playfield.
    playfield: &'a Playfield,
}

impl<'a> Context<'a> {
    /// Creates a new context from a flow graph and a playfield.
    pub fn new(flow_graph: &'a FlowGraph, playfield: &'a Playfield) -> Self {
        Self {
            should_run_pass: true,
            _flow_graph: flow_graph,
            playfield,
        }
    }

    /// Returns whether an optimization pass should be run.
    pub fn should_run_pass(&mut self) -> bool {
        let should_run_pass = self.should_run_pass;

        // Do not run another pass unless changes are made.
        self.should_run_pass = false;

        should_run_pass
    }

    /// Marks that a change was made to the program.
    pub fn mark_change(&mut self) {
        // Changes were made, so more optimization passes should be run.
        self.should_run_pass = true;
    }

    /// Returns whether a position in cells is in bounds of the playfield.
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        let (width, height) = self.playfield.bounds();
        x < width && y < height
    }
}
