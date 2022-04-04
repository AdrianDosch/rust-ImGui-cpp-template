use std::{ffi::c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Variables {
    pub color: ImVec4,
    pub window1: Window1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Window1 {
    pub show_demo_window: bool,
    pub show_another_window: bool,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GUI {
    pub window: &'static c_void,
    pub io: &'static c_void
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ImVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

extern "C" {
    fn init_gui() -> GUI;
    fn update_gui(GUI: GUI, vars: &mut Variables) -> ();
    fn destroy_gui(window: &c_void) -> ();
    fn close_window(window: &c_void) -> bool;
}

impl GUI {
    pub fn new() -> GUI {
        unsafe {init_gui()}
    }

    pub fn terminate(&self) -> bool {
        unsafe {close_window(self.window)}
    }

    pub fn update(&self, vars: &mut Variables) {
        unsafe {update_gui(self.clone(), vars)}
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        unsafe {destroy_gui(self.window)}
    }
}