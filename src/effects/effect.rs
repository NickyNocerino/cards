pub enum Effect{
    DrawACard,
    DiscardACard,
    DrawNCards(usize),
    DiscardNCards(usize),

}