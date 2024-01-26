use crate::food::Food;
use crate::hud::HUD;
use crate::snake::Snake;
use macroquad::color;
use macroquad::math::IVec2;
use macroquad::text::Font;
use macroquad::time::get_time;
use macroquad::window::{clear_background, next_frame};

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

pub struct Game {
    food: Food,
    snake: Snake,
    hud: HUD,
    score: i32,
    ticker: f64,
    game_state: GameState,
}

impl Game {
    pub fn new(font: Font) -> Self {
        return Game {
            food: Food::new(),
            snake: Snake::new(),
            hud: HUD::new(String::from("Score: 0"), font),
            score: 0,
            ticker: 0f64,
            game_state: GameState::Playing
        };
    }

    fn check_collision(&self, a: &IVec2, b: &IVec2) -> bool {
        return a.x == b.x && a.y == b.y;
    }

    fn draw(&self) {
        clear_background(color::BLACK);

        if self.game_state == GameState::Playing {
            self.hud.draw();
            self.food.draw();
            self.snake.draw();
        }
    }

    fn update(&mut self) {
        if get_time() - self.ticker > 1.0 / 30.0 {
            self.ticker = get_time();
            if self.game_state == GameState::Playing {
                self.snake.update();

                if self.check_collision(self.snake.head(), self.food.get_position()) {
                    self.snake.grow();
                    self.food.regenerate();
                    self.score += 1;
                    self.hud.update_text(self.score);
                }

                if self.snake.game_over() {
                    self.game_state = GameState::GameOver;
                }
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
