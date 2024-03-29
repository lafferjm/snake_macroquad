use macroquad::color;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::{ivec2, IVec2};
use macroquad::shapes::draw_rectangle;
use macroquad::window::{screen_height, screen_width};
use std::collections::LinkedList;

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    STOPPED,
}

pub struct Snake {
    direction: Direction,
    segments: LinkedList<IVec2>,
}

impl Snake {
    pub fn new() -> Self {
        let starting_x = screen_width() as i32 / 2 - 20;
        let starting_y = screen_height() as i32 / 2 - 20;

        let mut list: LinkedList<IVec2> = LinkedList::new();
        for i in 0..3 {
            list.push_back(ivec2(starting_x, starting_y + i * 20));
        }

        Snake {
            segments: list,
            direction: Direction::UP,
        }
    }

    pub fn draw(&self) {
        for segment in &self.segments {
            draw_rectangle(segment.x as f32, segment.y as f32, 20.0, 20.0, color::GREEN);
        }
    }

    pub fn update(&mut self) {
        let (x, y) = match self.direction {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
            Direction::STOPPED => (0, 0),
        };

        if self.direction != Direction::STOPPED {
            self.segments.pop_back();
            let head = self.segments.front().unwrap();
            let x = head.x + x * 20;
            let y = head.y + y * 20;
            let new_head = ivec2(x, y);
            self.segments.push_front(new_head);
        }
    }

    pub fn process_input(&mut self) {
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

    pub fn head(&self) -> &IVec2 {
        return self.segments.front().unwrap();
    }

    pub fn grow(&mut self) {
        let last_segment = self.segments.back().unwrap();
        self.segments
            .push_back(ivec2(last_segment.x, last_segment.y));
    }

    pub fn game_over(&self) -> bool {
        let head = self.segments.front().unwrap();

        if head.x > screen_width() as i32
            || head.x < 0
            || head.y > screen_height() as i32
            || head.y < 0
        {
            return true;
        }

        for segment in self.segments.iter().skip(1) {
            if segment.x == head.x && segment.y == head.y {
                return true;
            }
        }

        return false;
    }
}
