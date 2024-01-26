use macroquad::color;
use macroquad::math::IVec2;
use macroquad::text::{draw_text_ex, Font, measure_text, TextParams};
use macroquad::time::get_time;
use macroquad::window::{clear_background, next_frame};
use crate::food::Food;
use crate::snake::Snake;

pub struct Game {
    food: Food,
    snake: Snake,
    score: i32,
    ticker: f64,
    font: Font,
}

impl Game {
    pub fn new(font: Font) -> Self {

        return Game {
            food: Food::new(),
            snake: Snake::new(),
            score: 0,
            ticker: 0f64,
            font,
        };
    }

    fn check_collision(&self, a: &IVec2, b: &IVec2) -> bool {
        return a.x == b.x && a.y == b.y;
    }

    fn draw(&self) {
        clear_background(color::BLACK);

        let score_text = format!("Score: {}", self.score);
        let text_size = measure_text(score_text.as_str(), Some(&self.font), 24, 1.0);

        draw_text_ex(score_text.as_str(), 5.0, 5.0 + text_size.offset_y, TextParams {
                font_size: 24,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        self.food.draw();
        self.snake.draw();
    }

    fn update(&mut self) {
        if get_time() - self.ticker > 1.0 / 30.0 {
            self.ticker = get_time();
            self.snake.update();


            if self.check_collision(self.snake.head(), self.food.get_position()) {
                self.snake.grow();
                self.food.regenerate();
                self.score += 1;
            }
        }
    }

    pub async fn run(&mut self) {
        self.ticker = get_time();

        loop {
            self.snake.process_input();

            self.update();
            self.draw();

            next_frame().await;
        }
    }
}