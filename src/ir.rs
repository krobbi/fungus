mod interpreter;
mod optimizer;

use std::{collections::HashMap, fmt};

use crate::{
    error::Result,
    playfield::Playfield,
    pointer::{Direction, Label, Mode, Pointer},
};

/// An intermediate program representation.
pub struct Program {
    /// The playfield.
    playfield: Playfield,

    /// The basic blocks.
    blocks: HashMap<Label, Block>,
}

impl Program {
    /// Create a new program from source code.
    pub fn new(source: &str) -> Result<Self> {
        let playfield = Playfield::new(source)?;
        let mut labels = vec![Label::default()];
        let mut blocks = HashMap::new();

        while let Some(label) = labels.pop() {
            if blocks.contains_key(&label) {
                continue;
            }

            let block = Block::new(&playfield, Pointer::from(label));
            labels.append(&mut block.exit.exit_labels());
            blocks.insert(label, block);
        }

        Ok(Self { playfield, blocks })
    }

    /// Optimize the program.
    pub fn optimize(&mut self) {
        optimizer::optimize_program(self);
    }

    /// Interpret the program.
    pub fn interpret(&self) {
        interpreter::interpret_program(self);
    }

    /// Print the program as pseudo-assembly.
    pub fn dump(&self) {
        let mut labels: Vec<&Label> = self.blocks.keys().collect();
        labels.sort();
        let mut labels = labels.iter().peekable();

        loop {
            let label = labels.next().unwrap();
            println!("{label}:");
            let block = self.blocks.get(label).unwrap();

            for instruction in &block.instructions {
                println!("\t{instruction};");
            }

            println!("\t{};", block.exit);

            match labels.peek() {
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
    fn new(playfield: &Playfield, pointer: Pointer) -> Self {
        let (instructions, exit) = match (pointer.mode(), playfield[&pointer]) {
            (_, '"') => (None, Exit::new_string(playfield, pointer)),
            (Mode::Command, command) => (
                Instruction::new(command),
                Exit::from_command(command, playfield, pointer),
            ),
            (Mode::String, value) => (
                Some(Instruction::Push(i32::try_from(u32::from(value)).unwrap())),
                Exit::new(playfield, pointer),
            ),
        };

        let instructions = match instructions {
            None => vec![],
            Some(instruction) => vec![instruction],
        };

        Self { instructions, exit }
    }
}

/// A basic block's instruction.
enum Instruction {
    /// An instruction to pop two values, add them, and push the result.  
    /// `[...][l][r]` -> `[...][l + r]`
    Add,

    /// An instruction to pop two values, subtract them, and push the result.  
    /// `[...][l][r]` -> `[...][l - r]`
    Subtract,

    /// An instruction to pop two values, multiply them, and push the result.  
    /// `[...][l][r]` -> `[...][l * r]`
    Multiply,

    /// An instruction to pop two values, divide them, and push the result.  
    /// `[...][l][r]` -> `[...][floor(l / r)]`
    Divide,

    /// An instruction to pop two values, modulo them, and push the result.  
    /// `[...][l][r]` -> `[...][l % r]`
    Modulo,

    /// An instruction to pop a value, logically negate it, and push it.  
    /// `[...][value]` -> `[...][value == 0 ? 1 : 0]`
    Not,

    /// An instruction to pop two values, compare them, and push the result.  
    /// `[...][l][r]` -> `[...][l > r ? 1 : 0]`
    Greater,

    /// An instruction to pop a value and push it twice.  
    /// `[...][value]` -> `[...][value][value]`
    Duplicate,

    /// An instruction to pop two values and push them in reverse order.  
    /// `[...][a][b]` -> `[...][b][a]`
    Swap,

    /// An instruction to pop a value.  
    /// `[...][value]` -> `[...]`
    Pop,

    /// An instruction to pop a value and output it as an integer.  
    /// `[...][value]` -> `[...]`
    OutputInteger,

    /// An instruction to pop a value and output it as a character.  
    /// `[...][value]` -> `[...]`
    OutputCharacter,

    /// An instruction to get a value from the playfield and push it.  
    /// `[...][x][y]` -> `[...][value]`
    Get,

    /// An instruction to get an integer from the user and push it.  
    /// `[...]` -> `[...][value]`
    InputInteger,

    /// An instruction to get a character from the user and push it.  
    /// `[...]` -> `[...][value]`
    InputCharacter,

    /// An instruction to push a value.  
    /// `[...]` -> `[...][value]`
    Push(i32),
}

impl Instruction {
    /// Create a new optional instruction from a command.
    fn new(command: char) -> Option<Self> {
        match command {
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            '%' => Some(Self::Modulo),
            '!' => Some(Self::Not),
            '`' => Some(Self::Greater),
            ':' => Some(Self::Duplicate),
            '\\' => Some(Self::Swap),
            '$' => Some(Self::Pop),
            '.' => Some(Self::OutputInteger),
            ',' => Some(Self::OutputCharacter),
            'g' => Some(Self::Get),
            '&' => Some(Self::InputInteger),
            '~' => Some(Self::InputCharacter),
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
            Self::Add => write!(f, "add"),
            Self::Subtract => write!(f, "subtract"),
            Self::Multiply => write!(f, "multiply"),
            Self::Divide => write!(f, "divide"),
            Self::Modulo => write!(f, "modulo"),
            Self::Not => write!(f, "not"),
            Self::Greater => write!(f, "greater"),
            Self::Duplicate => write!(f, "duplicate"),
            Self::Swap => write!(f, "swap"),
            Self::Pop => write!(f, "pop"),
            Self::OutputInteger => write!(f, "output integer"),
            Self::OutputCharacter => write!(f, "output character"),
            Self::Get => write!(f, "get"),
            Self::InputInteger => write!(f, "input integer"),
            Self::InputCharacter => write!(f, "input character"),
            Self::Push(value) => write!(f, "push {value}"),
        }
    }
}

/// A basic block's exit point.
enum Exit {
    /// An unconditional jump.
    Jump(Label),

    /// A random jump.
    Random(Label, Label, Label, Label),

    /// A conditional jump based on whether a popped stack value is zero.
    If { non_zero: Label, zero: Label },

    /// A program ending.
    End,
}

impl Exit {
    /// Create a new exit from a playfield and a pointer.
    fn new(playfield: &Playfield, mut pointer: Pointer) -> Self {
        pointer.advance(playfield);
        Self::Jump(Label::from(pointer))
    }

    /// Create a new string exit from a playfield and a pointer.
    fn new_string(playfield: &Playfield, mut pointer: Pointer) -> Self {
        pointer.toggle_mode();
        pointer.advance(playfield);
        Self::Jump(Label::from(pointer))
    }

    /// Create a new exit from a command, a playfield, and a pointer.
    fn from_command(command: char, playfield: &Playfield, mut pointer: Pointer) -> Self {
        match command {
            '>' => Self::from_direction(Direction::Right, playfield, pointer),
            '<' => Self::from_direction(Direction::Left, playfield, pointer),
            '^' => Self::from_direction(Direction::Up, playfield, pointer),
            'v' => Self::from_direction(Direction::Down, playfield, pointer),
            '?' => {
                let right = pointer.branch_label(Direction::Right, playfield);
                let down = pointer.branch_label(Direction::Down, playfield);
                let left = pointer.branch_label(Direction::Left, playfield);
                let up = pointer.branch_label(Direction::Up, playfield);
                Self::Random(right, down, left, up)
            }
            '_' => Self::from_if(Direction::Left, Direction::Right, playfield, &pointer),
            '|' => Self::from_if(Direction::Up, Direction::Down, playfield, &pointer),
            '#' => {
                pointer.advance(playfield);
                pointer.advance(playfield);
                Self::Jump(Label::from(pointer))
            }
            '@' => Self::End,
            _ => Self::new(playfield, pointer),
        }
    }

    /// Create a new exit from a direction, a playfield, and a pointer.
    fn from_direction(direction: Direction, playfield: &Playfield, mut pointer: Pointer) -> Self {
        pointer.face(direction);
        pointer.advance(playfield);
        Self::Jump(Label::from(pointer))
    }

    /// Create a new exit from branch directions, a playfield, and a pointer.
    fn from_if(
        non_zero: Direction,
        zero: Direction,
        playfield: &Playfield,
        pointer: &Pointer,
    ) -> Self {
        let non_zero = pointer.branch_label(non_zero, playfield);
        let zero = pointer.branch_label(zero, playfield);
        Self::If { non_zero, zero }
    }

    /// Get the exit labels as a vector.
    fn exit_labels(&self) -> Vec<Label> {
        match *self {
            Self::Jump(label) => vec![label],
            Self::Random(right, down, left, up) => vec![right, down, left, up],
            Self::If { non_zero, zero } => vec![non_zero, zero],
            Self::End => vec![],
        }
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jump(label) => write!(f, "jump {label}"),
            Self::Random(right, down, left, up) => {
                write!(f, "random {right}, {down}, {left}, {up}")
            }
            Self::If { non_zero, zero } => write!(f, "if {non_zero} else {zero}"),
            Self::End => write!(f, "end"),
        }
    }
}
