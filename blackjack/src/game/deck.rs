use rand::prelude::SliceRandom;
use std::iter::repeat;

#[derive(Default)]
pub struct Deck {
    cards: Vec<String>,
}

impl Deck {
    pub fn new() -> Self {
        let card_types: [&str; 13] = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];

        let mut cards: Vec<String> = card_types
            .iter()
            .flat_map(|&card| repeat(card.to_string()).take(4))
            .collect();

        cards.shuffle(&mut rand::rng());

        Self { cards }
    }

    pub fn draw(&mut self) -> String {
        self.cards
            .pop()
            .expect("Deck is empty. No more cards to withdraw.")
    }
}
