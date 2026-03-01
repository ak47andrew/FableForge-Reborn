use std::any::Any;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

pub trait Mode: Any {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self, rl: &RaylibHandle, dt: f32);
    fn as_any(&self) -> &dyn Any;
}