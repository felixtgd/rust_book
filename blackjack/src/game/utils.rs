#[derive(Clone)]
pub enum GameStage {
    PlayerTurn,
    DealerTurn,
    Resolve,
    GameOver { winner: Winner },
}

#[derive(Clone)]
pub enum Winner {
    Player,
    Dealer,
    Tie,
}

pub struct PublicState {
    pub dealer_cards: Vec<String>,
    pub player_cards: Vec<String>,
    pub dealer_score: u8,
    pub player_score: u8,
    pub stage: GameStage,
}
