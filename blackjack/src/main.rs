use rand::prelude::SliceRandom;
use std::cmp::Ordering;
use std::io;

fn main() {
    let cards: [&str; 13] = [
        "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
    ];

    let mut deck: Vec<&str> = vec![];
    for _ in 0..4 {
        deck.extend(cards.iter().cloned());
    }
    deck.shuffle(&mut rand::rng());

    // deal initial hands
    let mut player_hand: Vec<&str> = vec![];
    let mut dealer_hand: Vec<&str> = vec![];

    for _ in 0..2 {
        player_hand.extend(deck.pop());
        dealer_hand.extend(deck.pop());
    }

    let mut player_score: u32 = calculate_score(&player_hand);
    let mut dealer_score: u32 = calculate_score(&dealer_hand);

    let player_blackjack: bool = check_blackjack(&player_hand);
    let dealer_blackjack: bool = check_blackjack(&dealer_hand);

    println!(
        "Your hand: {:?}, Score: {}, Blackjack: {}",
        player_hand, player_score, player_blackjack
    );
    println!(
        "Dealer's hand: {:?}, Score: {}, Blackjack: {}",
        dealer_hand, dealer_score, dealer_blackjack
    ); // delete later
    println!("Dealer's open card: {:?}", dealer_hand[0]);

    loop {
        println!("What's your move? Hit or stand? h|s");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "h" => {
                player_hand.extend(deck.pop());
                player_score = calculate_score(&player_hand);
                println!("Your hand: {:?}, Score: {}", player_hand, player_score);
                if check_bust(player_score) {
                    println!("You went bust. You loose!");
                    std::process::exit(0);
                }
            }
            "s" => break,
            _ => continue,
        }
    }

    while dealer_score < 17 {
        dealer_hand.extend(deck.pop());
        dealer_score = calculate_score(&dealer_hand);
    }

    println!("Dealer's hand: {:?}, Score: {}", dealer_hand, dealer_score);
    if check_bust(dealer_score) {
        println!("Dealer went bust. You win!");
        std::process::exit(0);
    }

    println!("Your hand: {:?}, Score: {}", player_hand, player_score);

    match player_score.cmp(&dealer_score) {
        Ordering::Less => println!("You loose!"),
        Ordering::Greater => println!("Congratulations, you win!"),
        Ordering::Equal => println!("Tie."),
    }
}

fn calculate_score(hand: &Vec<&str>) -> u32 {
    let mut score: u32 = 0;
    let mut num_aces: u32 = 0;

    for card in hand.iter() {
        match *card {
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

fn check_blackjack(hand: &Vec<&str>) -> bool {
    let mut ace: bool = false;
    let mut ten: bool = false;
    for card in hand.iter() {
        match *card {
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
