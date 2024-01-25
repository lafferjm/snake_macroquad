mod snake;

use macroquad::color;
use macroquad::time::get_time;
use macroquad::window::{clear_background, Conf, next_frame};
use crate::snake::Snake;

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
    let mut snake = Snake::new();

    let mut  last_update = get_time();
    loop {
        snake.process_input();

        if get_time() - last_update > 1.0/30.0 {
            last_update = get_time();
            snake.update();
        }

        clear_background(color::BLACK);
        snake.draw();

        next_frame().await;
    }
}
