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

#[derive(Debug)]
pub struct Creature {
    pub fast: bool,
    pub cost: usize,
    pub offense: usize,
    pub defense: usize,
    pub energy: usize,
}

#[derive(Debug)]
pub struct Spell{
    pub fast: bool,
    pub cost: usize,
    pub effects: &'static [Effect],

}