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
            let block = self.blocks.get(pointer).unwrap();

            for instruction in &block.instructions {
                println!("\t{instruction};");
            }

            println!("\t{};", block.exit);

            match pointers.peek() {
                None => break,
                Some(_) => println!(),
            }
        }
    }
}

/// A basic block.
struct Block {
    /// The instructions.
    instructions: Vec<Instruction>,

    /// The exit point.
    exit: Exit,
}

impl Block {
    /// Create a new basic block from a playfield and a pointer.
    fn new(playfield: &Playfield, pointer: &Pointer) -> Self {
        let command = playfield[pointer];

        let instructions = match Instruction::new(command) {
            None => vec![],
            Some(instruction) => vec![instruction],
        };

        Self {
            instructions,
            exit: Exit::from_command(command, playfield, pointer),
        }
    }
}

/// A basic block's instruction.
enum Instruction {
    /// An instruction to push a value to the stack.
    Push(i32),
}

impl Instruction {
    /// Create a new optional instruction from a command.
    fn new(command: char) -> Option<Self> {
        match command {
            '0' => Some(Self::Push(0)),
            '1' => Some(Self::Push(1)),
            '2' => Some(Self::Push(2)),
            '3' => Some(Self::Push(3)),
            '4' => Some(Self::Push(4)),
            '5' => Some(Self::Push(5)),
            '6' => Some(Self::Push(6)),
            '7' => Some(Self::Push(7)),
            '8' => Some(Self::Push(8)),
            '9' => Some(Self::Push(9)),
            _ => None,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Push(value) => write!(f, "push {value}"),
        }
    }
}

/// A basic block's exit point.
enum Exit {
    /// An unconditional jump.
    Jump(Pointer),

    /// A random jump.
    Random(Pointer, Pointer, Pointer, Pointer),

    /// A conditional jump based on whether a popped stack value is zero.
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
            '?' => {
                let p0 = pointer.to_facing(Direction::Right, playfield);
                let p1 = pointer.to_facing(Direction::Down, playfield);
                let p2 = pointer.to_facing(Direction::Left, playfield);
                let p3 = pointer.to_facing(Direction::Up, playfield);
                Self::Random(p0, p1, p2, p3)
            }
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
            Self::Random(p0, p1, p2, p3) => vec![p0.clone(), p1.clone(), p2.clone(), p3.clone()],
            Self::If { non_zero, zero } => vec![non_zero.clone(), zero.clone()],
            Self::End => vec![],
        }
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jump(pointer) => write!(f, "jump {pointer}"),
            Self::Random(p0, p1, p2, p3) => write!(f, "random {p0}, {p1}, {p2}, {p3}"),
            Self::If { non_zero, zero } => write!(f, "if {non_zero} else {zero}"),
            Self::End => write!(f, "end"),
        }
    }
}
