/// Context for optimizing a program.
pub struct Context {
    /// Whether an optimization pass should be run.
    should_run_pass: bool,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self {
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

    /// Marks that a change was made to the program.
    pub fn mark_change(&mut self) {
        // Changes were made, so more optimization passes should be run.
        self.should_run_pass = true;
    }
}
