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

fn main() {
    println!("Welcome to Blackjack!");

    let mut deck: Deck = Deck::new();

    // deal initial hands
    let mut player_hand: Vec<String> = Vec::new();
    let mut dealer_hand: Vec<String> = Vec::new();

    for _ in 0..2 {
        player_hand.push(deck.pop());
        dealer_hand.push(deck.pop());
    }

    if check_blackjack(&player_hand) {
        println!("\nBlackjack! You win!");
        std::process::exit(0);
    }

    if check_blackjack(&dealer_hand) {
        println!("\nDealer has Blackjack. You loose!");
        std::process::exit(0);
    }

    println!("\nDealer's open card: {:?}", dealer_hand[0]);

    let mut player_score: u32;
    loop {
        player_score = calculate_score(&player_hand);
        println!("\nYour hand: {:?}, Score: {}", player_hand, player_score);

        if check_bust(player_score) {
            println!("You went bust. You loose!");
            std::process::exit(0);
        }

        println!("What's your move? Hit or stand? h|s");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "h" => player_hand.push(deck.pop()),
            "s" => break,
            _ => continue,
        }
    }

    let mut dealer_score: u32 = calculate_score(&dealer_hand);
    while dealer_score < 17 {
        dealer_hand.push(deck.pop());
        dealer_score = calculate_score(&dealer_hand);
    }

    println!(
        "\nDealer's hand: {:?}, Score: {}",
        dealer_hand, dealer_score
    );
    if check_bust(dealer_score) {
        println!("Dealer went bust. You win!");
        std::process::exit(0);
    }

    match player_score.cmp(&dealer_score) {
        Ordering::Less => println!("You loose!"),
        Ordering::Greater => println!("Congratulations, you win!"),
        Ordering::Equal => println!("Tie."),
    }
}

fn calculate_score(hand: &[String]) -> u32 {
    let mut score: u32 = 0;
    let mut num_aces: u32 = 0;

    for card in hand.iter() {
        match card.as_str() {
            "A" => {
                score += 11; // Treat Ace as 11 initially
                num_aces += 1;
            }
            "J" | "Q" | "K" => score += 10,
            num_card => {
                let card_val: u32 = num_card.parse().expect("Could not parse card as a number!");
                score += card_val;
            }
        }
    }

    // Adjust score for Aces if it exceeds 21
    while score > 21 && num_aces > 0 {
        score -= 10; // Change an Ace from 11 to 1
        num_aces -= 1;
    }

    score
}

fn check_blackjack(hand: &[String]) -> bool {
    let mut ace: bool = false;
    let mut ten: bool = false;
    for card in hand.iter() {
        match card.as_str() {
            "A" => ace = true,
            "10" | "J" | "Q" | "K" => ten = true,
            _ => (),
        }
    }

    ace && ten
}

fn check_bust(score: u32) -> bool {
    score > 21
}
