mod smart_camera;
mod config;
mod token;
mod websocket;

use std::collections::HashMap;
use raylib::prelude::*;
use raylib::prelude::KeyboardKey::{KEY_N, KEY_Q};
use crate::config::{GRID_STEP, SCREEN};
use crate::smart_camera::SmartCamera;
use crate::token::Token;
use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use common::{CSPacket, SCPacket, TokenNetwork, Vector2D};
use crate::websocket::async_main;

fn main() {
    let (to_ws_tx, to_ws_rx) = mpsc::channel::<CSPacket>(100);
    let (from_ws_tx, mut from_ws_rx) = mpsc::channel::<SCPacket>(100);

    std::thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async_main(
            to_ws_rx,
            from_ws_tx,
            std::env::args().collect::<Vec<String>>().get(1).unwrap_or(&"0".to_string()).parse::<u64>().expect("Invalid id")
        ));
    });

    #[allow(unused_mut)]
    let (mut rl, mut thread) = init()
        .vsync()
        .size(SCREEN.x as i32, SCREEN.y as i32)
        .title("FableForge-Reborn")
        .build();

    let mut camera = SmartCamera::new();
    let bg_texture = rl.load_texture(&thread, "bg.jpg").expect("Failed to load texture");

    let icon = Image::load_image("logo.png").expect("Failed to load image");
    rl.set_window_icon(icon);

    let mut tokens = HashMap::<u32, Token>::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let world_mouse = rl.get_screen_to_world2D(rl.get_mouse_position(), camera.camera);

        if rl.is_key_pressed(KEY_N) {
            let id = tokens.keys().max().unwrap_or(&0u32) + 1;
            let mut token = Token::new(rl.load_texture(&thread, "token.png").expect("Failed to load image"), -world_mouse);
            token.snap_to_grid();
            to_ws_tx.blocking_send(CSPacket::AddToken {
                token: TokenNetwork {
                    id,
                    pos: Vector2D {
                        x: token.position.x,
                        y: token.position.y,
                    }
                }
            }).unwrap();
            tokens.insert(id, token);
        } else if rl.is_key_pressed(KEY_Q) {
            let mut selected_token = None;
            for (id, value) in tokens.iter().clone() {
                if value.is_dragging {
                    selected_token = Some(id.clone());
                }
            }
            if let Some(selected_token) = selected_token {
                tokens.remove(&selected_token);
                to_ws_tx.blocking_send(CSPacket::DeleteToken {
                    token: selected_token,
                }).unwrap();
            }
        }

        while let Ok(msg) = from_ws_rx.try_recv() {
            println!("Got from WS: {:?}", msg);

            match msg {
                SCPacket::Ok => {
                    // Ok :3
                }
                SCPacket::MoveToken { token } => {
                    let id = token.id;
                    tokens.get_mut(&id).unwrap().position = Vector2::new(token.pos.x, token.pos.y);
                }
                SCPacket::AddToken { token } => {
                    tokens.insert(token.id, Token::new(
                        rl.load_texture(&thread, "token.png").expect("Failed to load image"),
                        Vector2::new(token.pos.x, token.pos.y),
                    ));
                }
                SCPacket::DeleteToken { token } => {
                    tokens.remove(&token);
                }
            }
        }

        // Update
        camera.update_camera(&mut rl);

        for (id, token) in tokens.iter_mut() {
            let prev_pos = token.position;
            let result =  token.update(dt, &rl, world_mouse);
            let new_pos = token.position;
            if !token.is_dragging && prev_pos != new_pos {
                to_ws_tx.blocking_send(CSPacket::MoveToken {
                    token: TokenNetwork {
                        id: id.clone(),
                        pos: Vector2D {
                            x: new_pos.x,
                            y: new_pos.y,
                        }
                    }
                }).unwrap();
            }
            if result {
                break
            }
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);

        camera.draw_world(&mut d, |dd| draw_world(dd, &bg_texture, tokens.values().into_iter().collect()));
        draw_gui(&mut d, &camera);
    }
}

fn draw_world(d: &mut RaylibMode2D<RaylibDrawHandle>, bg: &Texture2D, tokens: Vec<&Token>) {
    d.draw_texture(bg, 0, 0, Color::WHITE);

    for x in (GRID_STEP..bg.width()).step_by(GRID_STEP as usize) {
        d.draw_line(x, 0, x, bg.height(), Color::WHITE);
    }

    for y in (GRID_STEP..bg.height()).step_by(GRID_STEP as usize) {
        d.draw_line(0, y, bg.width(), y, Color::WHITE);
    }

    for token in tokens {
        token.draw(d);
    }
}

fn draw_gui(d: &mut RaylibDrawHandle, camera: &SmartCamera) {
    // Just some temp GUI for now
    d.draw_text(format!("Position: {:?}", camera.camera.target).as_str(), 10, 10, 30, Color::WHITE);
    d.draw_text(format!("Zoom: {:?}", camera.camera.zoom).as_str(), 10, 10 * 2 + 30 * 1, 30, Color::WHITE);
}
