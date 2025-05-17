use macroquad::prelude::*;
use rhai::Engine;

pub fn register_input_api(engine: &mut Engine) {
    engine.register_fn("is_key_down", is_key_down_rhai);
}

fn is_key_down_rhai(key_name: &str) -> bool {
    let key = match key_name.to_lowercase().as_str() {
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "space" => KeyCode::Space,
        _ => return false,
    };
    is_key_down(key)
}
