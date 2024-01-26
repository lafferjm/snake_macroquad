use macroquad::math::{ivec2, IVec2};
use macroquad::prelude::{draw_text_ex, measure_text, TextParams};
use macroquad::text::Font;

pub struct HUD {
    score_text: String,
    font: Font,
    position: IVec2,
}

impl HUD {
    pub fn new(initial_text: String, font: Font) -> Self {
        let text_size = measure_text(initial_text.as_str(), Some(&font), 24, 1.0);
        return HUD {
            score_text: initial_text,
            font,
            position: ivec2(5, 5 + text_size.offset_y as i32),
        };
    }

    pub fn update_text(&mut self, score: i32) {
        self.score_text = format!("Score: {}", score);
        let text_size = measure_text(self.score_text.as_str(), Some(&self.font), 24, 1.0);
        self.position = ivec2(5, 5 + text_size.offset_y as i32);
    }

    pub fn draw(&self) {
        draw_text_ex(
            self.score_text.as_str(),
            self.position.x as f32,
            self.position.y as f32,
            TextParams {
                font_size: 24,
                font: Some(&self.font),
                ..Default::default()
            },
        );
    }
}
