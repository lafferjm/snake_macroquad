use macroquad::math::{ivec2, IVec2};
use macroquad::text::{draw_text_ex, measure_text, Font, TextParams};
use macroquad::window::screen_width;

pub struct Screen {
    font: Font,
    main_position: IVec2,
    secondary_position: IVec2,
    main_text: String,
    secondary_text: String,
}

impl Screen {
    pub fn new(font: Font, main_text: String, secondary_text: String) -> Self {
        // let main_text = "Snake";
        // let secondary_text = "Press <space> to play!";

        let main_size = measure_text(main_text.as_str(), Some(&font), 128, 1.0);
        let secondary_size = measure_text(secondary_text.as_str(), Some(&font), 32, 1.0);

        let main_x = (screen_width() / 2.0 - main_size.width / 2.0) as i32;
        let secondary_x = (screen_width() / 2.0 - secondary_size.width / 2.0) as i32;

        let main_position = ivec2(main_x, 150);
        let secondary_position = ivec2(secondary_x, 300);

        return Screen {
            font,
            main_position,
            secondary_position,
            main_text,
            secondary_text,
        };
    }

    pub fn draw(&self) {
        draw_text_ex(
            self.main_text.as_str(),
            self.main_position.x as f32,
            self.main_position.y as f32,
            TextParams {
                font_size: 128,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        draw_text_ex(
            self.secondary_text.as_str(),
            self.secondary_position.x as f32,
            self.secondary_position.y as f32,
            TextParams {
                font_size: 32,
                font: Some(&self.font),
                ..Default::default()
            },
        );
    }
}
