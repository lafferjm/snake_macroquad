use macroquad::color;
use macroquad::math::{IVec2, ivec2};
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

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    position: IVec2,
    direction: Direction
}

impl Snake {
    fn new() -> Self {
        let x = screen_width() as i32 / 2 - 20;
        let y = screen_height() as i32 / 2 - 20;

        Snake {
            position: ivec2(x, y),
            direction: Direction::UP,
        }
    }

    fn draw(&self) {
        draw_rectangle(self.position.x as f32, self.position.y as f32, 20.0, 20.0, color::GREEN);
    }

    fn update(&mut self) {
        let (x, y) = match self.direction {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        };

        self.position.x += x * 20;
        self.position.y += y * 20;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut snake = Snake::new();

    loop {
        snake.update();

        clear_background(color::BLACK);
        snake.draw();

        next_frame().await;
    }
}
