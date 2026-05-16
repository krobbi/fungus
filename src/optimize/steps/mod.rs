mod fold_exprs;
mod merge_basic_blocks;
mod replace_peepholes;

use crate::cfg::Cfg;

use super::context::Context;

/// Runs all optimization steps on a [`Cfg`] with a [`Context`].
pub fn run_all_steps(cfg: &mut Cfg, ctx: &mut Context) {
    merge_basic_blocks::run_step(cfg, ctx);
    replace_peepholes::run_step(cfg, ctx);
    fold_exprs::run_step(cfg, ctx);
}
