
extern "C" {
    fn create_gui() -> ();
}

fn main() {
    unsafe {
        create_gui();
    }
}
