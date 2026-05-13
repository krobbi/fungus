mod merge_basic_blocks;

use crate::cfg::Cfg;

use super::context::Context;

/// Runs all optimization steps on a [`Cfg`] with a [`Context`].
pub const fn run_all_steps(cfg: &mut Cfg, ctx: &mut Context) {
    merge_basic_blocks::run_step(cfg, ctx);
}
