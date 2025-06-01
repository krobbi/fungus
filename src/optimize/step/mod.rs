mod merge_blocks;
mod remove_unreachable_blocks;
mod replace_instructions;
mod thread_jumps;

pub use merge_blocks::merge_blocks;
pub use remove_unreachable_blocks::remove_unreachable_blocks;
pub use replace_instructions::replace_instructions;
pub use thread_jumps::thread_jumps;
