mod food;
mod game;
mod snake;

use macroquad::window::Conf;

use crate::game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    game.run().await;
}
