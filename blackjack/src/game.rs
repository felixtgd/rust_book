use crate::deck::Deck;
use crate::hand::Hand;
use std::cmp::Ordering;

pub struct Game {
    deck: Deck,
    player: Hand,
    dealer: Hand,
    players_turn: bool,
    dealers_turn: bool,
    player_wins: bool,
    dealer_wins: bool,
    is_over: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: Deck::new(),
            player: Hand::new(),
            dealer: Hand::new(),
            players_turn: true,
            dealers_turn: false,
            player_wins: false,
            dealer_wins: false,
            is_over: false,
        }
    }
}

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

    pub fn player_hit(&mut self) {
        self.player.hit(&mut self.deck);

        if self.player.is_bust() {
            self.game_over(false, true);
        }
    }

    pub fn player_stand(&mut self) {
        self.players_turn = false;
        self.dealers_turn = true;
    }

    pub fn dealer_hit(&mut self) {
        self.dealer.hit(&mut self.deck);

        if self.dealer.is_bust() {
            self.game_over(true, false);
        }
    }

    pub fn dealer_stand(&mut self) {
        self.dealers_turn = false;
    }

    pub fn check_blackjack(&mut self) {
        let player_blackjack: bool = self.player.has_blackjack();
        let dealer_blackjack: bool = self.dealer.has_blackjack();

        let tie: bool = player_blackjack && dealer_blackjack;
        let player_wins: bool = player_blackjack && !dealer_blackjack;
        let dealer_wins: bool = !player_blackjack && dealer_blackjack;

        if player_wins || dealer_wins || tie {
            self.game_over(player_wins && !tie, dealer_wins && !tie);
        }
    }

    pub fn player_action(&mut self, action: String) {
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
        if self.dealer.score < 17 {
            self.dealer_hit();
        } else {
            self.dealer_stand();
        }
    }

    pub fn select_winner(&mut self) {
        let mut player_wins: bool = false;
        let mut dealer_wins: bool = false;
        let mut tie: bool = false;

        match self.player.score.cmp(&self.dealer.score) {
            Ordering::Less => {
                dealer_wins = true;
            }
            Ordering::Greater => {
                player_wins = true;
            }
            Ordering::Equal => {
                tie = true;
            }
        }

        self.game_over(player_wins && !tie, dealer_wins && !tie);
    }

    pub fn game_over(&mut self, player_wins: bool, dealer_wins: bool) {
        self.player_wins = player_wins;
        self.dealer_wins = dealer_wins;
        self.is_over = true;
    }

    pub fn player_hand(&self) -> &Hand {
        &self.player
    }

    pub fn dealer_hand(&self) -> &Hand {
        &self.dealer
    }

    pub fn players_turn(&self) -> bool {
        self.players_turn
    }

    pub fn dealers_turn(&self) -> bool {
        self.dealers_turn
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }
}
