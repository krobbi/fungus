mod merge_blocks;
mod optimize_branches;
mod remove_unreachable_blocks;
mod replace_instructions;
mod replace_jumps_to_exits;
mod thread_jumps;

pub use merge_blocks::merge_blocks;
pub use optimize_branches::optimize_branches;
pub use remove_unreachable_blocks::remove_unreachable_blocks;
pub use replace_instructions::replace_instructions;
pub use replace_jumps_to_exits::replace_jumps_to_exits;
pub use thread_jumps::thread_jumps;
