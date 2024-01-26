mod food;
mod game;
mod hud;
mod screen;
mod snake;

use macroquad::text::load_ttf_font;
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
    let font = load_ttf_font("./Harabara.ttf").await.unwrap();

    let mut game = Game::new(font);
    game.run().await;
}
