use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use rand::random;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::SmartCamera;
use crate::widgets::{Button, ButtonCollection, ButtonStyle, Widget};
use crate::collect_events;
use crate::widgets::button::MinimalButton;
use crate::widgets::button_collection::ButtonCollectionConfig;

enum ButtonEvents {
    CounterChange(i32),
    ChangeColor(Color)
}

pub struct DebugMode {
    camera: SmartCamera,
    buttons: ButtonCollection<ButtonEvents>,
    counter: i32,
    text_color: Color
}

impl DebugMode {
    pub fn new() -> Self {
        DebugMode {
            camera: SmartCamera::new(),
            buttons: ButtonCollection::new(
                Vector2::new(100.0, 150.0),
                vec![
                    vec![
                        Some(MinimalButton::new(ButtonStyle::STYLE_GREEN).set_on_click(
                            || {vec![ButtonEvents::CounterChange(1), ButtonEvents::ChangeColor(ButtonStyle::STYLE_GREEN.normal_color)]}
                        )),
                        None,
                        Some(
                            MinimalButton::new(ButtonStyle::STYLE_RED).set_on_click(
                                || {vec![ButtonEvents::CounterChange(-1), ButtonEvents::ChangeColor(ButtonStyle::STYLE_RED.normal_color)]}
                            )
                        )
                    ],
                    vec![
                        None,
                        Some(
                            MinimalButton::new(ButtonStyle::STYLE_PURPLE).set_on_click(
                                || {vec![ButtonEvents::ChangeColor(Color::new(
                                    random(),
                                    random(),
                                    random(),
                                    255
                            ))]}
                        )),
                        None
                    ]
                ],
                ButtonCollectionConfig::new().with_size(Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32))
            ),
            counter: 0,
            text_color: Color::BLACK
        }
    }

    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text(format!("Counter: {}", self.counter).as_str(), 0, 0, 120, self.text_color);
        self.buttons.draw(d);
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

        self.buttons.update(rl, dt);

        let mouse_pos = Some(rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera.camera));
        let events: Vec<ButtonEvents> = self.buttons.handle_mouse(rl, mouse_pos);
        self.handle_event(events);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
