use macroquad::math::{ivec2, IVec2};
use macroquad::text::{draw_text_ex, measure_text, Font, TextParams};
use macroquad::window::screen_width;

pub struct GameOverScreen {
    font: Font,
    main_position: IVec2,
    secondary_position: IVec2,
}

impl GameOverScreen {
    pub fn new(font: Font) -> Self {
        let game_over_text = "Game Over";
        let instructions_text = "Press <space> to start over!";

        let go_size = measure_text(game_over_text, Some(&font), 128, 1.0);
        let it_size = measure_text(instructions_text, Some(&font), 32, 1.0);

        let main_x = (screen_width() / 2.0 - go_size.width / 2.0) as i32;
        let secondary_x = (screen_width() / 2.0 - it_size.width / 2.0) as i32;

        let main_position = ivec2(main_x, 150);
        let secondary_position = ivec2(secondary_x, 300);

        return GameOverScreen {
            font,
            main_position,
            secondary_position,
        };
    }

    pub fn draw(&self) {
        draw_text_ex(
            "Game Over",
            self.main_position.x as f32,
            self.main_position.y as f32,
            TextParams {
                font_size: 128,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        draw_text_ex(
            "Press <space> to start over!",
            self.secondary_position.x as f32,
            self.secondary_position.y as f32,
            TextParams {
                font_size: 32,
                font: Some(&self.font),
                ..Default::default()
            },
        )
    }
}
