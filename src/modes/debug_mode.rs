use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use rand::random;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::{Context, SmartCamera};

enum ButtonEvents {
    CounterChange(i32),
    ChangeColor(Color)
}

pub struct DebugMode {
    camera: SmartCamera,
    counter: i32,
    text_color: Color
}

impl DebugMode {
    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text(format!("Counter: {}", self.counter).as_str(), 0, 0, 120, self.text_color);
    }

    fn handle_event(&mut self, events: Vec<ButtonEvents>) {
        for event in events {
            match event {
                ButtonEvents::CounterChange(v) => {self.counter += v}
                ButtonEvents::ChangeColor(c) => {self.text_color = c}
            }
        }
    }
}

impl Mode for DebugMode {
    fn new(context: &mut Context) -> Self {
        DebugMode {
            camera: SmartCamera::new(),
            counter: 0,
            text_color: Color::BLACK
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, context: &mut Context, dt: f32) {
        self.camera.update_camera(context.rl);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
