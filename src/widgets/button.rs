use std::cmp::PartialEq;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::{Color, RaylibDraw, Rectangle, Texture2D, Vector2};
use raylib::RaylibHandle;
use raylib::consts::MouseButton;
use crate::utils::color_lerp;
use crate::widgets::button_style::ButtonStyle;
use crate::widgets::Widget;

#[derive(PartialEq, Eq)]
pub enum ButtonMode {
    Disabled,
    Normal,
    Hovered,
    Pressed,
}

pub struct Button<Event> {
    pos: Vector2,
    size: Vector2,
    button_style: ButtonStyle,

    on_click: Option<Box<dyn FnMut() -> Vec<Event>>>,
    icon: Option<Texture2D>,
    is_flat: bool,

    button_mode: ButtonMode,
    butt_rect: Rectangle,
    scale: f32,
    prev_color: Color,
    next_color: Color,
    delta_color: f32,
}

impl<Event> Button<Event> {
    pub fn new(pos: Vector2, size: Vector2, button_style: ButtonStyle) -> Self {
        let button_mode = ButtonMode::Normal;
        let next_color = button_style.get_button_color(&button_mode);
        Button {
            pos,
            size,
            button_style,

            on_click: None,
            icon: None,
            is_flat: false,

            button_mode,
            butt_rect: Rectangle::new(pos.x, pos.y, size.x, size.y),
            scale: 1.0,
            prev_color: Color::BLACK,
            next_color,
            delta_color: 1.0,
        }
    }

    pub fn set_on_click<F: FnMut() -> Vec<Event> + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn flatten(mut self) -> Self {
        self.is_flat = true;
        self
    }

    pub fn with_icon(mut self, texture: Texture2D) -> Self {
        self.icon = Some(texture);
        self
    }

    pub fn get_mode(&self) -> &ButtonMode {
        &self.button_mode
    }

    pub fn set_mode(&mut self, mode: ButtonMode) {
        self.button_mode = mode;
        self.delta_color = 0.0;
        self.prev_color = self.next_color;
        self.next_color = self.button_style.get_button_color(&self.button_mode);
    }

    fn render_icon(&self, d: &mut RaylibDrawHandle) {
        if self.icon.is_none() {return;}
        let icon_width = self.icon.as_ref().unwrap().width as f32;
        let icon_height = self.icon.as_ref().unwrap().height as f32;

        const PADDING_PERCENT: f32 = 0.05;

        let rect = Rectangle::new(self.butt_rect.x, self.butt_rect.y,
              self.butt_rect.width * self.scale, self.butt_rect.height * self.scale);

        let padding_x = rect.width * PADDING_PERCENT;
        let padding_y = rect.height * PADDING_PERCENT;

        let available_width = rect.width - (2.0 * padding_x);
        let available_height = rect.height - (2.0 * padding_y);

        let button_aspect = available_width / available_height;
        let icon_aspect = icon_width / icon_height;

        let scale_factor;
        if icon_aspect > button_aspect {
            scale_factor = available_width / icon_width;
        } else {
            scale_factor = available_height / icon_height;
        }

        let dest_width = icon_width * scale_factor;
        let dest_height = icon_height * scale_factor;

        let dest_x = rect.x + (rect.width - dest_width) / 2.0;
        let dest_y = rect.y + (rect.height - dest_height) / 2.0;

        d.draw_texture_pro(
            self.icon.as_ref().unwrap(),
            Rectangle::new(0.0, 0.0, icon_width, icon_height),
            Rectangle::new(dest_x, dest_y, dest_width, dest_height),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        )
    }

    pub fn handle_mouse(&mut self, rl: &RaylibHandle, mouse_pos: Option<Vector2>) -> Vec<Event> {
        let mouse_pos = mouse_pos.unwrap_or(rl.get_mouse_position());
        let is_mouse_over = self.butt_rect.check_collision_point_rec(mouse_pos);
        match self.button_mode {
            ButtonMode::Hovered | ButtonMode::Normal => {
                self.set_mode(if is_mouse_over {ButtonMode::Hovered} else {ButtonMode::Normal});

                if is_mouse_over && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    self.set_mode(ButtonMode::Pressed);
                }
            }
            ButtonMode::Pressed => {
                if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                    if is_mouse_over {
                        self.set_mode(ButtonMode::Hovered);
                        if let Some(callback) = &mut self.on_click {
                            return callback();
                        }
                    } else {
                        self.set_mode(ButtonMode::Normal);
                    }
                }
            }
            ButtonMode::Disabled => {}
        };
        vec![]
    }
}

impl<Event> Widget for Button<Event> {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        if !self.is_flat {
            d.draw_rectangle_rounded(
                Rectangle::new(self.butt_rect.x + 3.0, self.butt_rect.y + 3.0,
                    self.butt_rect.width * self.scale, self.butt_rect.height * self.scale),
                0.7,
                9,
                Color::new(0, 0, 0, 50)
            );
        }

        // if self.is_flat, _scale is always gonna be 1.0, so we can just plug the same thing
        d.draw_rectangle_rounded(
            Rectangle::new(self.butt_rect.x, self.butt_rect.y,
                           self.butt_rect.width * self.scale, self.butt_rect.height * self.scale),
            0.7,
            9,
            color_lerp(self.prev_color, self.next_color, self.delta_color)
        );
        self.render_icon(d);
    }

    fn update(&mut self, _rl: &RaylibHandle, dt: f32) {
        match self.button_mode {
            ButtonMode::Disabled => {
                self.delta_color = 1.0;
                return;
            }
            ButtonMode::Hovered => {
                if self.scale < 1.05 {
                    self.scale += dt;
                    self.scale = self.scale.min(1.05);
                }
            }
            ButtonMode::Pressed | ButtonMode::Normal => {
                if self.scale > 1.0 {
                    self.scale -= dt;
                    self.scale = self.scale.max(1.0);
                }
            }
        }

        self.delta_color += dt * 4.0;
        self.delta_color = self.delta_color.min(1.0);
    }
}

#[macro_export]
macro_rules! collect_events {
    ($ty:ty; $($expr:expr),* $(,)?) => {{
        let events: Vec<$ty> = [
            $($expr),*
        ]
        .into_iter()
        .flatten()
        .collect();
        events
    }};
}