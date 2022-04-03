use std::path::Path;
use cc;

fn main() {
    // println!("cargo:rerun-if-changed=src/gui/gui_lib.cpp");

    let inclue1 = Path::new("imgui");
    let inclue2 = Path::new("imgui/backends");
    let inclue3 = Path::new("imgui/examples/libs/glfw/include");

    cc::Build::new()
        .cpp(true)
        .include(inclue1)
        .include(inclue2)
        .include(inclue3)
        .file("src/gui/gui_lib.cpp")
        .file("imgui/imgui.cpp")
        .file("imgui/imgui_draw.cpp")
        .file("imgui/imgui_tables.cpp")
        .file("imgui/imgui_widgets.cpp")
        .file("imgui/backends/imgui_impl_opengl3.cpp")
        .file("imgui/backends/imgui_impl_glfw.cpp")
        .file("imgui/imgui_demo.cpp")
        .compile("gui_lib_cc");
    
    println!("cargo:rustc-link-lib=glfw3");
    println!("cargo:rustc-link-search=imgui/examples/libs/glfw/lib-vc2010-64");

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=opengl32");
    println!("cargo:rustc-link-lib=shell32");
}