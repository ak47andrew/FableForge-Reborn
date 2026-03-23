use raylib::{RaylibHandle, RaylibThread};
use raylib::prelude::Vector2;

pub struct Context<'a> {
    pub rl: &'a mut RaylibHandle,
    pub thread: &'a mut RaylibThread,
}

pub struct UpdateContext {
    pub dt: f32,
    pub mouse_pos: Vector2
}