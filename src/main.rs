use macroquad::color;
use macroquad::window::{clear_background, Conf, next_frame};

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_resizable:  false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(color::BLACK);
        next_frame().await;
    }
}
