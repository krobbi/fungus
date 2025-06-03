use std::{
    collections::VecDeque,
    io::{self, Write},
};

use crate::{
    common::{Playfield, Value},
    ir::{Block, Exit, Instruction, Label, Program, State, ops::BinOp},
    parse,
};

/// Interprets a program with a playfield.
pub fn interpret_program(program: &Program, playfield: &mut Playfield) {
    Interpreter::new(playfield).interpret_program(program);
}

/// A high-level interpreter for potentially self-modifying programs.
struct Interpreter<'a> {
    /// The playfield.
    playfield: &'a mut Playfield,

    /// The stack.
    stack: Vec<Value>,

    /// The character input buffer.
    input_chars: VecDeque<char>,
}

impl<'a, 'b> Interpreter<'a> {
    /// Creates a new interpreter from a playfield.
    fn new(playfield: &'a mut Playfield) -> Self {
        Self {
            playfield,
            stack: Vec::new(),
            input_chars: VecDeque::new(),
        }
    }

    /// Interprets a program.
    fn interpret_program(&mut self, program: &Program) {
        let mut program = program;
        let mut recompiled_program;
        let mut label = Label::Main;

        loop {
            match self.interpret_block(&program.blocks[&label]) {
                Flow::Jump(l) => label = l.clone(),
                Flow::Recompile(s) => {
                    recompiled_program = parse::parse_program_state(self.playfield, s.clone());
                    program = &recompiled_program;
                    label = Label::Main;
                }
                Flow::End => break,
            }
        }

        flush_output();
    }

    /// Interprets a block and returns the control flow from the block.
    fn interpret_block(&mut self, block: &'b Block) -> Flow<'b> {
        for instruction in &block.instructions {
            if let Some(state) = self.interpret_instruction(instruction) {
                return Flow::Recompile(state);
            }
        }

        match &block.exit {
            Exit::Jump(l) => Flow::Jump(l),
            Exit::Random(r, d, l, u) => Flow::Jump(match rand::random::<u32>() & 0b11 {
                0b00 => r,
                0b01 => d,
                0b10 => l,
                0b11 => u,
                _ => unreachable!(),
            }),
            Exit::Branch(t, e) => Flow::Jump(if self.pop().into_i32() != 0 { t } else { e }),
            Exit::End => Flow::End,
        }
    }

    /// Interprets an instruction and returns the state to recompile the program
    /// from. Returns `None` if the program should not be recompiled.
    fn interpret_instruction(&mut self, instruction: &'b Instruction) -> Option<&'b State> {
        match instruction {
            Instruction::Push(v) => self.push(*v),
            Instruction::Unary(o) => {
                let rhs = self.pop();
                self.push(o.eval(rhs));
            }
            Instruction::Binary(o) => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(o.eval(lhs, rhs));
            }
            Instruction::Divide(o) => {
                let rhs = self.pop();
                let lhs = self.pop();
                if rhs.into_i32() != 0 {
                    self.push(BinOp::from(*o).eval(lhs, rhs));
                } else {
                    print!("What do you want {}{o}0 to be? ", lhs.into_i32());
                    self.input_int();
                }
            }
            Instruction::Duplicate => self.push(self.peek()),
            Instruction::Swap => {
                let top = self.pop();
                let under = self.pop();
                self.push(top);
                self.push(under);
            }
            Instruction::Pop => {
                self.pop();
            }
            Instruction::OutputInt => print!("{} ", self.pop().into_i32()),
            Instruction::OutputChar => print!("{}", self.pop().into_char_lossy()),
            Instruction::Get => {
                let y = self.pop().into_i32();
                let x = self.pop().into_i32();
                let value = match (usize::try_from(x), usize::try_from(y)) {
                    (Ok(x), Ok(y)) => self.playfield.get(x, y).unwrap_or_default(),
                    _ => Value::default(),
                };
                self.push(value);
            }
            Instruction::Put(s) => {
                let y = self.pop().into_i32();
                let x = self.pop().into_i32();
                let value = self.pop();
                if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
                    if let Some(previous_value) = self.playfield.put(x, y, value) {
                        if previous_value.into_i32() != value.into_i32() {
                            return Some(s);
                        }
                    }
                }
            }
            Instruction::InputInt => self.input_int(),
            Instruction::InputChar => {
                if self.input_chars.is_empty() {
                    self.input_chars.extend(read_line().chars());
                }

                let value = self
                    .input_chars
                    .pop_front()
                    .map_or(Value::from(-1), Into::into);
                self.push(value);
            }
            Instruction::Print(s) => print!("{s}"),
        }

        None
    }

    /// Parses an integer from a line of user input and pushes it to the stack.
    fn input_int(&mut self) {
        self.push(read_line().trim().parse().unwrap_or(-1).into());
    }

    /// Returns the top value of the stack.
    fn peek(&self) -> Value {
        self.stack.last().copied().unwrap_or_default()
    }

    /// Pushes a value to the stack.
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Pops a value from the stack.
    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or_default()
    }
}

/// A control flow from a block.
enum Flow<'a> {
    /// A jump to another label.
    Jump(&'a Label),

    /// A recompilation at a state.
    Recompile(&'a State),

    /// A program ending.
    End,
}

/// Reads a line of user input.
fn read_line() -> String {
    flush_output();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("reading from stdin should not fail");
    line
}

/// Flushes the standard output stream.
fn flush_output() {
    io::stdout()
        .flush()
        .expect("flushing stdout should not fail");
}
