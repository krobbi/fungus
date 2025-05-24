mod basic_block;
mod build;
mod exit_point;
mod label;

pub use basic_block::BasicBlock;
pub use build::build_basic_block;

use exit_point::ExitPoint;
use label::Label;
