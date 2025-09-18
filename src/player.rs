use crate::card::Card;
use crate::deck::Deck;

pub struct Player{
    deck: Deck,
    hand: Vec<Card>,
    trash: Vec<Card>,
    field: Vec<Card>,
}