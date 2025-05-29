use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter, Write},
};

use super::{Block, Label};

/// A Befunge program.
pub struct Program {
    /// The blocks.
    pub blocks: BTreeMap<Label, Block>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut data = String::new();
        for (label, block) in &self.blocks {
            writeln!(&mut data, "{label}:")?;

            for line in block.to_string().lines() {
                writeln!(&mut data, "{:8}{line}", "")?;
            }

            writeln!(&mut data)?;
        }

        f.write_str(data.trim_end())
    }
}
