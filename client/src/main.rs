mod smart_camera;
mod config;
mod token;

use raylib::prelude::*;
use crate::config::{GRID_STEP, SCREEN};
use crate::smart_camera::SmartCamera;
use crate::token::Token;

fn main() {
    let (mut rl, mut thread) = init()
        .vsync()
        .size(SCREEN.x as i32, SCREEN.y as i32)
        .title("FableForge-Reborn")
        .build();


    let mut camera = SmartCamera::new();
    let bg_texture = rl.load_texture(&thread, "bg.jpg").expect("Failed to load texture");

    let icon = Image::load_image("logo.png").expect("Failed to load image");
    rl.set_window_icon(icon);

    let token_texture = rl.load_texture(&thread, "token.png").expect("Failed to load image");
    let mut token = Token::new(
        token_texture,
        Vector2::zero(),
    );

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let world_mouse = rl.get_screen_to_world2D(rl.get_mouse_position(), camera.camera);

        // Update
        camera.update_camera(&mut rl);
        token.update(dt, &rl, world_mouse);

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);

        camera.draw_world(&mut d, |dd| draw_world(dd, &bg_texture, &token, world_mouse));
        draw_gui(&mut d, &camera);
    }
}

fn draw_world(d: &mut RaylibMode2D<RaylibDrawHandle>, bg: &Texture2D, token: &Token, mouse_pos: Vector2) {
    d.draw_texture(bg, 0, 0, Color::WHITE);

    for x in (GRID_STEP..bg.width()).step_by(GRID_STEP as usize) {
        d.draw_line(x, 0, x, bg.height(), Color::WHITE);
    }

    for y in (GRID_STEP..bg.height()).step_by(GRID_STEP as usize) {
        d.draw_line(0, y, bg.width(), y, Color::WHITE);
    }

    token.draw(d);
}

fn draw_gui(d: &mut RaylibDrawHandle, camera: &SmartCamera) {
    // Just some temp GUI for now
    d.draw_text(format!("Position: {:?}", camera.camera.target).as_str(), 10, 10, 30, Color::WHITE);
    d.draw_text(format!("Zoom: {:?}", camera.camera.zoom).as_str(), 10, 10 * 2 + 30 * 1, 30, Color::WHITE);
}
