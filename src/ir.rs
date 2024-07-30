use std::{collections::HashMap, fmt};

use crate::{
    playfield::Playfield,
    pointer::{Direction, Pointer},
};

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

        Self { blocks }
    }

    /// Print the program as pseudo-assembly.
    pub fn dump(&self) {
        let mut pointers: Vec<&Pointer> = self.blocks.keys().collect();
        pointers.sort();
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
        let command = playfield[pointer];

        Self {
            exit: Exit::from_command(command, playfield, pointer),
        }
    }
}

/// A basic block's exit point.
enum Exit {
    /// An unconditional jump.
    Jump(Pointer),

    /// A conditional jump based on whether a value popped from a stack is zero.
    If { non_zero: Pointer, zero: Pointer },

    /// A program ending.
    End,
}

impl Exit {
    /// Create a new exit from a playfield and a pointer.
    fn new(playfield: &Playfield, pointer: &Pointer) -> Self {
        let mut pointer = pointer.clone();
        pointer.advance(playfield);
        Self::Jump(pointer)
    }

    /// Create a new exit from a command, a playfield, and a pointer.
    fn from_command(command: char, playfield: &Playfield, pointer: &Pointer) -> Self {
        match command {
            '>' => Self::from_direction(Direction::Right, playfield, pointer),
            '<' => Self::from_direction(Direction::Left, playfield, pointer),
            '^' => Self::from_direction(Direction::Up, playfield, pointer),
            'v' => Self::from_direction(Direction::Down, playfield, pointer),
            '_' => Self::from_if(Direction::Left, Direction::Right, playfield, pointer),
            '|' => Self::from_if(Direction::Up, Direction::Down, playfield, pointer),
            '#' => {
                let mut pointer = pointer.clone();
                pointer.advance(playfield);
                pointer.advance(playfield);
                Self::Jump(pointer)
            }
            '@' => Self::End,
            _ => Self::new(playfield, pointer),
        }
    }

    /// Create a new exit from a direction, a playfield, and a pointer.
    fn from_direction(direction: Direction, playfield: &Playfield, pointer: &Pointer) -> Self {
        Self::Jump(pointer.to_facing(direction, playfield))
    }

    /// Create a new exit from if directions, a playfield, and a pointer.
    fn from_if(
        non_zero: Direction,
        zero: Direction,
        playfield: &Playfield,
        pointer: &Pointer,
    ) -> Self {
        let non_zero = pointer.to_facing(non_zero, playfield);
        let zero = pointer.to_facing(zero, playfield);
        Self::If { non_zero, zero }
    }

    /// Get the next pointers as a vector.
    fn pointers(&self) -> Vec<Pointer> {
        match self {
            Self::Jump(pointer) => vec![pointer.clone()],
            Self::If { non_zero, zero } => vec![non_zero.clone(), zero.clone()],
            Self::End => vec![],
        }
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jump(pointer) => write!(f, "jump {pointer}"),
            Self::If { non_zero, zero } => write!(f, "if {non_zero} else {zero}"),
            Self::End => write!(f, "end"),
        }
    }
}
