
use std::{ffi::c_void, env::VarError};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Handle {
    window: &'static c_void,
    io: &'static c_void
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Variables {
    show_demo_window: bool,
    show_another_window: bool,
    color: [f32; 4]
}

extern "C" {
    fn init_gui() -> Handle;
    fn update_gui(handle: Handle, vars: &mut Variables) -> ();
    fn destroy_gui(window: &c_void) -> ();
    fn close_window(window: &c_void) -> bool;
}

fn main() {
    unsafe {
        
        let mut vars = Variables {
            show_demo_window: true,
            show_another_window: true,
            color: [0.1, 0.1, 0.1, 1.0]
        };
        let handle = init_gui();
        while !close_window(handle.window) {
            update_gui(handle, &mut vars);
        }
        // update_gui(handle);
        destroy_gui(handle.window);
    }
}
