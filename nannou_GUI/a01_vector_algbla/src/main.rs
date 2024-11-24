use nannou::prelude::*;
use std::f64::consts::PI;

const SPACE_WIDTH: f64 = 10.0;
const RUN_TIME: f64 = 3.0; // Animation duration in seconds
const FRAME_RATE: u32 = 60; // Frames per second

// Interpolation function
fn interpolate(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

// Sigmoid function
fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + (-t).exp())
}

// Homotopy function
fn homotopy(x: f64, y: f64, t: f64) -> (f64, f64) {
    let norm = (x * x + y * y).sqrt();
    let tau = interpolate(5.0, -5.0, t) + norm / SPACE_WIDTH;
    let alpha = sigmoid(tau);
    (x, y + 0.5 * (2.0 * PI * alpha).sin())
}

// Model to manage application state
struct Model {
    time: f64, // Current time in seconds
}

// Initialize the Nannou app
fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    Model { time: 0.0 }
}

// Update function: Increment the time
fn update(app: &App, model: &mut Model, _update: Update) {
    model.time += 1.0 / FRAME_RATE as f64;
    if model.time > RUN_TIME {
        model.time = 0.0; // Loop the animation
    }
}

// View function: Draw the scene
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let grid_size = 20;
    let step = 400.0 / grid_size as f64;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = -200.0 + i as f64 * step;
            let y = -200.0 + j as f64 * step;
            let (hx, hy) = homotopy(x / 100.0, y / 100.0, model.time / RUN_TIME);

            // Scale homotopy output back for visualization
            draw.ellipse()
                .x_y(hx as f32 * 100.0, hy as f32 * 100.0)
                .radius(2.0)
                .color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

// Main entry point
fn main() {
    nannou::app(model).update(update).run();
}
