mod fold_exprs;
mod merge_basic_blocks;
mod remove_unreachable_basic_blocks;
mod replace_peepholes;
mod thread_jumps;
mod unroll_print_loops;

use crate::cfg::Cfg;

use super::context::Context;

/// Runs all optimization steps on a [`Cfg`] with a [`Context`].
pub fn run_all_steps(cfg: &mut Cfg, ctx: &mut Context) {
    merge_basic_blocks::run_step(cfg, ctx);
    thread_jumps::run_step(cfg, ctx);
    remove_unreachable_basic_blocks::run_step(cfg, ctx);
    replace_peepholes::run_step(cfg, ctx);
    fold_exprs::run_step(cfg, ctx);
    unroll_print_loops::run_step(cfg, ctx);
    remove_unreachable_basic_blocks::run_step(cfg, ctx);
}
