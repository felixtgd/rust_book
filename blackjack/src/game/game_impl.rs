use crate::game::deck::Deck;
use crate::game::hand::Hand;
use crate::game::utils::{GameStage, PublicState, Winner};
use std::cmp::Ordering;

pub struct Game {
    deck: Deck,
    player: Hand,
    dealer: Hand,
    stage: GameStage,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: Deck::new(),
            player: Hand::new(),
            dealer: Hand::new(),
            stage: GameStage::PlayerTurn,
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
        let n: u8 = 2;
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

        if self.dealer.score() < 17 {
            self.dealer_hit();
        } else {
            self.dealer_stand();
        }
    }

    pub fn select_winner(&mut self) {
        let winner = match self.player.score().cmp(&self.dealer.score()) {
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
        self.player.stand();
        self.transition_to(GameStage::DealerTurn);
    }

    fn dealer_hit(&mut self) {
        self.dealer.hit(&mut self.deck);

        if self.dealer.is_bust() {
            self.game_over(Winner::Player);
        }
    }

    fn dealer_stand(&mut self) {
        self.dealer.stand();
        self.transition_to(GameStage::Resolve);
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

    fn transition_to(&mut self, new_stage: GameStage) {
        self.stage = new_stage;
    }

    fn game_over(&mut self, winner: Winner) {
        self.transition_to(GameStage::GameOver { winner });
    }
}

// Helper methods
impl Game {
    pub fn player_cards(&self) -> &Vec<String> {
        self.player.cards()
    }

    pub fn dealer_cards(&self) -> &Vec<String> {
        self.dealer.cards()
    }

    pub fn players_turn(&self) -> bool {
        matches!(self.stage, GameStage::PlayerTurn)
    }

    pub fn dealers_turn(&self) -> bool {
        matches!(self.stage, GameStage::DealerTurn)
    }

    pub fn is_over(&self) -> bool {
        matches!(self.stage, GameStage::GameOver { .. })
    }
}

// Managing game state
impl Game {
    pub fn get_state(&self) -> PublicState {
        let (dealer_cards, dealer_score) = if self.players_turn() {
            // During player's turn, show only the first card
            let first_card: Vec<String> = vec![self.dealer.cards()[0].clone()];
            let mut temp_hand: Hand = Hand {
                cards: first_card,
                score: 0,
                actions: Vec::new(),
            };
            temp_hand.calculate_score();
            (temp_hand.cards().clone(), temp_hand.score())
        } else {
            // Show all dealer cards otherwise
            (self.dealer.cards().clone(), self.dealer.score())
        };

        PublicState {
            dealer_cards,
            player_cards: self.player.cards().clone(),
            dealer_score,
            player_score: self.player.score(),
            stage: self.stage.clone(),
        }
    }
    pub fn show_state(&self) {
        let state: PublicState = self.get_state();

        println!("===== BLACKJACK =====");
        println!(
            "Dealer: {} ({})",
            state.dealer_cards.join(", "),
            state.dealer_score
        );
        println!(
            "Player: {} ({})",
            state.player_cards.join(", "),
            state.player_score
        );
        println!(
            "{}",
            match state.stage {
                GameStage::PlayerTurn => "Your turn - (h)it or (s)tand?",
                GameStage::DealerTurn => "Dealer's turn",
                GameStage::Resolve => "Resolving game...",
                GameStage::GameOver { winner } => match winner {
                    Winner::Player => "Game over - You win!",
                    Winner::Dealer => "Game over - Dealer wins!",
                    Winner::Tie => "Game over - It's a tie!",
                },
            }
        );

        println!("=====================");
    }
}
