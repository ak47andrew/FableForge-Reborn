use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::Color;
use crate::utils::Context;
use crate::utils::context::UpdateContext;
use crate::utils::helper::{Alignment, Size};
use crate::widgets::abc::Widget;
use crate::widgets::button_style::ButtonStyle;

#[derive(PartialEq, Eq)]
pub enum ButtonMode {
    Disabled,
    Normal,
    Hovered,
    Pressed,
}

pub struct Button {
    button_style: ButtonStyle,
    size: Size,
    child: Option<Box<dyn Widget>>,
    child_alignment: Alignment,
    is_flat: bool,

    button_mode: ButtonMode,
    scale: f32,
    prev_color: Color,
    next_color: Color,
    delta_color: f32,
}

impl Button {
    pub fn new(button_style: ButtonStyle, size: Size) -> Button {
        let next_color = button_style.normal_color;
        Button {
            button_style,
            size,
            child: None,
            child_alignment: Alignment::TRUE_CENTER,
            is_flat: false,

            button_mode: ButtonMode::Normal,
            scale: 1.0,
            prev_color: Color::BLACK,
            next_color,
            delta_color: 1.0
        }
    }

    pub fn flatten(mut self) -> Button {
        self.is_flat = true;
        self
    }

    pub fn with_child(mut self, child: Box<dyn Widget>, alignment: Alignment) -> Button {
        self.child = Some(child);
        self.child_alignment = alignment;
        self
    }
}

impl Widget for Button {
    fn update(&mut self, ctx: &Context, update_ctx: &UpdateContext) {
        todo!()
    }

    fn draw(&self, d: &mut RaylibDrawHandle, position: Vector2) {
        let inner_size = match &self.child {
            None => Vector2::zero(),
            Some(child) => child.get_size()
        };


    }

    fn get_size(&self) -> Vector2 {
        let inner_size = match &self.child {
            None => Vector2::zero(),
            Some(child) => child.get_size()
        };

        let x = inner_size.x.clamp(self.size.min_width, self.size.max_width);
        let y = inner_size.y.clamp(self.size.min_height, self.size.max_height);

        Vector2::new(x, y)
    }

    fn children(&self) -> Vec<&Box<dyn Widget>> {
        match &self.child {
            None => {vec![]}
            Some(c) => {vec![c]}
        }
    }

    fn handle_mouse(&mut self, ctx: &Context, update_ctx: &UpdateContext) {
        todo!()
    }

    fn handle_click(&mut self, ctx: &Context, update_ctx: &UpdateContext, on_click: &mut dyn FnMut()) {
        todo!()
    }
}