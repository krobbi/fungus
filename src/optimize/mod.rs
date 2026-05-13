mod context;
mod steps;

use crate::cfg::Cfg;

use self::context::Context;

/// Optimizes a [`Cfg`].
pub fn optimize_cfg(cfg: &mut Cfg) {
    let mut ctx = Context::new();

    while ctx.should_run_pass() {
        run_pass(cfg, &mut ctx);
    }
}

/// Runs an optimization pass on a [`Cfg`] with a [`Context`].
const fn run_pass(cfg: &mut Cfg, ctx: &mut Context) {
    steps::run_all_steps(cfg, ctx);
}
