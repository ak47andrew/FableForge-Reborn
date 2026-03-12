use raylib::prelude::Color;

pub fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        ((a.r + (b.r - a.r)) as f32 * t) as u8,
        ((a.g + (b.g - a.g)) as f32 * t) as u8,
        ((a.b + (b.b - a.b)) as f32 * t) as u8,
        ((a.a + (b.a - a.a)) as f32 * t) as u8,
    )
}