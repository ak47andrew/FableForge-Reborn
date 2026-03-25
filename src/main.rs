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

    while !rl.window_should_close() {
        // Update
        camera.update_camera(&mut rl);

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        camera.draw_world(&mut d, draw_world);
        draw_gui(&mut d);
    }
}

fn draw_world(d: &mut RaylibMode2D<RaylibDrawHandle>) {
    d.draw_rectangle(0, 0, 100, 100, Color::RED);
}

fn draw_gui(d: &mut RaylibDrawHandle) {
    d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
}
