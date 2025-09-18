use crate::card::Card;
use crate::deck::Deck;

pub struct Player{
    deck: Deck,
    hand: Vec<Card>,
    discard: Vec<Card>,
}