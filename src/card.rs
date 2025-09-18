use crate::effects::effect::Effect;

pub struct Card {
    fast: bool,
    effects: Vec<Effect>,
}