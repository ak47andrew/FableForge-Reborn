use std::any::Any;
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use rand::random;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::{Context, SmartCamera};
use crate::widgets::{ButtonCollection, ButtonStyle, Widget};
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
    fn new(context: &mut Context) -> Self {
        DebugMode {
            camera: SmartCamera::new(),
            buttons: ButtonCollection::new(
                Vector2::new(100.0, 150.0),
                vec![
                    vec![
                        Some(MinimalButton::new(ButtonStyle::STYLE_GREEN).set_on_click(
                            || {vec![ButtonEvents::CounterChange(1), ButtonEvents::ChangeColor(ButtonStyle::STYLE_GREEN.normal_color)]}
                        ).with_icon(context.rl.load_texture(context.thread, "logo.png").unwrap())),
                        None,
                        Some(
                            MinimalButton::new(ButtonStyle::STYLE_RED).set_on_click(
                                || {vec![ButtonEvents::CounterChange(-1), ButtonEvents::ChangeColor(ButtonStyle::STYLE_RED.normal_color)]}
                            ).with_icon(context.rl.load_texture(context.thread, "logo.png").unwrap())
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
                            ).with_icon(context.rl.load_texture(context.thread, "logo.png").unwrap())),
                        None
                    ]
                ],
                ButtonCollectionConfig::new().with_size(Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32))
            ),
            counter: 0,
            text_color: Color::BLACK
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, context: &mut Context, dt: f32) {
        self.camera.update_camera(context.rl);

        self.buttons.update(context.rl, dt);

        let mouse_pos = Some(context.rl.get_screen_to_world2D(context.rl.get_mouse_position(), self.camera.camera));
        let events: Vec<ButtonEvents> = self.buttons.handle_mouse(context.rl, mouse_pos);
        self.handle_event(events);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
