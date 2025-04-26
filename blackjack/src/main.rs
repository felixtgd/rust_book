use blackjack::Game;
use std::io;

fn main() {
    let mut game = Game::new();
    game.start();

    while !game.is_over() {
        // Player's turn
        if game.players_turn() {
            game.show_state();

            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");

            game.player_action(action);
            continue;
        }

        // Dealer's turn
        if game.dealers_turn() {
            game.dealer_action();
            continue;
        }

        // Resolve game if neither player's or dealer's turn
        break;
    }

    if !game.is_over() {
        game.select_winner();
    }

    game.show_state()
}
