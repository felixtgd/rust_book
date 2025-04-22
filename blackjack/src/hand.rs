use crate::deck::Deck;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<String>,
    pub score: u32,
    pub actions: Vec<String>,
    pub is_bust: bool,
    pub has_blackjack: bool,
    pub wins: bool,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            score: 0,
            actions: Vec::new(),
            is_bust: false,
            has_blackjack: false,
            wins: false,
        }
    }

    pub fn deal_cards(&mut self, deck: &mut Deck, n: u32) {
        for _ in 0..n {
            self.cards.push(deck.pop());
        }
        self.calculate_score();
        self.is_bust();
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
        self.has_blackjack = ace && ten;
        self.has_blackjack
    }

    pub fn calculate_score(&mut self) -> u32 {
        let mut score: u32 = 0;
        let mut num_aces: u32 = 0;

        for card in self.cards.iter() {
            match card.as_str() {
                "A" => {
                    score += 11; // Treat Ace as 11 initially
                    num_aces += 1;
                }
                "J" | "Q" | "K" => score += 10,
                num_card => {
                    let card_val: u32 =
                        num_card.parse().expect("Could not parse card as a number!");
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
        score
    }

    pub fn is_bust(&mut self) -> bool {
        self.is_bust = self.score > 21;
        self.is_bust
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        self.actions.push(String::from("h"));
        self.deal_cards(deck, 1);
    }

    pub fn stand(&mut self) {
        self.actions.push(String::from("s"));
    }
}
