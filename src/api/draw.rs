use macroquad::prelude::*;
use rhai::{Array, Engine};

pub fn register_draw_api(engine: &mut Engine) {
    engine.register_fn("clear_background", clear_background_rhai);
    engine.register_fn("draw_circle", draw_circle_rhai);
    engine.register_fn("draw_text", draw_text_rhai);
}

fn clear_background_rhai(color: Array) {
    println!("clear_background_rhai called with color: {:?}", color);
    if color.len() != 4 {
        println!("Invalid color array length: {}", color.len());
        return;
    }
    let vec: Vec<u8> = color.iter().map(|v| v.as_int().unwrap() as u8).collect();
    clear_background(Color::from_rgba(vec[0], vec[1], vec[2], vec[3]));
}

fn draw_circle_rhai(x: f64, y: f64, radius: f64, color: Array) {
    println!(
        "draw_circle_rhai called with x: {}, y: {}, radius: {}, color: {:?}",
        x, y, radius, color
    );
    if color.len() != 4 {
        println!("Invalid color array length: {}", color.len());
        return;
    }
    let vec: Vec<u8> = color.iter().map(|v| v.as_int().unwrap() as u8).collect();
    draw_circle(
        x as f32,
        y as f32,
        radius as f32,
        Color::from_rgba(vec[0], vec[1], vec[2], vec[3]),
    );
}

fn draw_text_rhai(text: &str, x: f64, y: f64, font_size: i64, color: Array) {
    println!(
        "draw_text_rhai called with text: {}, x: {}, y: {}, font_size: {}, color: {:?}",
        text, x, y, font_size, color
    );
    if text.is_empty() {
        println!("Text is empty");
        return;
    }
    if font_size < 0 {
        println!("Invalid font size: {}", font_size);
        return;
    }
    if color.len() != 4 {
        println!("Invalid color array length: {}", color.len());
        return;
    }
    let vec: Vec<u8> = color.iter().map(|v| v.as_int().unwrap() as u8).collect();
    draw_text(
        text,
        x as f32,
        y as f32,
        font_size as f32,
        Color::from_rgba(vec[0], vec[1], vec[2], vec[3]),
    );
}
