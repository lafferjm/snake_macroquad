use macroquad::color;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::{IVec2, ivec2};
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_time;
use macroquad::window::{clear_background, Conf, next_frame, screen_height, screen_width};

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

#[derive(PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    position: IVec2,
    direction: Direction,
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

    fn process_input(&mut self) {
        if is_key_pressed(KeyCode::Up) && self.direction != Direction::DOWN {
            self.direction = Direction::UP;
        } else if is_key_pressed(KeyCode::Down) && self.direction != Direction::UP {
            self.direction = Direction::DOWN;
        } else if is_key_pressed(KeyCode::Left) && self.direction != Direction::RIGHT {
            self.direction = Direction::LEFT;
        } else if is_key_pressed(KeyCode::Right) && self.direction != Direction::LEFT {
            self.direction = Direction::RIGHT;
        }
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
