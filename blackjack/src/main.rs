use rand::prelude::SliceRandom;

fn main() {
    let cards: Vec<&str> = vec![
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

    let player_score: i32 = calculate_score(&player_hand);
    let dealer_score: i32 = calculate_score(&dealer_hand);

    println!("Player hand: {:?}, Score: {}", player_hand, player_score);
    println!("Dealer hand: {:?}, Score: {}", dealer_hand, dealer_score);
}

fn calculate_score(hand: &Vec<&str>) -> i32 {
    let mut score: i32 = 0;
    let mut num_aces: i32 = 0;

    for card in hand.iter() {
        match *card {
            "A" => {
                score += 11; // Treat Ace as 11 initially
                num_aces += 1;
            }
            "J" | "Q" | "K" => score += 10,
            num_str => {
                if let Ok(num) = num_str.parse::<i32>() {
                    score += num;
                } else {
                    println!("Warning: Could not parse card '{}' as a number!", num_str);
                }
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
