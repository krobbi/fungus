use std::collections::{HashMap, HashSet};

use crate::pointer::Label;

use super::{Exit, Program};

/// Optimize a program.
pub fn optimize_program(program: &mut Program) {
    while thread_jumps(program) {}
}

/// Redirect exits that target jumps to the jump's target.
fn thread_jumps(program: &mut Program) -> bool {
    /// Redirect a label if it exists in a map of redirects and set a flag.
    fn redirect_label(label: &mut Label, redirects: &HashMap<Label, Label>, redirected: &mut bool) {
        if redirects.contains_key(label) {
            *label = *redirects.get(label).unwrap();
            *redirected = true;
        }
    }

    let mut redirects = HashMap::new();

    for (&label, block) in &program.blocks {
        if let Exit::Jump(target) = block.exit {
            if target != label && block.instructions.is_empty() {
                redirects.insert(label, target);
            }
        }
    }

    if redirects.is_empty() {
        return false;
    }

    let mut redirected = false;

    for exit in program.blocks.values_mut().map(|block| &mut block.exit) {
        match exit {
            Exit::Jump(label) => redirect_label(label, &redirects, &mut redirected),
            Exit::Random(right, down, left, up) => {
                redirect_label(right, &redirects, &mut redirected);
                redirect_label(down, &redirects, &mut redirected);
                redirect_label(left, &redirects, &mut redirected);
                redirect_label(up, &redirects, &mut redirected);
            }
            Exit::If { non_zero, zero } => {
                redirect_label(non_zero, &redirects, &mut redirected);
                redirect_label(zero, &redirects, &mut redirected);
            }
            Exit::End => (),
        }
    }

    if redirected {
        remove_unreachable_blocks(program);
        true
    } else {
        false
    }
}

/// Remove unreachable basic blocks.
fn remove_unreachable_blocks(program: &mut Program) {
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
    }
}
