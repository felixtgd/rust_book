use crate::deck::Deck;
use crate::hand::Hand;
use std::cmp::Ordering;

pub struct Game {
    deck: Deck,
    player: Hand,
    dealer: Hand,
    state: GameState,
}

pub enum GameState {
    PlayerTurn,
    DealerTurn,
    Resolve,
    GameOver { winner: Winner },
}

pub enum Winner {
    Player,
    Dealer,
    Tie,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: Deck::new(),
            player: Hand::new(),
            dealer: Hand::new(),
            state: GameState::PlayerTurn,
        }
    }
}

// Public API
impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        // Deal initial cards
        let n: u32 = 2;
        self.player.deal_cards(&mut self.deck, n);
        self.dealer.deal_cards(&mut self.deck, n);

        // Check for blackjack
        self.check_blackjack();
    }

    pub fn player_action(&mut self, action: String) {
        if !self.players_turn() {
            return;
        }

        match action.trim() {
            "h" => {
                self.player_hit();
            }
            "s" => {
                self.player_stand();
            }
            _ => (),
        }
    }

    pub fn dealer_action(&mut self) {
        if !self.dealers_turn() {
            return;
        }

        if self.dealer.score < 17 {
            self.dealer_hit();
        } else {
            self.dealer_stand();
        }
    }

    pub fn select_winner(&mut self) {
        let winner = match self.player.score.cmp(&self.dealer.score) {
            Ordering::Less => Winner::Dealer,
            Ordering::Greater => Winner::Player,
            Ordering::Equal => Winner::Tie,
        };
        self.game_over(winner);
    }
}

// Private implementation
impl Game {
    fn player_hit(&mut self) {
        self.player.hit(&mut self.deck);

        if self.player.is_bust() {
            self.game_over(Winner::Dealer);
        }
    }

    fn player_stand(&mut self) {
        self.transition_to(GameState::DealerTurn);
    }

    fn dealer_hit(&mut self) {
        self.dealer.hit(&mut self.deck);

        if self.dealer.is_bust() {
            self.game_over(Winner::Player);
        }
    }

    fn dealer_stand(&mut self) {
        self.transition_to(GameState::Resolve);
    }

    fn check_blackjack(&mut self) {
        let player_blackjack: bool = self.player.has_blackjack();
        let dealer_blackjack: bool = self.dealer.has_blackjack();

        match (player_blackjack, dealer_blackjack) {
            (true, true) => self.game_over(Winner::Tie),
            (true, false) => self.game_over(Winner::Player),
            (false, true) => self.game_over(Winner::Dealer),
            (false, false) => (), // No blackjack, continue normal play
        }
    }

    fn transition_to(&mut self, new_state: GameState) {
        self.state = new_state;
    }

    fn game_over(&mut self, winner: Winner) {
        self.transition_to(GameState::GameOver { winner });
    }
}

// Helper methods
impl Game {
    pub fn player_hand(&self) -> &Hand {
        &self.player
    }

    pub fn dealer_hand(&self) -> &Hand {
        &self.dealer
    }

    pub fn players_turn(&self) -> bool {
        matches!(self.state, GameState::PlayerTurn)
    }

    pub fn dealers_turn(&self) -> bool {
        matches!(self.state, GameState::DealerTurn)
    }

    pub fn is_over(&self) -> bool {
        matches!(self.state, GameState::GameOver { .. })
    }
}
