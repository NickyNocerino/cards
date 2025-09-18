use crate::effects::effect::Effect;

#[derive(Debug)]
pub enum Face{
    Creature {
        fast: bool,
        cost: usize,
        offense: usize,
        defense: usize,
        energy: usize,
    },
    Spell {
        fast: bool,
        cost: usize,
        effects: &'static [Effect],
    },
}

