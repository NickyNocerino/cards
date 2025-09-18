#[derive(Debug)]
pub enum Effect{
    AddAnEnergy,
    DrawACard,
    DiscardACardAtRandom,
    DiscardACardOfYourChoice,
    FlipThisCArd,
    WinTheGame,
    LoseTheGame,
}