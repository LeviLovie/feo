use crate::script::ScriptEngine;
use std::{cell::RefCell, rc::Rc};

pub async fn run() {
    let script = std::fs::read_to_string("game.rhai").expect("game.rhai not found");
    let engine = Rc::new(RefCell::new(ScriptEngine::new()));
    let loading_start = std::time::Instant::now();
    println!("Script loaded");
    engine.borrow_mut().load_script(&script).unwrap();
    println!("Script compiled");
    let loading_duration = loading_start.elapsed();
    let loading_duration_ms = loading_duration.as_secs_f32() * 1000.0;

    let initialization_start = std::time::Instant::now();
    engine.borrow_mut().call_init();
    println!("Script initialized");
    let initialization_duration = initialization_start.elapsed();
    let initialization_duration_ms = initialization_duration.as_secs_f32() * 1000.0;

    let mut debug = false;
    let mut frame_times = Vec::new();
    let mut update_times = Vec::new();
    let mut draw_times = Vec::new();

    loop {
        let dt = macroquad::time::get_frame_time();
        frame_times.push(dt);
        if frame_times.len() > 100 {
            frame_times.remove(0);
        }

        if macroquad::input::is_key_pressed(macroquad::prelude::KeyCode::F3) {
            debug = !debug;
        }

        {
            let update_start = std::time::Instant::now();
            engine.borrow_mut().call_update(dt);
            let update_duration = update_start.elapsed().as_micros() as f32 / 10000.0;
            update_times.push(update_duration);
            if update_times.len() > 100 {
                update_times.remove(0);
            }

            let draw_start = std::time::Instant::now();
            engine.borrow_mut().call_draw();
            let draw_duration = draw_start.elapsed().as_micros() as f32 / 10000.0;
            draw_times.push(draw_duration);
            if draw_times.len() > 100 {
                draw_times.remove(0);
            }
        }

        if debug {
            let font_size = 24.0;

            let mut max_text_width = 0.0;

            let fps = macroquad::time::get_fps();
            let fps_text = format!("FPS: {}", fps);
            let fps_text_size = macroquad::text::measure_text(&fps_text, None, 32, 1.0);
            if fps_text_size.width > max_text_width {
                max_text_width = fps_text_size.width;
            }

            let loading_text = format!("Loading time: {:.2} ms", loading_duration_ms);
            let loading_text_size = macroquad::text::measure_text(&loading_text, None, 32, 1.0);
            if loading_text_size.width > max_text_width {
                max_text_width = loading_text_size.width;
            }

            let initialization_text = format!("Init time: {:.2} ms", initialization_duration_ms);
            let initialization_text_size =
                macroquad::text::measure_text(&initialization_text, None, 32, 1.0);
            if initialization_text_size.width > max_text_width {
                max_text_width = initialization_text_size.width;
            }

            macroquad::shapes::draw_rectangle(
                0.0,
                0.0,
                max_text_width,
                font_size * 3.0,
                macroquad::color::Color::from_rgba(0, 0, 0, 100),
            );
            macroquad::text::draw_text(
                &fps_text,
                8.0,
                font_size,
                font_size,
                macroquad::color::WHITE,
            );
            macroquad::text::draw_text(
                &loading_text,
                8.0,
                1.75 * font_size,
                font_size,
                macroquad::color::WHITE,
            );
            macroquad::text::draw_text(
                &initialization_text,
                8.0,
                2.5 * font_size,
                font_size,
                macroquad::color::WHITE,
            );

            draw_graph_at(
                frame_times.clone(),
                0.0,
                macroquad::prelude::screen_height() - 60.0 * 3.0,
                true,
                "Delta",
            );
            let avg_update = update_times.iter().sum::<f32>() / update_times.len() as f32;
            draw_graph_at(
                update_times.clone(),
                macroquad::prelude::screen_width() - update_times.len() as f32 * 3.0 - 20.0,
                0.0,
                false,
                &format!("Update: {}us", avg_update * 10000.0),
            );
            let avg_draw = draw_times.iter().sum::<f32>() / draw_times.len() as f32;
            draw_graph_at(
                draw_times.clone(),
                macroquad::prelude::screen_width() - update_times.len() as f32 * 3.0 - 20.0,
                60.0 * 3.0 + 10.0,
                false,
                &format!("Draw: {}us", avg_draw * 10000.0),
            );
        }

        macroquad::prelude::next_frame().await;
    }
}

fn draw_graph_at(data: Vec<f32>, x: f32, y: f32, fps_lines: bool, title: &str) {
    macroquad::shapes::draw_rectangle(
        x,
        y,
        data.len() as f32 * 3.0 + 20.0,
        60.0 * 3.0,
        macroquad::color::Color::from_rgba(0, 0, 0, 100),
    );
    for (i, &time) in data.iter().enumerate() {
        let bar_height = (time * 1000.0) as f32;
        let mut color = macroquad::color::GREEN;
        if time > 0.016 {
            color = macroquad::color::YELLOW;
        }
        if time > 0.033 {
            color = macroquad::color::RED;
        }
        macroquad::shapes::draw_rectangle(
            x + 10.0 + (i as f32) * 3.0,
            y + 60.0 * 3.0 - bar_height * 3.0 - 10.0,
            3.0,
            bar_height * 3.0,
            color,
        );
    }
    if fps_lines {
        macroquad::shapes::draw_line(
            x + 10.0,
            y + 60.0 * 3.0 - 16.0 * 3.0 - 10.0,
            x + data.len() as f32 * 3.0 + 10.0,
            y + 60.0 * 3.0 - 16.0 * 3.0 - 10.0,
            1.0,
            macroquad::color::Color::from_rgba(255, 255, 0, 100),
        );
        macroquad::shapes::draw_line(
            x + 10.0,
            y + 60.0 * 3.0 - 33.0 * 3.0 - 10.0,
            x + data.len() as f32 * 3.0 + 10.0,
            y + 60.0 * 3.0 - 33.0 * 3.0 - 10.0,
            1.0,
            macroquad::color::Color::from_rgba(255, 0, 0, 100),
        );
    }
    macroquad::text::draw_text(title, x + 10.0, y + 22.0, 20.0, macroquad::color::WHITE);
}
