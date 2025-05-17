use macroquad::time;
use rhai::Engine;

pub fn register_time_api(engine: &mut Engine) {
    engine.register_fn("get_time", get_time_rhai);
}

fn get_time_rhai() -> f64 {
    time::get_time()
}
