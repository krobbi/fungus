use std::collections::{BTreeSet, HashMap, HashSet};

/// A directed graph of reachable positions in a program.
pub struct FlowGraph {
    /// The reachable positions and the positions they flow into.
    connections: HashMap<(usize, usize), HashSet<(usize, usize)>>,
}

impl FlowGraph {
    /// Creates a new flow graph from a root position.
    pub fn new(root: (usize, usize)) -> Self {
        let mut connections = HashMap::new();
        connections.insert(root, HashSet::new());
        Self { connections }
    }

    /// Inserts a new connection between a source position and a target
    /// position.
    pub fn insert_connection(&mut self, source: (usize, usize), target: (usize, usize)) {
        self.connections
            .get_mut(&source)
            .expect("source position should exist in flow map")
            .insert(target);
        self.connections.entry(target).or_default();
    }

    /// Returns whether a target position can be reached from a source position.
    #[expect(dead_code, reason = "will be used in a future version")]
    pub fn can_reach(&self, source: (usize, usize), target: (usize, usize)) -> bool {
        assert!(self.connections.contains_key(&source));
        if !self.connections.contains_key(&target) {
            return false;
        }

        let mut pending_positions = BTreeSet::new();
        let mut checked_positions = HashSet::new();
        pending_positions.insert(source);

        while let Some(position) = pending_positions.pop_first() {
            if checked_positions.contains(&position) {
                continue;
            }
            if position == target {
                return true;
            }

            checked_positions.insert(position);
            pending_positions.extend(&self.connections[&position]);
        }

        false
    }
}
