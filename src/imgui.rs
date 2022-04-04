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
#[derive(Debug, Clone, Copy)]
pub struct Handle {
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
    pub fn init_gui() -> Handle;
    pub fn update_gui(handle: Handle, vars: &mut Variables) -> ();
    pub fn destroy_gui(window: &c_void) -> ();
    pub fn close_window(window: &c_void) -> bool;
}