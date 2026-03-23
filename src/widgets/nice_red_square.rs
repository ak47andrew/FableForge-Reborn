use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Color;
use raylib::math::Vector2;
use raylib::prelude::RaylibDraw;
use crate::utils::Context;
use crate::utils::context::UpdateContext;
use crate::widgets::abc::Widget;

pub struct NiceRedSquare {
    pub size: Vector2,
    pub color: Color
}

impl NiceRedSquare {
    pub fn new(size: Vector2, color: Color) -> Self {
        NiceRedSquare { size, color }
    }
}

impl Widget for NiceRedSquare {
    fn update(&mut self, ctx: &Context, update_ctx: &UpdateContext) {

    }

    fn draw(&self, d: &mut RaylibDrawHandle, position: Vector2) {
        d.draw_rectangle_v(
            position,
            self.size,
            self.color,
        )
    }

    fn get_size(&self) -> Vector2 {
        self.size
    }

    fn children(&self) -> Vec<Box<dyn Widget>> {
        vec![]
    }
}