use std::{collections::HashMap, fmt};

use crate::{playfield::Playfield, pointer::Pointer};

/// An intermediate program representation.
pub struct Program {
    /// The basic blocks.
    blocks: HashMap<Pointer, Block>,
}

impl Program {
    /// Create a new program from source code.
    pub fn new(source: &str) -> Self {
        let playfield = Playfield::new(source);
        let mut pointers = vec![Pointer::default()];
        let mut blocks = HashMap::new();

        while let Some(pointer) = pointers.pop() {
            if blocks.contains_key(&pointer) {
                continue;
            }

            let block = Block::new(&playfield, &pointer);
            pointers.append(&mut block.exit.pointers());
            blocks.insert(pointer, block);
        }

        let blocks = blocks;
        Self { blocks }
    }

    /// Print the program as pseudo-assembly.
    pub fn dump(&self) {
        let mut pointers: Vec<&Pointer> = self.blocks.keys().collect();
        pointers.sort();
        let pointers = pointers;
        let mut pointers = pointers.iter().peekable();

        loop {
            let pointer = pointers.next().unwrap();
            println!("{pointer}:");
            println!("\t{};", self.blocks.get(pointer).unwrap().exit);

            match pointers.peek() {
                None => break,
                Some(_) => println!(),
            }
        }
    }
}

/// A basic block.
struct Block {
    /// The exit point.
    exit: Exit,
}

impl Block {
    /// Create a new basic block from a playfield and a pointer.
    fn new(playfield: &Playfield, pointer: &Pointer) -> Self {
        Self {
            exit: Exit::new(playfield, pointer),
        }
    }
}

/// A basic block's exit point.
enum Exit {
    /// An unconditional jump to a basic block.
    Jump(Pointer),
}

impl Exit {
    /// Create a new exit from a playfield and a pointer.
    fn new(playfield: &Playfield, pointer: &Pointer) -> Self {
        let mut pointer = pointer.clone();
        pointer.advance(playfield);
        let pointer = pointer;
        Self::Jump(pointer)
    }

    /// Get the next pointers as a vector.
    fn pointers(&self) -> Vec<Pointer> {
        match self {
            Self::Jump(pointer) => vec![pointer.clone()],
        }
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jump(pointer) => write!(f, "goto {pointer}"),
        }
    }
}
