use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use raylib::prelude::{Color, RaylibDraw, Vector2};
use raylib::RaylibHandle;
use rand::random;
use crate::config::BASE_UI_SCALE;
use crate::modes::Mode;
use crate::utils::SmartCamera;
use crate::widgets::{Button, ButtonStyle, Widget};

pub struct DebugMode {
    camera: SmartCamera,
    butt_add: Button,
    butt_remove: Button,
    butt_change_color: Button,
    counter: Arc<AtomicU32>,
    text_color: Arc<Mutex<Color>>
}

impl DebugMode {
    pub fn new() -> Self {
        let counter = Arc::new(AtomicU32::new(0));
        let text_color = Arc::new(Mutex::new(Color::BLACK));

        let mut debug_mode = DebugMode {
            camera: SmartCamera::new(),
            butt_add: Button::new(
                Vector2::new(100.0, 150.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_GREEN
            ),
            butt_remove: Button::new(
                Vector2::new(300.0, 150.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_RED
            ),
            butt_change_color: Button::new(
                Vector2::new(200.0, 300.0),
                Vector2::new(10.0 * BASE_UI_SCALE as f32, 10.0 * BASE_UI_SCALE as f32),
                ButtonStyle::STYLE_PURPLE
            ),
            counter: counter.clone(),
            text_color: text_color.clone()
        };

        // Set the callbacks after initialization
        let counter_clone = counter.clone();
        debug_mode.butt_add = debug_mode.butt_add
            .set_on_click(move || {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            });

        let counter_clone = counter.clone();
        debug_mode.butt_remove = debug_mode.butt_remove
            .set_on_click(move || {
                counter_clone.fetch_sub(1, Ordering::Relaxed);
            });

        let text_color_clone = text_color.clone();
        debug_mode.butt_change_color = debug_mode.butt_change_color
            .set_on_click(move || {
                let mut color = text_color_clone.lock().unwrap();
                *color = Color::new(
                    random(),
                    random(),
                    random(),
                    255
                );
            });

        debug_mode
    }


    fn draw_world(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_text(format!("Counter: {}", self.counter.load(Ordering::Relaxed)).as_str(), 0, 0, 120, *self.text_color.lock().unwrap());
        self.butt_add.draw(d);
        self.butt_remove.draw(d);
        self.butt_change_color.draw(d);
    }
}

impl Mode for DebugMode {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.camera.draw_world(d, |s| self.draw_world(s))
    }

    fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        self.camera.update_camera(rl);
        self.butt_add.update(rl, dt);
        self.butt_add.handle_mouse(rl, Some(rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera.camera)));
        self.butt_remove.update(rl, dt);
        self.butt_remove.handle_mouse(rl, Some(rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera.camera)));
        self.butt_change_color.update(rl, dt);
        self.butt_change_color.handle_mouse(rl, Some(rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera.camera)));
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
