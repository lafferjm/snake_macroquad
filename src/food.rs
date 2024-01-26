use macroquad::color;
use rand::Rng;
use macroquad::math::{IVec2, ivec2};
use macroquad::shapes::{draw_rectangle};
use macroquad::window::{screen_height, screen_width};

pub struct Food {
    position: IVec2,
}

impl Food {
    pub fn new() -> Self{
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..(screen_width() / 20f32) as i32);
        let y  =  rng.gen_range(0..(screen_height() /20f32) as i32);
        return Food {
            position: ivec2(x * 20, y * 20),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x as f32, self.position.y as f32, 20f32, 20f32, color::RED);
    }
}
