use crate::card::Card;
use crate::face::*;
use crate::effects::effect::*;

pub static EXAMPLE_CARD: Card = Card{
    front: Some(Face::Creature{
        fast: false,
        cost: 0,
        offense: 1,
        defense: 5,
        energy: 2,
    }),
    back: Some(Face::Spell{
        fast: true,
        cost: 2,
        effects: &[Effect::DrawNCards(1)]
    })
};