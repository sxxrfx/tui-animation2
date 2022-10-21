use utils::Stars;
use crate::{utils::V2ff, game::Game};

mod utils;
mod game;

fn main() {
    // let mut game = Game::init();

    // game.ball_animation();
    Game::init()
        .space_animation();

    // let star = Stars::new(1, V2ff::new(5.6, 7.2), '*');

    // println!("{:?}", star);
}
