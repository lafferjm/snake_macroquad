use macroquad::color;
use rand::Rng;
use macroquad::math::{IVec2, ivec2};
use macroquad::shapes::{draw_rectangle};
use macroquad::window::{screen_height, screen_width};
use rand::rngs::ThreadRng;

pub struct Food {
    position: IVec2,
    rng: ThreadRng,
}

impl Food {
    pub fn new() -> Self{
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..(screen_width() / 20f32) as i32);
        let y  =  rng.gen_range(0..(screen_height() /20f32) as i32);

        return Food {
            position: ivec2(x * 20, y * 20),
            rng,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x as f32, self.position.y as f32, 20f32, 20f32, color::RED);
    }

    pub fn get_position(&self) -> &IVec2 {
        return &self.position;
    }

    pub fn regenerate(&mut self) {
        let x = self.rng.gen_range(0..(screen_width() / 20f32) as i32);
        let y = self.rng.gen_range(0..(screen_height()/20f32) as i32);

        self.position = ivec2(x * 20, y * 20);
    }
}
