fn main() {
    println!("cargo:rerun-if-changed=src/gui/gui_lib.cpp");

    std::process::Command::new("cmake").args(&[".."]).current_dir("src/gui/build").status().unwrap();
    std::process::Command::new("cmake").args(&["--build", "."]).current_dir("src/gui/build").status().unwrap();
    println!("cargo:rustc-link-lib=gui_lib");
    println!("cargo:rustc-link-search=src/gui/build/Debug");
    
    println!("cargo:rustc-link-lib=glfw3");
    println!("cargo:rustc-link-search=imgui/examples/libs/glfw/lib-vc2010-64");

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=opengl32");
    println!("cargo:rustc-link-lib=shell32");
}