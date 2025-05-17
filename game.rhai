fn init() {
    print("Game initialized!");
}

fn update(dt) {
    if is_key_down("space") {
        print("Space pressed!");
    }
}

fn draw() {
    clear_background([100, 149, 237, 255]);

    draw_text("Hello from Rhai!", 20.0, 40.0, 30, [255, 255, 255, 255]);

    draw_circle(400.0, 300.0, 50.0, [255, 0, 0, 255]);
}
