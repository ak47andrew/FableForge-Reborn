use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use rand::random;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::SmartCamera;
use crate::widgets::{Button, ButtonStyle, Widget};
use crate::collect_events;

enum ButtonEvents {
    CounterChange(i32),
    ChangeColor(Color)
}

pub struct DebugMode {
    camera: SmartCamera,
    butt_add: Button<ButtonEvents>,
    butt_remove: Button<ButtonEvents>,
    butt_change_color: Button<ButtonEvents>,
    counter: i32,
    text_color: Color
}

impl DebugMode {
    pub fn new() -> Self {
        DebugMode {
            camera: SmartCamera::new(),
            butt_add: Button::new(
                Vector2::new(100.0, 150.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_GREEN
            ).set_on_click(
                || {vec![ButtonEvents::CounterChange(1), ButtonEvents::ChangeColor(ButtonStyle::STYLE_GREEN.normal_color)]}
            ),
            butt_remove: Button::new(
                Vector2::new(300.0, 150.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_RED
            ).set_on_click(
                || {vec![ButtonEvents::CounterChange(-1), ButtonEvents::ChangeColor(ButtonStyle::STYLE_RED.normal_color)]}
            ),
            butt_change_color: Button::new(
                Vector2::new(200.0, 300.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_PURPLE
            ).set_on_click(
                || {vec![ButtonEvents::ChangeColor(Color::new(
                    random(),
                    random(),
                    random(),
                    255
                ))]}
            ),
            counter: 0,
            text_color: Color::BLACK
        }
    }


    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text(format!("Counter: {}", self.counter).as_str(), 0, 0, 120, self.text_color);
        self.butt_add.draw(d);
        self.butt_remove.draw(d);
        self.butt_change_color.draw(d);
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
    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        self.camera.update_camera(rl);

        self.butt_add.update(rl, dt);
        self.butt_remove.update(rl, dt);
        self.butt_change_color.update(rl, dt);

        let mouse_pos = Some(rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera.camera));
        let events: Vec<ButtonEvents> = collect_events![ButtonEvents;
            self.butt_add.handle_mouse(rl, mouse_pos),
            self.butt_remove.handle_mouse(rl, mouse_pos),
            self.butt_change_color.handle_mouse(rl, mouse_pos),
        ];

        self.handle_event(events);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
