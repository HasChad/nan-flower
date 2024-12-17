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

    let sine = (app.time * 2.0).sin();

    let points = (0..15).map(|i| {
        let x = i as f32;
        let point = pt2(sine * x.sin(), -x * 1.4) * 20.0; //scale sine wave by 20.0
        (point, GREEN)
    });
    draw.polyline().weight(15.).points_colored(points);

    for degree in (0..=360).step_by(30) {
        let radus = deg_to_rad(app.time * 10.0 + degree as f32);
        let x = radus.sin() * 75.;
        let y = radus.cos() * 75.;
        draw.ellipse()
            .color(WHITE)
            .w_h(50., 150.)
            .x_y(x, y)
            .rotate(-radus);
    }
    draw.ellipse().color(YELLOW);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
