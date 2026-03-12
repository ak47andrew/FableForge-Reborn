use crate::modes::Mode;

pub struct ModeManager {
    pub current_mode: Box<dyn Mode>
}

impl ModeManager {
    pub fn new(init_mode: Box<dyn Mode>) -> ModeManager { ModeManager {current_mode: init_mode} }
    pub fn set_mode(&mut self, new_mode: Box<dyn Mode>) {
        self.current_mode = new_mode;
    }
    pub fn get_current_mode(&mut self) -> &mut dyn Mode {
        self.current_mode.as_mut()
    }
}
