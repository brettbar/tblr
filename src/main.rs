use raylib_ffi::*;
use std::ffi::CString;

fn main() {
    println!("Hello, world!");

    let screen_width = 800;
    let screen_height = 450;

    unsafe {
        let title = CString::new("my rust window").unwrap();
        let text = CString::new("Hello, World!").unwrap();

        InitWindow(screen_width, screen_height, title.as_ptr());

        SetTargetFPS(60);

        while !WindowShouldClose() {
            BeginDrawing();

            let white = Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            };
            let red = Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            };

            ClearBackground(white);

            DrawText(text.as_ptr(), 190, 200, 20, red);

            EndDrawing();
        }
    }
}
