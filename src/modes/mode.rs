use std::any::Any;
use raylib::drawing::RaylibDrawHandle;
use crate::utils::Context;

pub trait Mode: Any {
    fn new(context: &mut Context) -> Self where Self: Sized;
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self, context: &mut Context, dt: f32);
    fn as_any(&self) -> &dyn Any;
}