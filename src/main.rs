mod imgui;
use imgui::*;

fn main() {
    unsafe {
        let mut vars = Variables {
            window1: Window1 { show_demo_window: false, show_another_window: true },
            color: ImVec4::default()
        };
        let handle = init_gui();
        while !close_window(handle.window) {
            update_gui(handle, &mut vars);
        }
        destroy_gui(handle.window);
    }
}
