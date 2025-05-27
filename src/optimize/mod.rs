mod context;

use context::OptimizationContext;

use crate::ir::Program;

/// Optimizes a program.
pub fn optimize_program(program: &mut Program) {
    let mut ctx = OptimizationContext::new(program);

    while ctx.should_run_pass() {
        run_pass(&mut ctx);
    }
}

/// Runs an optimization pass.
fn run_pass(_ctx: &mut OptimizationContext) {
    println!("TODO: Implement optimizations.");
}
