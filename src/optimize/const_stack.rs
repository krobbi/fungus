use crate::{cfg::Instruction, value::Value};

/// A stack of known constant or unknown [`Value`]s.
pub struct ConstStack {
    /// The [`Value`]s.
    values: Vec<Option<Value>>,
}

impl ConstStack {
    /// Creates a new empty `ConstStack`.
    pub const fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Returns the [`Value`] at the top of the `ConstStack`. This function
    /// returns [`None`] if the `ConstStack` has no known top value.
    pub fn peek(&self) -> Option<Value> {
        self.values.last().copied().unwrap_or(None)
    }

    /// Evaluates the effect of a slice of [`Instruction`]'s on the
    /// `ConstStack`.
    pub fn eval_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.eval_instruction(instruction);
        }
    }

    /// Evaluates an [`Instruction`]'s effect on the `ConstStack`.
    fn eval_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Push(expr) => self.push(expr.eval_const()),
            Instruction::Pop | Instruction::OutputInt | Instruction::OutputChar => {
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
                let value = rhs.map(|v| op.eval(v));
                self.push(value);
            }
            Instruction::Binary(op) => {
                let rhs = self.pop();
                let lhs = self.pop();

                let value = if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    op.eval_const(lhs, rhs)
                } else {
                    None
                };

                self.push(value);
            }
            Instruction::Assoc(op) => {
                let rhs = self.pop();
                let lhs = self.pop();

                let value = if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    Some(op.eval(lhs, rhs))
                } else {
                    None
                };

                self.push(value);
            }
            Instruction::Print(_) => (),
            Instruction::PrintStack => {
                while let Some(value) = self.pop() {
                    if value == Value(0) {
                        // The printed stack was constant and terminated. Push
                        // the terminator back to the stack.
                        self.push(Some(Value(0)));
                        return;
                    }
                }

                // A non-constant value was printed. The state of the stack is
                // unknown, but there must be an implicit or explicit terminator
                // on top of the stack.
                self.values.clear();
                self.push(Some(Value(0)));
            }
        }
    }

    /// Pushes a known constant or unknown [`Value`] to the `ConstStack`.
    fn push(&mut self, value: Option<Value>) {
        self.values.push(value);
    }

    /// Pops a [`Value`] from the `ConstStack`. This function returns [`None`]
    /// if the [`Value`] is not a known constant.
    fn pop(&mut self) -> Option<Value> {
        self.values.pop().unwrap_or(None)
    }
}
