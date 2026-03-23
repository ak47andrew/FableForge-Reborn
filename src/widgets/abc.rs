use raylib::prelude::{RaylibDrawHandle, Vector2};
use crate::utils::Context;
use crate::utils::context::UpdateContext;

pub trait Widget {
    fn update(&mut self, ctx: &Context, update_ctx: &UpdateContext);
    fn draw(&self, d: &mut RaylibDrawHandle, position: Vector2);
    fn get_size(&self) -> Vector2; // Should return the size as the actual `draw` or can lead to UB
    fn children(&self) -> Vec<Box<dyn Widget>>;

    // Mouse stuff
    fn handle_mouse(&mut self, ctx: &Context, update_ctx: &UpdateContext) {}
    fn handle_click(&mut self, ctx: &Context, update_ctx: &UpdateContext, on_click: &mut dyn FnMut()) {}
}

/// Just a little helper that holds your widgets in place
/// so you don't need to store positions separately in an ugly way :3
pub struct Positioned<T: Widget> {
    pub widget: Box<T>,
    pub position: Vector2,
}

impl<T: Widget> Positioned<T> {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.widget.draw(d, self.position);
    }
}

impl<T: Widget> Widget for Positioned<T> {
    fn update(&mut self, ctx: &Context, update_ctx: &UpdateContext) {
        self.widget.update(ctx, update_ctx);
    }

    fn draw(&self, d: &mut RaylibDrawHandle, position: Vector2) {
        self.widget.draw(d, position);
    }

    fn get_size(&self) -> Vector2 {
        self.widget.get_size()
    }

    fn children(&self) -> Vec<Box<dyn Widget>> {
        self.widget.children()
    }

    fn handle_mouse(&mut self, ctx: &Context, update_ctx: &UpdateContext) {
        self.widget.handle_mouse(ctx, update_ctx);
    }

    fn handle_click(&mut self, ctx: &Context, update_ctx: &UpdateContext, on_click: &mut dyn FnMut()) {
        self.widget.handle_click(ctx, update_ctx, on_click);
    }
}