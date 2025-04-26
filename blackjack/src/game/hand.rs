use crate::game::deck::Deck;

#[derive(Default, Clone)]
pub struct Hand {
    pub cards: Vec<String>,
    pub score: u8,
    pub actions: Vec<String>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            score: 0,
            actions: Vec::new(),
        }
    }

    pub fn deal_cards(&mut self, deck: &mut Deck, n: u8) {
        for _ in 0..n {
            self.cards.push(deck.draw());
        }
        self.calculate_score();
    }

    pub fn has_blackjack(&mut self) -> bool {
        let mut ace: bool = false;
        let mut ten: bool = false;
        for card in self.cards.iter() {
            match card.as_str() {
                "A" => ace = true,
                "10" | "J" | "Q" | "K" => ten = true,
                _ => (),
            }
        }
        ace && ten
    }

    pub fn is_bust(&mut self) -> bool {
        self.score > 21
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        self.actions.push(String::from("h"));
        self.deal_cards(deck, 1);
    }

    pub fn stand(&mut self) {
        self.actions.push(String::from("s"));
    }

    pub fn calculate_score(&mut self) {
        let mut score: u8 = 0;
        let mut num_aces: u8 = 0;

        for card in self.cards.iter() {
            match card.as_str() {
                "A" => {
                    score += 11; // Treat Ace as 11 initially
                    num_aces += 1;
                }
                "J" | "Q" | "K" => score += 10,
                num_card => {
                    let card_val: u8 = num_card.parse().expect("Could not parse card as a number!");
                    score += card_val;
                }
            }
        }

        // Adjust score for Aces if it exceeds 21
        while score > 21 && num_aces > 0 {
            score -= 10; // Change an Ace from 11 to 1
            num_aces -= 1;
        }

        self.score = score;
    }
}

// Getters
impl Hand {
    pub fn cards(&self) -> &Vec<String> {
        &self.cards
    }

    pub fn score(&self) -> u8 {
        self.score
    }
}
