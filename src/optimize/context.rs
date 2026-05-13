use std::mem;

/// Context for optimizing a [`Cfg`][crate::cfg::Cfg].
pub struct Context {
    /// Whether an optimization pass should be run.
    should_run_pass: bool,
}

impl Context {
    /// Creates a new `Context`.
    pub const fn new() -> Self {
        Self {
            should_run_pass: true,
        }
    }

    /// Returns [`true`] if an optimization pass should be run.
    pub fn should_run_pass(&mut self) -> bool {
        mem::take(&mut self.should_run_pass)
    }

    /// Marks a change to the [`Cfg`][crate::cfg::Cfg].
    #[expect(dead_code, reason = "function should be used later")]
    pub const fn mark_change(&mut self) {
        self.should_run_pass = true;
    }
}
