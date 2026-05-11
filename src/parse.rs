use crate::{
    cfg::{BasicBlock, Cfg, Label, Terminator},
    playfield::Playfield,
};

/// Parses a [`Cfg`] from a [`Playfield`].
pub fn parse_playfield(playfield: &Playfield) -> Cfg {
    let _: &Playfield = playfield;
    let mut cfg = Cfg::new();
    cfg.insert_basic_block(
        Label::Main,
        BasicBlock {
            terminator: Terminator::Jump(Label::Main),
        },
    );

    cfg
}
