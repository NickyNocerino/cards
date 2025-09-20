#[derive(Debug)]
pub enum Effect{
    AddNEnergy(usize),
    DrawNCards(usize),
    DiscardNCardsAtRandom(usize),
    DiscardNCardsOfYourChoice(usize),
    FlipThisCArd,
    WinTheGame,
    LoseTheGame,
    StealResolvingCreature
}