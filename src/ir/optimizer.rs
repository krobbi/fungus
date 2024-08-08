use std::collections::{HashMap, HashSet};

use crate::pointer::Label;

use super::{Exit, Instruction, Program};

/// Optimize a program.
pub fn optimize_program(program: &mut Program) {
    loop {
        let mut optimized = false;
        merge_blocks(program, &mut optimized);
        replace_instructions(program, &mut optimized);
        replace_ifs(program, &mut optimized);
        remove_unreachable_blocks(program, &mut optimized);

        if !optimized {
            break;
        }
    }
}

/// Merge basic blocks with one entry point from a jump into their predecessor.
fn merge_blocks(program: &mut Program, optimized: &mut bool) {
    let mut successor_predecessors = HashMap::new();
    let mut invalid_successors = HashSet::new();
    invalid_successors.insert(Label::default());

    for (&predecessor, block) in &program.blocks {
        if let Exit::Jump(successor) = block.exit {
            if successor != predecessor && !successor_predecessors.contains_key(&successor) {
                successor_predecessors.insert(successor, predecessor);
            } else {
                invalid_successors.insert(successor);
            }
        } else {
            for successor in block.exit.exit_labels() {
                invalid_successors.insert(successor);
            }
        }
    }

    for (successor, predecessor) in successor_predecessors {
        if invalid_successors.contains(&successor) || !program.blocks.contains_key(&predecessor) {
            continue;
        }

        let mut successor = program.blocks.remove(&successor).unwrap();
        let predecessor = program.blocks.get_mut(&predecessor).unwrap();
        predecessor.instructions.append(&mut successor.instructions);
        predecessor.exit = successor.exit;
        *optimized = true;
    }
}

/// Replace peepholes of instructions with more optimal instructions.
fn replace_instructions(program: &mut Program, optimized: &mut bool) {
    for instructions in program
        .blocks
        .values_mut()
        .map(|block| &mut block.instructions)
    {
        for length in 2..=3 {
            let mut index = 0;

            loop {
                let range = index..index + length;

                let Some(peephole) = instructions.get(range.clone()) else {
                    break;
                };

                if let Some(peephole) = optimize_peephole(peephole) {
                    instructions.splice(range, peephole);
                    index = index.saturating_sub(length - 1);
                    *optimized = true;
                } else {
                    index += 1;
                }
            }
        }
    }
}

/// Optimize ifs by replacing them with jumps or swapping their branches.
fn replace_ifs(program: &mut Program, optimized: &mut bool) {
    for block in program.blocks.values_mut() {
        let Exit::If { non_zero, zero } = block.exit else {
            continue;
        };

        if non_zero == zero {
            block.instructions.push(Instruction::Pop);
            block.exit = Exit::Jump(non_zero);
            *optimized = true;
            continue;
        }

        match block.instructions.last() {
            Some(Instruction::Not) => {
                block.instructions.pop();
                block.exit = Exit::If {
                    non_zero: zero,
                    zero: non_zero,
                };
                *optimized = true;
            }
            Some(&Instruction::Push(value)) => {
                block.instructions.pop();
                block.exit = Exit::Jump(if value != 0 { non_zero } else { zero });
                *optimized = true;
            }
            _ => (),
        }
    }
}

/// Remove basic blocks that can never be reached.
fn remove_unreachable_blocks(program: &mut Program, optimized: &mut bool) {
    let mut pending_labels = vec![Label::default()];
    let mut reachable_labels = HashSet::new();

    while let Some(label) = pending_labels.pop() {
        if reachable_labels.contains(&label) {
            continue;
        }

        pending_labels.append(&mut program.blocks.get(&label).unwrap().exit.exit_labels());
        reachable_labels.insert(label);
    }

    let mut unreachable_labels = HashSet::new();

    for &label in program.blocks.keys() {
        if !reachable_labels.contains(&label) {
            unreachable_labels.insert(label);
        }
    }

    for label in unreachable_labels {
        program.blocks.remove(&label);
        *optimized = true;
    }
}

/// Get optional, more optimal instructions from a peephole of instructions.
fn optimize_peephole(peephole: &[Instruction]) -> Option<Vec<Instruction>> {
    #[allow(clippy::enum_glob_use)]
    use Instruction::*;

    #[allow(clippy::match_same_arms)]
    match peephole {
        [Not, Pop] => Some(vec![Pop]),
        [Duplicate, Greater] => Some(vec![Pop, Push(0)]),
        [Duplicate, Swap] => Some(vec![Duplicate]),
        [Duplicate, Pop] => Some(vec![]),
        [Swap, Swap] => Some(vec![]),
        [Push(0), Add] => Some(vec![]),
        [Push(0), Subtract] => Some(vec![]),
        [Push(0), Multiply] => Some(vec![Pop, Push(0)]),
        [Push(0), Divide] => Some(vec![Pop, InputInteger]),
        [Push(0), Modulo] => Some(vec![Pop, InputInteger]),
        [Push(1), Multiply] => Some(vec![]),
        [Push(1), Divide] => Some(vec![]),
        [Push(1), Modulo] => Some(vec![Pop, Push(0)]),
        &[Push(value), Not] => Some(vec![Push(i32::from(value == 0))]),
        &[Push(value), Duplicate] => Some(vec![Push(value), Push(value)]),
        [Push(_), Pop] => Some(vec![]),
        [Not, Not, Not] => Some(vec![Not]),
        [Greater, Not, Not] => Some(vec![Greater]),
        &[InputInteger, Push(b), Swap] => Some(vec![Push(b), InputInteger]),
        [InputCharacter, Push(0), Greater] => Some(vec![InputCharacter, Not, Not]),
        [InputCharacter, Push(..=-1), Greater] => Some(vec![InputCharacter, Pop, Push(1)]),
        [InputCharacter, Push(0x10_ffff..), Greater] => Some(vec![InputCharacter, Pop, Push(0)]),
        &[InputCharacter, Push(b), Swap] => Some(vec![Push(b), InputCharacter]),
        [Push(0), InputInteger, Add] => Some(vec![InputInteger]),
        [Push(0), InputInteger, Multiply] => Some(vec![InputInteger, Pop, Push(0)]),
        [Push(0), InputCharacter, Add] => Some(vec![InputCharacter]),
        [Push(0), InputCharacter, Multiply] => Some(vec![InputCharacter, Pop, Push(0)]),
        [Push(1), InputInteger, Multiply] => Some(vec![InputInteger]),
        [Push(1), InputCharacter, Multiply] => Some(vec![InputCharacter]),
        [Push(1), InputCharacter, Greater] => Some(vec![InputCharacter, Not]),
        [Push(..=0), InputCharacter, Greater] => Some(vec![InputCharacter, Pop, Push(0)]),
        [Push(0x11_0000..), InputCharacter, Greater] => Some(vec![InputCharacter, Pop, Push(1)]),
        &[Push(a), InputInteger, Swap] => Some(vec![InputInteger, Push(a)]),
        &[Push(a), InputCharacter, Swap] => Some(vec![InputCharacter, Push(a)]),
        [Push(l), Push(r), Add] => Some(vec![Push(l + r)]),
        [Push(l), Push(r), Subtract] => Some(vec![Push(l - r)]),
        [Push(l), Push(r), Multiply] => Some(vec![Push(l * r)]),
        [Push(l), Push(r @ (..=-1 | 1..)), Divide] => Some(vec![Push(l / r)]),
        [Push(l), Push(r @ (..=-1 | 1..)), Modulo] => Some(vec![Push(l % r)]),
        [Push(l), Push(r), Greater] => Some(vec![Push(i32::from(l > r))]),
        &[Push(a), Push(b), Swap] => Some(vec![Push(b), Push(a)]),
        _ => None,
    }
}
