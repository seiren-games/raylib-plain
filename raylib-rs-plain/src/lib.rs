use raylib_rs_plain_sys as rl;
pub use rl::ConfigFlags;
pub use rl::KeyboardKey;
pub use rl::MouseButton;
use std::ptr::null_mut;
pub mod color_define;
pub use color_define as color;
pub mod function;
pub use function::*;
pub mod tools;
pub use tools::str_to_c_char;

pub fn load_font(file_name: &str) -> Option<Font> {
    let font: Font = function::load_font(file_name);
    return if is_font_ready(font) {
        Some(font)
    } else {
        None
    };
}

pub fn load_font_ex(
    file_name: &str,
    font_size: c_int,
    font_chars: Option<&mut [c_int]>,
    glyph_count: c_int,
) -> Option<Font> {
    let raw_font_chars = match font_chars {
        None => null_mut(),
        Some(v) => v.as_mut_ptr(),
    };
    let font: Font = function::load_font_ex(file_name, font_size, raw_font_chars, glyph_count);

    return if is_font_ready(font) {
        Some(font)
    } else {
        None
    };
}

pub fn load_texture(file_name: &str) -> Option<rl::Texture2D> {
    let texture: rl::Texture2D = function::load_texture(file_name);

    return if is_texture_ready(texture) {
        Option::Some(texture)
    } else {
        Option::None
    };
}

pub fn is_key_down(key: rl::KeyboardKey) -> bool {
    function::is_key_down(key as i32)
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    function::is_mouse_button_pressed(button as i32)
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    function::is_mouse_button_down(button as i32)
}

pub fn set_exit_key(key: KeyboardKey) {
    function::set_exit_key(key as i32)
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
            draw_text(
                "Congrats! You created your first window!",
                190,
                200,
                20,
                color::LIGHTGRAY,
            );
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
