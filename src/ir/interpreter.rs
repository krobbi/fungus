use std::io::{self, Write};

use crate::pointer::Label;

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
}

impl Interpreter<'_> {
    /// Create a new interpreter from a program.
    fn new(program: &Program) -> Interpreter {
        Interpreter {
            program,
            stack: vec![],
        }
    }

    /// Run the interpreter.
    fn run(&mut self) {
        let mut label = Label::default();

        while let Some(next_label) = self.interpret_label(label) {
            label = next_label;
        }
    }

    /// Interpret a label and get the optional next label.
    fn interpret_label(&mut self, label: Label) -> Option<Label> {
        let block = self.program.blocks.get(&label).unwrap();

        for instruction in &block.instructions {
            self.interpret_instruction(instruction);
        }

        match block.exit {
            Exit::Jump(label) => Some(label),
            Exit::Random(right, down, left, up) => Some(match rand::random::<u8>() & 0b11 {
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
                    self.divide_by_zero();
                }
            }
            Instruction::Modulo => {
                let r = self.pop();
                let l = self.pop();

                if r != 0 {
                    self.push(l % r);
                } else {
                    self.divide_by_zero();
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
                io::stdout().flush().unwrap();
            }
            Instruction::OutputCharacter => {
                let value = self.pop();

                #[allow(clippy::cast_sign_loss)]
                let value = char::from_u32(value as u32).unwrap_or(char::REPLACEMENT_CHARACTER);

                print!("{value}");
                io::stdout().flush().unwrap();
            }
            Instruction::InputInteger => {
                self.input_integer();
            }
            Instruction::InputCharacter => {
                // TODO: Implement character input.
                self.push('\n' as i32);
            }
            &Instruction::Push(value) => {
                self.push(value);
            }
        }
    }

    /// Handle a division by zero.
    fn divide_by_zero(&mut self) {
        self.input_integer();
    }

    /// Input an integer and push it to the stack.
    fn input_integer(&mut self) {
        // TODO: Implement integer input.
        self.push(0);
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
