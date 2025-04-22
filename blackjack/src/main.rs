use crate::deck::Deck;
use crate::hand::Hand;
use std::{cmp::Ordering, io};

mod deck;
mod hand;

fn game_over(winner: &mut Hand, loser: &mut Hand) {
    winner.wins = true;
    loser.wins = false;
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
        game_over(&mut player, &mut dealer);
    }

    if dealer.has_blackjack() {
        println!("\nDealer has Blackjack. You loose!");
        game_over(&mut dealer, &mut player);
    }

    println!("\nDealer's open card: {:?}", dealer.cards[0]);

    while !(player.wins || dealer.wins) {
        println!("\nYour hand: {:?}, Score: {}", player.cards, player.score);
        println!("What's your move? Hit or stand? h|s");

        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "h" => {
                player.hit(&mut deck);
                if player.is_bust() {
                    println!("You went bust. You loose!");
                    game_over(&mut dealer, &mut player);
                }
            }
            "s" => {
                player.stand();
                break;
            }
            _ => continue,
        }
    }

    while !(player.wins || dealer.wins) {
        if dealer.score < 17 {
            dealer.hit(&mut deck);
            if dealer.is_bust() {
                println!("Dealer went bust. You win!");
                game_over(&mut player, &mut dealer);
            }
        } else {
            dealer.stand();
            break;
        }
    }

    if !(player.wins || dealer.wins) {
        println!(
            "\nDealer's hand: {:?}, Score: {}",
            dealer.cards, dealer.score
        );

        match player.score.cmp(&dealer.score) {
            Ordering::Less => {
                println!("You loose!");
                game_over(&mut dealer, &mut player);
            }
            Ordering::Greater => {
                println!("Congratulations, you win!");
                game_over(&mut player, &mut dealer);
            }
            Ordering::Equal => {
                println!("Tie.");
                player.wins = false;
                dealer.wins = false;
            }
        }
    }

    println!("Player: {:?}", player);
    print!("Dealer: {:?}", dealer);
}
