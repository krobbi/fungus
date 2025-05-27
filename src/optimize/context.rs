use crate::ir::Program;

/// Context for optimizing a program.
pub struct OptimizationContext<'a> {
    /// The program.
    _program: &'a mut Program,

    /// Whether an optimization pass should be run.
    should_run_pass: bool,
}

impl<'a> OptimizationContext<'a> {
    /// Creates a new optimization context from a program.
    pub fn new(program: &'a mut Program) -> Self {
        Self {
            _program: program,
            should_run_pass: true,
        }
    }

    /// Returns whether an optimization pass should be run.
    pub fn should_run_pass(&mut self) -> bool {
        let should_run_pass = self.should_run_pass;

        // Do not run another pass unless changes are made.
        self.should_run_pass = false;

        should_run_pass
    }
}
