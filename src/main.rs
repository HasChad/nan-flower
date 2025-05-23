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

    let white_flower_pos = window_width / -2. + 200.;
    let purple_flower_pos = window_width / 2. - 200.;
    let big_flower_pos = 0.;

    let speed = 2000. / window_width;
    let saturation = if speed < 3.5 { speed / 3.5 } else { 1.0 };
    let spread = 2.0;

    // colors
    let sap = hsv(0.22, saturation * 1.0, 0.67);

    let white_flow_color = hsv(1.0, 0.0, 1.0);
    let white_inside_color = hsv(0.55, saturation * 0.5, 0.9);

    let purple_flow_color = hsv(0.77, 0.84, 0.67);
    let purple_flow_color = hsl(0.77, 0.72, 0.39);
    let purple_inside_color = hsv(0.33, saturation, 0.5);

    if window_width > 400.0 {
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
            (point, sap)
        });
        draw.polyline().weight(10.).points_colored(points);

        for degree in (0..=360).step_by(30) {
            let radus = deg_to_rad(app.time * speed * 10.0 + degree as f32);
            let x = radus.sin() * 75.;
            let y = radus.cos() * 75.;
            draw.ellipse()
                .color(white_flow_color)
                .w_h(50., 150.)
                .x_y(white_flower_pos + x, y)
                .rotate(-radus);
        }
        draw.ellipse()
            .x_y(white_flower_pos, 0.0)
            .color(white_inside_color);

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
            (point, sap)
        });
        draw.polyline().weight(15.).points_colored(points);

        for degree in (0..=360).step_by(45) {
            let radus = deg_to_rad(app.time * speed * -10.0 + degree as f32);
            let x = radus.sin() * 75.;
            let y = radus.cos() * 75.;
            draw.ellipse()
                .color(purple_flow_color)
                .w_h(100., 100.)
                .x_y(purple_flower_pos + x, y)
                .rotate(-radus);
        }
        draw.ellipse()
            .x_y(purple_flower_pos, 0.0)
            .color(purple_inside_color);

    // ! rainbow flower
    } else {
        let points = (0..50).map(|i| {
            let x = i as f32;
            let point = pt2(
                big_flower_pos
                    + map_range(
                        -(app.time * speed + x / spread).sin(),
                        -1.0,
                        1.0,
                        -25.0,
                        25.0,
                    ),
                x * -10.0,
            );
            (point, hsv(x / 50.0, 1.0, 1.0))
        });
        draw.polyline().weight(15.).points_colored(points);

        for degree in (0..=360).step_by(45) {
            let radus = deg_to_rad(app.time * speed * 10.0 + degree as f32);
            let x = radus.sin() * 75.;
            let y = radus.cos() * 75.;
            draw.ellipse()
                .color(WHITE)
                .w_h(75., 150.)
                .x_y(big_flower_pos + x, y)
                .rotate(-radus);
        }

        for degree in (0..=360).step_by(45) {
            let radus = deg_to_rad(app.time * speed * -10.0 + degree as f32);
            let x = radus.sin() * 75.;
            let y = radus.cos() * 75.;
            draw.ellipse()
                .color(PURPLE)
                .w_h(50., 50.)
                .x_y(big_flower_pos + x, y)
                .rotate(-radus);
        }
        draw.ellipse()
            .w_h(50., 50.)
            .x(big_flower_pos)
            .color(YELLOWGREEN);
    }

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
