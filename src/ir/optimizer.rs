use std::collections::{HashMap, HashSet};

use crate::pointer::Label;

use super::{Exit, Program};

/// Optimize a program.
pub fn optimize_program(program: &mut Program) {
    loop {
        let mut optimized = false;
        merge_blocks(program, &mut optimized);

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
