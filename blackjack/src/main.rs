use blackjack::Game;
use std::io;

fn main() {
    println!("Welcome to Blackjack!");

    let mut game = Game::new();
    game.start();

    println!("\nDealer's open card: {:?}", game.dealer_hand().cards[0]);

    while !game.is_over() && game.players_turn() {
        println!(
            "\nYour hand: {:?}, Score: {}",
            game.player_hand().cards,
            game.player_hand().score
        );
        println!("What's your move? Hit or stand? h|s");

        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        game.player_action(action);
    }

    while !game.is_over() && game.dealers_turn() {
        game.dealer_action();
    }

    if !game.is_over() {
        println!(
            "\nDealer's hand: {:?}, Score: {}",
            game.dealer_hand().cards,
            game.dealer_hand().score
        );
        game.select_winner();
    }

    println!("Player: {:?}", game.player);
    print!("Dealer: {:?}", game.dealer);
}
