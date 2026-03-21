use raylib::{RaylibHandle, RaylibThread};

pub struct Context<'a> {
    pub rl: &'a mut RaylibHandle,
    pub thread: &'a mut RaylibThread,
}

impl<'a> Context<'a> {
    pub fn new(rl: &'a mut RaylibHandle, thread: &'a mut RaylibThread) -> Self {
        Self { rl, thread }
    }
}