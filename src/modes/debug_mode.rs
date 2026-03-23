use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use rand::random;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::{Context, SmartCamera};
use crate::utils::context::UpdateContext;
use crate::widgets::abc::{Positioned, Widget};
use crate::widgets::nice_red_square::NiceRedSquare;

pub struct DebugMode {
    camera: SmartCamera,
    counter: i32,
    text_color: Color,
    nrq: Positioned<NiceRedSquare>
}

impl DebugMode {
    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text(format!("Counter: {}", self.counter).as_str(), 0, 0, 120, self.text_color);
        self.nrq.draw(d);
    }
}

impl Mode for DebugMode {
    fn new(_: &mut Context) -> Self {
        DebugMode {
            camera: SmartCamera::new(),
            counter: 0,
            text_color: Color::BLACK,
            nrq: Positioned {
                widget: Box::from(NiceRedSquare::new(
                    Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                    Color::RED
                )),
                position: Vector2::zero()
            }
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, context: &mut Context, dt: f32) {
        self.camera.update_camera(context.rl);
        self.nrq.update(context, &UpdateContext {
            dt,
            mouse_pos: Vector2::zero(),  // FIXME: placeholder
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
