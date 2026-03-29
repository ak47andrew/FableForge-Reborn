use raylib::prelude::{Camera2D, MouseButton, RaylibDrawHandle, RaylibMode2D, RaylibMode2DExt, Vector2};
use raylib::RaylibHandle;
use crate::config::{MAX_ZOOM, MIN_ZOOM, SCREEN, ZOOM_SPEED};

pub struct SmartCamera {
    pub camera: Camera2D,
    is_dragging: bool,
    drag_start_position: Vector2,
    drag_start_camera_target: Vector2
}

impl SmartCamera {
    pub fn new() -> Self {
        let mut camera = Camera2D::default();
        camera.target = Vector2::zero();
        camera.offset = Vector2::new(SCREEN.x / 2.0, SCREEN.y / 2.0);
        camera.rotation = 0.0;
        camera.zoom = 1.0;

        Self {
            camera,
            is_dragging: false,
            drag_start_position: Vector2::zero(),
            drag_start_camera_target: Vector2::zero()
        }
    }

    pub fn draw_world<FWorld>(
        &self,
        d: &mut RaylibDrawHandle,
        world: FWorld,
    )
    where
        FWorld: Fn(&mut RaylibMode2D<RaylibDrawHandle>),
    {
        let mut mode2d = d.begin_mode2D(self.camera);
        world(&mut mode2d);
    }

    pub fn update_camera(&mut self, rl: &RaylibHandle) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            self.is_dragging = true;
            self.drag_start_position = rl.get_mouse_position();
            self.drag_start_camera_target = self.camera.target;
        } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) {
            self.is_dragging = false;
        }

        let wheel = rl.get_mouse_wheel_move();

        if wheel != 0.0 {
            let mouse_world_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera);

            self.camera.zoom += wheel * ZOOM_SPEED;
            self.camera.zoom = self.camera.zoom.clamp(MIN_ZOOM, MAX_ZOOM);
            self.camera.zoom = (self.camera.zoom * 100.0).round() / 100.0;

            let new_mouse_world_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), self.camera);

            self.camera.target += mouse_world_pos - new_mouse_world_pos;
        }

        if self.is_dragging {
            let current_mouse_pos = rl.get_mouse_position();
            let mut delta = current_mouse_pos - self.drag_start_position;

            delta /= self.camera.zoom;
            self.camera.target = self.drag_start_camera_target - delta;
        }
    }
}