use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::SmartCamera;
use crate::widgets::{Button, ButtonStyle, Widget};

pub struct DebugMode {
    camera: SmartCamera,
    butt: Button
}

impl DebugMode {
    pub fn new() -> DebugMode {
        DebugMode {
            camera: SmartCamera::new(),
            butt: Button::new(
                Vector2::new(0.0, 50.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_PURPLE
            ).set_on_click(|| println!("Clicked"))
        }
    }

    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text("Hello, world!", 0, 0, 120, Color::BLACK);
        self.butt.draw(d);
    }
}

impl Mode for DebugMode {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        self.camera.update_camera(rl);
        self.butt.update(rl, dt);
        self.butt.handle_mouse(rl, Some(rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera.camera)))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
