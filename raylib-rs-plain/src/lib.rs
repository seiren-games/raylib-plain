use raylib_rs_plain_sys as rl;
pub use rl::KeyboardKey;
pub use rl::ConfigFlags;
pub use rl::MouseButton;
use std::ptr::null_mut;
pub mod color_define;
pub use color_define as color;
pub mod function;
pub use function::*;

pub fn init_window(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int, title: &str) {
    let title: CString = CString::new(title).unwrap();
    unsafe { rl::InitWindow(width, height, title.as_ptr()) }
}

pub fn load_font(file_name: &str) -> Option<Font> {
    let file_name: CString = CString::new(file_name).unwrap();
    let font:Font = unsafe { rl::LoadFont(file_name.as_ptr()) };

    return if is_font_ready(font) {
        Some(font)
    } else {
        None
    }
}

pub fn load_font_ex(
    file_name: &str,
    font_size: ::std::os::raw::c_int,
    font_chars: Option<&mut [::std::os::raw::c_int]>,
    glyph_count: ::std::os::raw::c_int,
) -> Option<Font> {
    let file_name: CString = CString::new(file_name).unwrap();
    let raw_font_chars = match font_chars {
        None => null_mut(),
        Some(v) => v.as_mut_ptr()
    };
    let font:Font = unsafe { rl::LoadFontEx(file_name.as_ptr(), font_size, raw_font_chars, glyph_count) };
    
    return if is_font_ready(font) {
        Some(font)
    } else {
        None
    }
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

pub fn draw_text_ex(
    font: Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    let text: CString = CString::new(text).unwrap();
    unsafe { rl::DrawTextEx(font, text.as_ptr(), position, font_size, spacing, tint) }
}

pub fn load_texture(file_name: &str) -> Option<rl::Texture2D> {
    let file_name: CString = CString::new(file_name).unwrap();
    let texture: rl::Texture2D = unsafe { rl::LoadTexture(file_name.as_ptr()) };

    return if texture.id == 0 {
        Option::None
    } else {
        Option::Some(texture)
    };
}

pub fn is_key_down(key: rl::KeyboardKey) -> bool {
    // TODO: Might want to avoid `as`.
    // - e.g. `use num::ToPrimitive`
    return unsafe { rl::IsKeyDown(key as i32) };
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { rl::IsMouseButtonPressed(button as i32) }
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { rl::IsMouseButtonDown(button as i32) }
}

pub fn measure_text(
    text: &str,
    font_size: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let text: CString = CString::new(text).unwrap();
    return unsafe { rl::MeasureText(text.as_ptr(), font_size) };
}

pub fn measure_text_ex(
    font: Font,
    text: &str,
    font_size: f32,
    spacing: f32,
) -> Vector2 {
    let text: CString = CString::new(text).unwrap();
    unsafe { rl::MeasureTextEx(font, text.as_ptr(), font_size, spacing) }
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
            clear_background(color::RAYWHITE);
            draw_text("Congrats! You created your first window!", 190, 200, 20, color::LIGHTGRAY);
            end_drawing();
        }
        close_window();
    }

    #[test]
    fn test_ci() {
        set_target_fps(60);
        dbg!(get_fps());
        assert_eq!(get_random_value(0, 0), 0);
        set_exit_key(KeyboardKey::KEY_NULL);
    }
}
