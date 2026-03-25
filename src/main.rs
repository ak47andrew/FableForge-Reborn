mod smart_camera;
mod config;

use raylib::prelude::*;
use crate::config::SCREEN;
use crate::smart_camera::SmartCamera;

fn main() {
    let (mut rl, mut thread) = init()
        .vsync()
        .size(SCREEN.x as i32, SCREEN.y as i32)
        .title("Hello, World")
        .build();

    let mut camera = SmartCamera::new();
    let bg_texture = rl.load_texture(&thread, "logo.png").expect("Failed to load texture");

    while !rl.window_should_close() {
        // Update
        camera.update_camera(&mut rl);

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);

        camera.draw_world(&mut d, |dd| draw_world(dd, &bg_texture));
        draw_gui(&mut d);
    }
}

fn draw_world(d: &mut RaylibMode2D<RaylibDrawHandle>, bg: &Texture2D) {
    d.draw_texture(bg, 0, 0, Color::WHITE);
}

fn draw_gui(d: &mut RaylibDrawHandle) {
    // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
}
