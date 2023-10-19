mod game;

fn main() {
    let game = game::Game::new([0u8; 16], 4);
    game.print_history()
}

