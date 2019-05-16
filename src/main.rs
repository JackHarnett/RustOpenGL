mod graph;
mod game;

use game::Game;
use game::GameLogic;

fn main() {
    println!("Hello, world!");

    let game = Game::new(GameLogic {
    	should_close : false,
    } );

    game.unwrap().init();
}
