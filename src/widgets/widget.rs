use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

pub trait Widget {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self, rl: &RaylibHandle, dt: f32);
}