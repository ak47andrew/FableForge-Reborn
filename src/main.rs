mod config;
mod managers;
mod modes;
mod utils;
mod widgets;

use raylib::prelude::*;
use crate::managers::ModeManager;
use crate::modes::{DebugMode, Mode};
use crate::utils::Context;

fn main() {
    let (mut rl, mut thread) = init()
        .vsync()
        .size(640, 480)
        .title("Hello, World")
        .build();
    let mut mode_manager = {
        let mut context = Context::new(&mut rl, &mut thread);
        ModeManager::new(Box::new(DebugMode::new(&mut context)))
    };

    let mut return_to_debug = 0.0f32;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        {

            if config::DEBUG {
                if rl.is_key_down(KeyboardKey::KEY_ENTER) && rl.is_key_down(KeyboardKey::KEY_BACKSPACE) &&
                    mode_manager.get_current_mode().as_any().downcast_ref::<DebugMode>().is_none() {
                    let mut context = Context::new(&mut rl, &mut thread);
                    return_to_debug += dt;
                    println!("Returning to DebugMode in {} seconds...", config::ENTER_DEBUG_TIME - return_to_debug);
                    if return_to_debug > config::ENTER_DEBUG_TIME {
                        mode_manager.set_mode(Box::new(DebugMode::new(&mut context)));
                        return_to_debug = 0.0;
                    }
                } else {
                    return_to_debug = 0.0;
                }
            }


            let mut context = Context::new(&mut rl, &mut thread);
            mode_manager.get_current_mode().update(&mut context, dt);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);
        mode_manager.get_current_mode().draw(&mut d);
    }
}
