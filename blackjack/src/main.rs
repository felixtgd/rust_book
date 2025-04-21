use rand::prelude::SliceRandom;
use std::cmp::Ordering;
use std::io;

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let card_types: [&str; 13] = [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];

        let mut cards: Vec<String> = Vec::new();
        for _ in 0..4 {
            for &card in &card_types {
                cards.push(card.to_string());
            }
        }
        cards.shuffle(&mut rand::rng());

        Self { cards }
    }

    fn pop(&mut self) -> String {
        self.cards
            .pop()
            .expect("Deck is empty. No more cards to withdraw.")
    }
}

struct Hand {
    cards: Vec<String>,
    score: u32,
    is_bust: bool,
    has_blackjack: bool,
}

impl Hand {
    fn new() -> Self {
        Self {
            cards: Vec::new(),
            score: 0,
            is_bust: false,
            has_blackjack: false,
        }
    }

    fn deal_cards(&mut self, deck: &mut Deck, n: u32) {
        for _ in 0..n {
            self.cards.push(deck.pop());
        }
    }

    fn has_blackjack(&mut self) -> bool {
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

    fn calculate_score(&mut self) -> u32 {
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

    fn is_bust(&mut self) -> bool {
        self.is_bust = self.score > 21;
        self.is_bust
    }
}

fn main() {
    println!("Welcome to Blackjack!");

    let mut deck: Deck = Deck::new();
    let mut player: Hand = Hand::new();
    let mut dealer: Hand = Hand::new();

    let n: u32 = 2;
    player.deal_cards(&mut deck, n);
    dealer.deal_cards(&mut deck, n);

    if player.has_blackjack() {
        println!("\nBlackjack! You win!");
        std::process::exit(0);
    }

    if dealer.has_blackjack() {
        println!("\nDealer has Blackjack. You loose!");
        std::process::exit(0);
    }

    println!("\nDealer's open card: {:?}", dealer.cards[0]);

    loop {
        player.calculate_score();
        println!("\nYour hand: {:?}, Score: {}", player.cards, player.score);

        if player.is_bust() {
            println!("You went bust. You loose!");
            std::process::exit(0);
        }

        println!("What's your move? Hit or stand? h|s");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "h" => player.deal_cards(&mut deck, 1),
            "s" => break,
            _ => continue,
        }
    }

    dealer.calculate_score();
    while dealer.score < 17 {
        dealer.deal_cards(&mut deck, 1);
        dealer.calculate_score();
    }

    println!(
        "\nDealer's hand: {:?}, Score: {}",
        dealer.cards, dealer.score
    );
    if dealer.is_bust() {
        println!("Dealer went bust. You win!");
        std::process::exit(0);
    }

    match player.score.cmp(&dealer.score) {
        Ordering::Less => println!("You loose!"),
        Ordering::Greater => println!("Congratulations, you win!"),
        Ordering::Equal => println!("Tie."),
    }
}
