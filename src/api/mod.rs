mod draw;
mod input;
mod time;

pub use draw::*;
pub use input::*;
pub use time::*;

pub fn register_apis(engine: &mut rhai::Engine) {
    register_draw_api(engine);
    register_input_api(engine);
    register_time_api(engine);
}
