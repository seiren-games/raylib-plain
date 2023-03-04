use raylib_rs_plain_sys as rl;
use std::ffi::CString;
pub use rl::Color;

pub fn init_window(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int, title: &str) {
    let title: CString = CString::new(title).unwrap();
    unsafe {
        rl::InitWindow(width, height, title.as_ptr());
    }
}

pub fn window_should_close() -> bool {
    unsafe {
        return rl::WindowShouldClose();
    }
}

pub fn begin_drawing() {
    unsafe {
        rl::BeginDrawing();
    }
}

pub fn clear_background(color: rl::Color) {
    unsafe {
        rl::ClearBackground(color);
    }
}

pub fn draw_text(text: &str, pos_x: ::std::os::raw::c_int, pos_y: ::std::os::raw::c_int, font_size: ::std::os::raw::c_int, color: rl::Color) {
    let text: CString = CString::new(text).unwrap();
    unsafe {
        rl::DrawText(text.as_ptr(), pos_x, pos_y, font_size, color);
    }
}

pub fn end_drawing() {
    unsafe {
        rl::EndDrawing();
    }
}

pub fn close_window() {
    unsafe {
        rl::CloseWindow();
    }
}

pub fn set_target_fps(fps: ::std::os::raw::c_int) {
    unsafe {
        rl::SetTargetFPS(fps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let text: &str = "Congrats! You created your first window!";
        let bg_color:rl::Color = rl::Color { r: 245, g: 245, b: 245, a: 255 };
        let text_color:rl::Color = rl::Color { r: 200, g: 200, b: 200, a: 255 };
        init_window(800, 450, "raylib [core] example - basic window");
        while !window_should_close() {
            begin_drawing();
            clear_background(bg_color);
        
            draw_text(
                text,
                190,
                200,
                20,
                text_color
            );
            end_drawing();
        }
        close_window();
    }
}
