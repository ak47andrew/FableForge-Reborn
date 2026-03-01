use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw};
use raylib::RaylibHandle;
use crate::modes::Mode;
use crate::utils::SmartCamera;

pub struct DebugMode {
    camera: SmartCamera
}

impl DebugMode {
    pub fn new() -> DebugMode {
        DebugMode {camera: SmartCamera::new()}
    }

    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text("Hello, world!", 0, 0, 120, Color::BLACK);
    }
}

impl Mode for DebugMode {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        self.camera.update_camera(rl);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
