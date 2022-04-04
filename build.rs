use std::{process::Command, fs, path::Path};
use cc;

fn main() {
    let imgui_str = format!("{}{}", std::env::var("OUT_DIR").unwrap(), "/imgui/");
    let imgui_path = imgui_str.as_str();

    // fs::remove_dir_all("target/debug/build/rust_imgui-9999b756f624420c/out/imgui/backends").expect("deleting error");
    
    //clone Dear ImGui 
    if !Path::new(imgui_path).exists() {
        Command::new("git").args(["clone", "https://github.com/ocornut/imgui.git", "--branch", "docking", imgui_path]).status().expect("cloning failed");
    }
    // Command::new("git").args(["-C", imgui_path, "pull",]).status().expect("pulling failed");

    //compile Dear ImGui
    cc::Build::new()
        .cpp(true)
        .include(format!("{}{}", imgui_path, ""))
        .include(format!("{}backends", imgui_path))
        .include(format!("{}{}", imgui_path, "examples/libs/glfw/include"))
        .file("src/gui/gui_lib.cpp")
        .file(format!("{}{}", imgui_path, "imgui.cpp"))
        .file(format!("{}{}", imgui_path, "imgui_draw.cpp"))
        .file(format!("{}{}", imgui_path, "imgui_tables.cpp"))
        .file(format!("{}{}", imgui_path, "imgui_widgets.cpp"))
        .file(format!("{}backends/imgui_impl_opengl3.cpp", imgui_path))
        .file(format!("{}backends/imgui_impl_glfw.cpp", imgui_path))
        .file(format!("{}backends/imgui_impl_glfw.cpp", imgui_path))
        .file(format!("{}{}", imgui_path, "imgui_demo.cpp"))
        .compile("gui_lib_cc");
    
    //link everything
    println!("cargo:rustc-link-lib=glfw3");
    println!("{}", format!("cargo:rustc-link-search={}examples/libs/glfw/lib-vc2010-64", imgui_path));

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=opengl32");
    println!("cargo:rustc-link-lib=shell32");
}