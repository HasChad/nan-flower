use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let window_width = app.window_rect().w();

    let white_flower_pos = window_width / -2. + 150.;
    let purple_flower_pos = window_width / 2. - 150.;

    let speed = 3000. / window_width;
    let spread = 2.0;

    // ! white flower
    let points = (0..50).map(|i| {
        let x = i as f32;
        let point = pt2(
            white_flower_pos
                + map_range(
                    (app.time * speed + x / spread).sin(),
                    -1.0,
                    1.0,
                    -25.0,
                    25.0,
                ),
            x * -10.0,
        );
        (point, GREEN)
    });
    draw.polyline().weight(10.).points_colored(points);

    for degree in (0..=360).step_by(30) {
        let radus = deg_to_rad(app.time * speed * 10.0 + degree as f32);
        let x = radus.sin() * 75.;
        let y = radus.cos() * 75.;
        draw.ellipse()
            .color(WHITE)
            .w_h(50., 150.)
            .x_y(white_flower_pos + x, y)
            .rotate(-radus);
    }
    draw.ellipse().x_y(white_flower_pos, 0.0).color(LIGHTBLUE);

    // ! purple flower
    let points = (0..50).map(|i| {
        let x = i as f32;
        let point = pt2(
            purple_flower_pos
                + map_range(
                    -(app.time * speed + x / spread).sin(),
                    -1.0,
                    1.0,
                    -25.0,
                    25.0,
                ),
            x * -10.0,
        );
        (point, GREEN)
    });
    draw.polyline().weight(15.).points_colored(points);

    for degree in (0..=360).step_by(45) {
        let radus = deg_to_rad(app.time * speed * -10.0 + degree as f32);
        let x = radus.sin() * 75.;
        let y = radus.cos() * 75.;
        draw.ellipse()
            .color(PURPLE)
            .w_h(100., 100.)
            .x_y(purple_flower_pos + x, y)
            .rotate(-radus);
    }
    draw.ellipse().x_y(purple_flower_pos, 0.0).color(WHITE);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
