use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use crate::config::BASE_UI_SCALE;
use crate::widgets::{Button, Widget};
use crate::widgets::button::MinimalButton;

pub struct ButtonCollection<Event> {
    buttons: Vec<Button<Event>>,
}

impl<Event> ButtonCollection<Event> {
    pub fn new(pos: Vector2, grid: Vec<Vec<Option<MinimalButton<Event>>>>, config: ButtonCollectionConfig) -> ButtonCollection<Event> {
        let mut buttons = vec![];

        for (i, row) in grid.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if let Some(mb) = cell {
                    let button = Button::new(
                        pos + Vector2::new(
                            j as f32 * (config.horizontal_spacing as f32 + config.size.x),
                            i as f32 * (config.vertical_spacing as f32 + config.size.y)
                        ),
                        config.size,
                        mb
                    );
                    buttons.push(button);
                }
            }
        }

        ButtonCollection { buttons }
    }

    pub fn handle_mouse(&mut self, rl: &RaylibHandle, mouse_pos: Option<Vector2>) -> Vec<Event> {
        let mut events = vec![];
        for button in &mut self.buttons {
            events.push(button.handle_mouse(rl, mouse_pos));
        }

        events.into_iter().flatten().collect()
    }
}

pub struct ButtonCollectionConfig {
    size: Vector2,
    horizontal_spacing: u32,
    vertical_spacing: u32,
}

impl ButtonCollectionConfig {
    pub fn new() -> ButtonCollectionConfig {
        ButtonCollectionConfig {
            size: Vector2::new(BASE_UI_SCALE as f32, BASE_UI_SCALE as f32),
            horizontal_spacing: BASE_UI_SCALE,
            vertical_spacing: BASE_UI_SCALE,
        }
    }

    pub fn with_size(mut self, size: Vector2) -> ButtonCollectionConfig {
        self.size = size;
        self
    }

    pub fn with_horizontal_spacing(mut self, horizontal_spacing: u32) -> ButtonCollectionConfig {
        self.horizontal_spacing = horizontal_spacing;
        self
    }

    pub fn with_vertical_spacing(mut self, vertical_spacing: u32) -> ButtonCollectionConfig {
        self.vertical_spacing = vertical_spacing;
        self
    }
}

impl<Event> Widget for ButtonCollection<Event> {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        for button in &self.buttons {
            button.draw(d);
        }
    }

    fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        for button in &mut self.buttons {
            button.update(rl, dt);
        }
    }
}