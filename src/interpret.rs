use std::{
    collections::VecDeque,
    io::{self, Write as _},
    thread,
    time::Duration,
};

use crate::{
    cfg::{BasicBlock, BinOp, Cfg, Expr, Instruction, Label, Terminator},
    playfield::Playfield,
    value::Value,
};

/// Interprets a [`Cfg`] with a [`Playfield`].
pub fn interpret_cfg(cfg: &Cfg, playfield: &Playfield) {
    let mut interpreter = Interpreter::new(playfield);
    interpreter.interpret_cfg(cfg);
}

/// A structure which interprets a [`Cfg`].
struct Interpreter<'ply> {
    /// The [`Playfield`].
    playfield: &'ply Playfield,

    /// The stack of [`Value`]s.
    stack: Vec<Value>,

    /// The buffer of input [`char`]s.
    input_chars: VecDeque<char>,
}

impl<'ply> Interpreter<'ply> {
    /// Creates a new `Interpreter` from a [`Playfield`].
    const fn new(playfield: &'ply Playfield) -> Self {
        Self {
            playfield,
            stack: Vec::new(),
            input_chars: VecDeque::new(),
        }
    }

    /// Interprets a [`Cfg`].
    fn interpret_cfg(&mut self, cfg: &Cfg) {
        let mut label = Label::Main;

        loop {
            let basic_block = cfg.basic_block(label);

            match self.interpret_basic_block(basic_block) {
                Flow::Halt => break,
                Flow::InfiniteLoop => infinite_loop(),
                Flow::Jump(next_label) => label = next_label,
            }
        }

        flush_stdout();
    }

    /// Interprets a [`BasicBlock`] and returns its [`Flow`].
    fn interpret_basic_block(&mut self, basic_block: &BasicBlock) -> Flow {
        for instruction in &basic_block.instructions {
            self.interpret_instruction(instruction);
        }

        self.interpret_terminator(&basic_block.terminator)
    }

    /// Interprets an [`Instruction`].
    fn interpret_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Push(expr) => {
                let value = self.eval_expr(expr);
                self.push(value);
            }
            Instruction::Pop => {
                self.pop();
            }
            Instruction::Duplicate => {
                let value = self.pop();
                self.push(value);
                self.push(value);
            }
            Instruction::Swap => {
                let top = self.pop();
                let under = self.pop();
                self.push(top);
                self.push(under);
            }
            Instruction::Unary(op) => {
                let rhs = self.pop();
                self.push(op.eval(rhs));
            }
            Instruction::Binary(op) => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(self.eval_binary(*op, lhs, rhs));
            }
            Instruction::Assoc(op) => {
                let rhs = self.pop();
                let lhs = self.pop();
                self.push(op.eval(lhs, rhs));
            }
            Instruction::OutputInt => {
                let value = self.pop();
                print!("{value} ");
            }
            Instruction::OutputChar => {
                let value = self.pop();
                print!("{}", value.to_char_lossy());
            }
            Instruction::Print(string) => {
                print!("{string}");
            }
        }
    }

    /// Interprets a [`Terminator`] and returns its [`Flow`].
    fn interpret_terminator(&mut self, terminator: &Terminator) -> Flow {
        match terminator {
            Terminator::Halt => Flow::Halt,
            Terminator::InfiniteLoop => Flow::InfiniteLoop,
            Terminator::Jump(label) => Flow::Jump(*label),
            Terminator::Branch(then_label, else_label) => {
                let condition = self.pop();

                let label = if condition.is_non_zero() {
                    *then_label
                } else {
                    *else_label
                };

                Flow::Jump(label)
            }
            Terminator::Random(right_label, down_label, left_label, up_label) => {
                let label = match fastrand::u64(..) & 0b11 {
                    0b00 => *right_label,
                    0b01 => *down_label,
                    0b10 => *left_label,
                    0b11 => *up_label,
                    _ => unreachable!("random number should be masked"),
                };

                Flow::Jump(label)
            }
            Terminator::Put(_) => todo!("interpreting put terminator"),
        }
    }

    /// Evaluates and returns an [`Expr`]'s result [`Value`].
    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Const(value) => *value,
            Expr::InputInt => read_line().trim().parse().map_or(Value(-1), Value),
            Expr::InputChar => {
                if self.input_chars.is_empty() {
                    self.input_chars.extend(read_line().chars());
                }

                self.input_chars.pop_front().map_or(Value(-1), Into::into)
            }
            Expr::Unary(op, rhs) => {
                let rhs = self.eval_expr(rhs);
                op.eval(rhs)
            }
            Expr::Binary(op, lhs, rhs) => {
                let lhs = self.eval_expr(lhs);
                let rhs = self.eval_expr(rhs);
                self.eval_binary(*op, lhs, rhs)
            }
            Expr::Assoc(op, terms) => {
                let mut value = op.identity();

                for term in terms {
                    let term = self.eval_expr(term);
                    value = op.eval(value, term);
                }

                value
            }
            Expr::Sequence(prefix, expr) => {
                for prefix_expr in prefix {
                    self.eval_expr(prefix_expr);
                }

                self.eval_expr(expr)
            }
        }
    }

    /// Evaluates and returns a [`BinOp`]'s result [`Value`] from operand
    /// [`Value`]s.
    fn eval_binary(&self, op: BinOp, lhs: Value, rhs: Value) -> Value {
        let _: &Self = self;

        if let Some(value) = op.eval_const(lhs, rhs) {
            return value;
        }

        debug_assert_eq!(op, BinOp::Get, "unknown non-constant binary operator");

        let (Ok(x), Ok(y)) = (lhs.0.try_into(), rhs.0.try_into()) else {
            return Value::SPACE;
        };

        self.playfield.value(x, y).unwrap_or(Value::SPACE)
    }

    /// Pushes a [`Value`] to the stack.
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Pops and returns a [`Value`] from the stack.
    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value(0))
    }
}

/// Control flow from a [`Terminator`].
enum Flow {
    /// Halt execution.
    Halt,

    /// Infinite loop without side effects.
    InfiniteLoop,

    /// Unconditionally jump to a [`Label`].
    Jump(Label),
}

/// Reads and returns a line of user input.
fn read_line() -> String {
    flush_stdout();
    let mut line = String::new();

    // NOTE: This will panic if user input is not UTF-8. Ideally, a panic should
    // never occur because of user error. Consider using a buffer of bytes to
    // emulate the C standard library input of the original Befunge interpreter.
    io::stdin()
        .read_line(&mut line)
        .expect("reading from stdin should not fail");

    line
}

/// Enters a cold infinite loop.
#[cold]
fn infinite_loop() -> ! {
    const SLEEP_DURATION: Duration = Duration::from_secs(1);

    flush_stdout();

    loop {
        thread::sleep(SLEEP_DURATION);
    }
}

/// Flushes the standard output stream.
fn flush_stdout() {
    io::stdout()
        .flush()
        .expect("flushing stdout should not fail");
}
