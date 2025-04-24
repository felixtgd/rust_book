use crate::deck::Deck;
use crate::hand::Hand;
use std::cmp::Ordering;

pub struct Game {
    deck: Deck,
    pub player: Hand,
    pub dealer: Hand,
    is_over: bool,
    players_turn: bool,
    dealers_turn: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: Deck::new(),
            player: Hand::new(),
            dealer: Hand::new(),
            is_over: false,
            players_turn: true,
            dealers_turn: false,
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

    pub fn player_hit(&mut self) {
        self.player.hit(&mut self.deck);

        let player_bust: bool = self.player.is_bust();
        self.is_over = player_bust;
        self.dealer.wins = player_bust;
    }

    pub fn player_stand(&mut self) {
        self.players_turn = false;
        self.dealers_turn = true;
    }

    pub fn dealer_hit(&mut self) {
        self.dealer.hit(&mut self.deck);

        let dealer_bust: bool = self.dealer.is_bust();
        self.is_over = dealer_bust;
        self.player.wins = dealer_bust;
    }

    pub fn dealer_stand(&mut self) {
        self.dealers_turn = false;
    }

    pub fn is_over(&self) -> bool {
        self.is_over
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

    pub fn select_winner(&mut self) -> (bool, bool) {
        let mut player_wins: bool = false;
        let mut dealer_wins: bool = false;
        let mut tie: bool = false;

        match self.player.score.cmp(&self.dealer.score) {
            Ordering::Less => {
                println!("You loose!");
                dealer_wins = true;
            }
            Ordering::Greater => {
                println!("Congratulations, you win!");
                player_wins = true;
            }
            Ordering::Equal => {
                println!("Tie.");
                tie = true;
            }
        }

        self.game_over(player_wins && !tie, dealer_wins && !tie);
        (player_wins, tie)
    }

    pub fn game_over(&mut self, player_wins: bool, dealer_wins: bool) {
        self.player.wins = player_wins;
        self.dealer.wins = dealer_wins;
        self.is_over = true;
    }
}
