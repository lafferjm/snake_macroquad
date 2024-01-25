use macroquad::color;
use macroquad::math::{Vec2, vec2};
use macroquad::shapes::draw_rectangle;
use macroquad::window::{clear_background, Conf, next_frame, screen_height, screen_width};

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

struct Snake {
    position: Vec2
}

impl Snake {
    fn new() -> Self {
        let x = screen_width() / 2.0 - 20.0;
        let y = screen_height() / 2.0 - 20.0;
        Snake {
            position: vec2(x, y),
        }
    }

    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, 20.0, 20.0, color::GREEN);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let snake = Snake::new();

    loop {
        clear_background(color::BLACK);

        snake.draw();

        next_frame().await;
    }
}
