use raylib_rs_plain_sys as rl;
pub use rl::Color;
pub use rl::KeyboardKey;
pub use rl::Rectangle;
pub use rl::Texture;
pub use rl::Texture2D;
pub use rl::Vector2;
use std::ffi::CString;
pub mod color_define;

pub fn init_window(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int, title: &str) {
    let title: CString = CString::new(title).unwrap();
    unsafe { rl::InitWindow(width, height, title.as_ptr()) }
}

pub fn window_should_close() -> bool {
    return unsafe { rl::WindowShouldClose() };
}

pub fn begin_drawing() {
    unsafe { rl::BeginDrawing() }
}

pub fn clear_background(color: rl::Color) {
    unsafe { rl::ClearBackground(color) }
}

pub fn draw_text(
    text: &str,
    pos_x: ::std::os::raw::c_int,
    pos_y: ::std::os::raw::c_int,
    font_size: ::std::os::raw::c_int,
    color: rl::Color,
) {
    let text: CString = CString::new(text).unwrap();
    unsafe { rl::DrawText(text.as_ptr(), pos_x, pos_y, font_size, color) }
}

pub fn draw_texture(
    texture: rl::Texture2D,
    pos_x: ::std::os::raw::c_int,
    pos_y: ::std::os::raw::c_int,
    tint: Color,
) {
    unsafe { rl::DrawTexture(texture, pos_x, pos_y, tint) }
}

pub fn draw_texture_rec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color) {
    unsafe { rl::DrawTextureRec(texture, source, position, tint) }
}

pub fn end_drawing() {
    unsafe { rl::EndDrawing() }
}

pub fn close_window() {
    unsafe { rl::CloseWindow() }
}

pub fn set_target_fps(fps: ::std::os::raw::c_int) {
    unsafe { rl::SetTargetFPS(fps) }
}

pub fn get_fps() -> ::std::os::raw::c_int {
    return unsafe { rl::GetFPS() };
}

pub fn load_texture(file_name: &str) -> Option<rl::Texture2D> {
    let file_name: CString = CString::new(file_name).unwrap();
    let texture: rl::Texture2D;
    texture = unsafe { rl::LoadTexture(file_name.as_ptr()) };

    return if texture.id == 0 {
        Option::None
    } else {
        Option::Some(texture)
    };
}

pub fn unload_texture(texture: rl::Texture2D) {
    unsafe { rl::UnloadTexture(texture) };
}

pub fn get_screen_width() -> ::std::os::raw::c_int {
    return unsafe { rl::GetScreenWidth() };
}

pub fn get_screen_height() -> ::std::os::raw::c_int {
    return unsafe { rl::GetScreenHeight() };
}

pub fn is_key_down(key: rl::KeyboardKey) -> bool {
    // TODO: Might want to avoid `as`.
    // - e.g. `use num::ToPrimitive`
    return unsafe { rl::IsKeyDown(key as i32) };
}

pub fn get_random_value(
    min: ::std::os::raw::c_int,
    max: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    return unsafe { rl::GetRandomValue(min, max) };
}

pub fn measure_text(
    text: &str,
    font_size: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let text: CString = CString::new(text).unwrap();
    return unsafe { rl::MeasureText(text.as_ptr(), font_size) };
}

pub fn draw_rectangle(
    pos_x: ::std::os::raw::c_int,
    pos_y: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    color: Color,
) {
    unsafe {
        rl::DrawRectangle(pos_x, pos_y, width, height, color);
    }
}

pub fn fade(color: Color, alpha: f32) -> Color {
    return unsafe {
        rl::Fade(color, alpha)
    }
}

pub fn set_exit_key(key: KeyboardKey) {
    unsafe {
        rl::SetExitKey(key as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        init_window(800, 450, "raylib [core] example - basic window");
        assert_eq!(get_screen_width(), 800);
        assert_eq!(get_screen_height(), 450);
        while !window_should_close() {
            begin_drawing();
            clear_background(color_define::RAYWHITE);
            draw_text("Congrats! You created your first window!", 190, 200, 20, color_define::LIGHTGRAY);
            end_drawing();
        }
        close_window();
    }
}
