@REM g++ gui.cpp -o test.exe -I../../imgui -I..\..\imgui/backends -I../../imgui/examples\libs\glfw\include\ -lopengl32 -llibglfw3 -L../../imgui/examples\libs\glfw\lib-mingw-w64\ ../../imgui/imgui.cpp ../../imgui\imgui_draw.cpp ../../imgui\imgui_tables.cpp ../../imgui\imgui_widgets.cpp ../../imgui\backends\imgui_impl_opengl3.cpp ../../imgui\backends\imgui_impl_glfw.cpp -lgdi32 ../../imgui\imgui_demo.cpp
g++ -shared -o gui_lib.lib gui_lib.cpp -I../../imgui -I..\..\imgui/backends -I../../imgui/examples\libs\glfw\include\ -lopengl32 -llibglfw3 -L../../imgui/examples\libs\glfw\lib-mingw-w64\ ../../imgui/imgui.cpp ../../imgui\imgui_draw.cpp ../../imgui\imgui_tables.cpp ../../imgui\imgui_widgets.cpp ../../imgui\backends\imgui_impl_opengl3.cpp ../../imgui\backends\imgui_impl_glfw.cpp -lgdi32 ../../imgui\imgui_demo.cpp




