use crate::{
    cfg::{BasicBlock, Cfg, Label, Terminator},
    playfield::Playfield,
    state::State,
};

/// Parses a [`Cfg`] from a [`Playfield`].
pub fn parse_playfield(playfield: &Playfield) -> Cfg {
    parse_playfield_at(playfield, State::default())
}

/// Parses a [`Cfg`] from a [`Playfield`] at a main [`State`].
fn parse_playfield_at(playfield: &Playfield, main_state: State) -> Cfg {
    let mut cfg = Cfg::new();
    cfg.insert_basic_block(
        Label::Main,
        BasicBlock {
            terminator: Terminator::Jump(Label::State(main_state)),
        },
    );

    let mut pending_states = vec![main_state];

    while let Some(state) = pending_states.pop() {
        let label = Label::State(state);

        if cfg.contains_label(label) {
            continue;
        }

        let basic_block = parse_basic_block(playfield, state);

        for pending_label in basic_block.terminator.labels() {
            let Label::State(pending_state) = pending_label else {
                unreachable!("all pending labels should point to states");
            };

            pending_states.push(pending_state);
        }

        cfg.insert_basic_block(label, basic_block);
    }

    cfg
}

/// Parses a [`BasicBlock`] from a [`Playfield`] and a [`State`].
const fn parse_basic_block(playfield: &Playfield, state: State) -> BasicBlock {
    let _: &Playfield = playfield;

    BasicBlock {
        terminator: Terminator::Jump(Label::State(state)),
    }
}
