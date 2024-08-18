use std::{
    collections::VecDeque,
    io::{self, Write},
};

use crate::{playfield::Playfield, pointer::Label};

use super::{Exit, Instruction, Program};

/// Interpret a program.
pub fn interpret_program(program: &Program) {
    let mut interpreter = Interpreter::new(program);
    interpreter.run();
}

/// A program and its runtime state.
struct Interpreter<'a> {
    /// The program.
    program: &'a Program,

    /// The stack.
    stack: Vec<i32>,

    /// The character input buffer.
    input_chars: VecDeque<char>,
}

impl Interpreter<'_> {
    /// Create a new interpreter from a program.
    fn new(program: &Program) -> Interpreter {
        Interpreter {
            program,
            stack: vec![],
            input_chars: VecDeque::new(),
        }
    }

    /// Run the interpreter.
    fn run(&mut self) {
        let mut label = Label::default();

        while let Some(next_label) = self.interpret_label(label) {
            label = next_label;
        }

        flush_output();
    }

    /// Interpret a label and get the optional next label.
    fn interpret_label(&mut self, label: Label) -> Option<Label> {
        let block = self.program.blocks.get(&label).unwrap();

        for instruction in &block.instructions {
            self.interpret_instruction(instruction);
        }

        match block.exit {
            Exit::Jump(label) => Some(label),
            Exit::Random(right, down, left, up) => Some(match rand::random::<u32>() & 0b11 {
                0b00 => right,
                0b01 => down,
                0b10 => left,
                0b11 => up,
                _ => unreachable!(),
            }),
            Exit::If { non_zero, zero } => Some(if self.pop() != 0 { non_zero } else { zero }),
            Exit::End => None,
        }
    }

    /// Interpret an instruction.
    fn interpret_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Add => {
                let r = self.pop();
                let l = self.pop();
                self.push(l + r);
            }
            Instruction::Subtract => {
                let r = self.pop();
                let l = self.pop();
                self.push(l - r);
            }
            Instruction::Multiply => {
                let r = self.pop();
                let l = self.pop();
                self.push(l * r);
            }
            Instruction::Divide => {
                let r = self.pop();
                let l = self.pop();

                if r != 0 {
                    self.push(l / r);
                } else {
                    self.divide_by_zero(l, '/');
                }
            }
            Instruction::Modulo => {
                let r = self.pop();
                let l = self.pop();

                if r != 0 {
                    self.push(l % r);
                } else {
                    self.divide_by_zero(l, '%');
                }
            }
            Instruction::Not => {
                let value = self.pop();
                self.push(i32::from(value == 0));
            }
            Instruction::Greater => {
                let r = self.pop();
                let l = self.pop();
                self.push(i32::from(l > r));
            }
            Instruction::Duplicate => {
                let value = self.pop();
                self.push(value);
                self.push(value);
            }
            Instruction::Swap => {
                let b = self.pop();
                let a = self.pop();
                self.push(b);
                self.push(a);
            }
            Instruction::Pop => {
                self.pop();
            }
            Instruction::OutputInteger => {
                let value = self.pop();
                print!("{value} ");
            }
            Instruction::OutputCharacter => {
                let value = self.pop();
                let value = Playfield::value_to_char(value);
                print!("{value}");
            }
            Instruction::Get => {
                let y = self.pop();
                let x = self.pop();
                let value = self.program.playfield.value(x, y);
                self.push(value);
            }
            Instruction::InputInteger => {
                self.input_integer();
            }
            Instruction::InputCharacter => {
                if self.input_chars.is_empty() {
                    self.input_chars.append(&mut read_line().chars().collect());
                }

                let value = match self.input_chars.pop_front() {
                    None => -1,
                    Some(value) => Playfield::char_to_value(value),
                };

                self.push(value);
            }
            &Instruction::Push(value) => {
                self.push(value);
            }
        }
    }

    /// Handle a division by zero.
    fn divide_by_zero(&mut self, l: i32, operator: char) {
        print!("What do you want the result of {l} {operator} 0 to be? ");
        self.input_integer();
    }

    /// Input an integer and push it to the stack.
    fn input_integer(&mut self) {
        let value = read_line().trim().parse().unwrap_or(-1);
        self.push(value);
    }

    /// Push a value to the stack.
    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    /// Pop a value from the stack.
    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(0)
    }
}

/// Read a line of input.
fn read_line() -> String {
    flush_output();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

/// Flush the standard output stream.
fn flush_output() {
    io::stdout().flush().unwrap();
}
