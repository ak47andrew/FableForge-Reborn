use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::{Color, MouseButton, Rectangle, Texture2D, Vector2};
use raylib::RaylibHandle;

pub struct Button {
    pub rect: Rectangle,
    pub texture: Texture2D,

    pub main_color: Color,
    pub hover_color: Color,
}

impl Button {
    pub fn new(pos: Vector2, size: Vector2, texture: Texture2D, main_color: Color, hover_color: Color) -> Self {
        Button {
            rect: Rectangle::new(
                pos.x,
                pos.y,
                size.x,
                size.y,
            ),
            texture,
            main_color,
            hover_color
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, mouse_pos: &Vector2, is_btn_released: bool) -> bool {
        let is_inside = self.rect.check_collision_point_rec(mouse_pos);
        d.draw_rectangle_rounded(
            self.rect,
            0.7,
            9,
            if is_inside {
                self.main_color
            } else {
                self.hover_color
            }
        );
        is_inside && is_btn_released
    }
}
