use raylib::prelude::Color;

pub fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        (a.r as f32 + (b.r as f32 - a.r as f32) * t) as u8,
        (a.g as f32 + (b.g as f32 - a.g as f32) * t) as u8,
        (a.b as f32 + (b.b as f32 - a.b as f32) * t) as u8,
        (a.a as f32 + (b.a as f32 - a.a as f32) * t) as u8,
    )
}