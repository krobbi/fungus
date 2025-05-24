use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter, Write},
};

use super::{BasicBlock, Label};

/// A graph of labeled basic blocks.
pub struct Program {
    /// The basic blocks.
    pub(super) basic_blocks: BTreeMap<Label, BasicBlock>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut data = String::new();
        for (label, basic_block) in &self.basic_blocks {
            writeln!(&mut data, "{label}:")?;

            for line in basic_block.to_string().lines() {
                writeln!(&mut data, "{:8}{line}", "")?;
            }

            writeln!(&mut data)?;
        }

        f.write_str(data.trim_end())
    }
}
