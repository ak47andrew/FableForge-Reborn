use raylib::color::Color;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{MouseButton, RaylibDraw, Rectangle, Texture2D, Vector2};
use raylib::RaylibHandle;
use crate::config::GRID_STEP;

pub struct Token {
    pub texture: Texture2D,
    pub position: Vector2,

    pub is_dragging: bool,
}

impl Token {
    pub fn new(texture: Texture2D, position: Vector2) -> Self {
        Token {
            texture,
            position,
            is_dragging: false,
        }
    }

    pub fn update(&mut self, dt: f32, rl: &RaylibHandle, mouse_pos: Vector2) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && Rectangle::new(
                -self.position.x, -self.position.y,
                GRID_STEP as f32, GRID_STEP as f32,
        ).check_collision_point_rec(mouse_pos){
            self.is_dragging = true;
        }
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
            || rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)  {
            self.is_dragging = false;
            self.snap_to_grid();
        }

        if self.is_dragging {
            self.position = -mouse_pos + Vector2::new(GRID_STEP as f32 / 2.0, GRID_STEP as f32 / 2.0);
        }
    }

    pub fn draw(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_texture_pro(
            &self.texture,
            Rectangle::new(0.0, 0.0, self.texture.width as f32, self.texture.height as f32),
            Rectangle::new(0.0, 0.0, GRID_STEP as f32, GRID_STEP as f32),
            &self.position,
            0.0,
            Color::WHITE
        )
    }

    fn snap_to_grid(&mut self) {
        self.position = Vector2::new(
            (self.position.x / GRID_STEP as f32).round() * GRID_STEP as f32,
            (self.position.y / GRID_STEP as f32).round() * GRID_STEP as f32
        );
    }

    pub fn try_update_pos(&mut self, pos: Vector2) {
        if !self.is_dragging {
            self.position = pos;
        }
    }
}