use pyo3::*;
use rust_manim::{Scene, Circle, Square, Animation, Axes};

fn main() {
    Python::with_gil(|py| {
        let scene = Scene::new(py).unwrap();

        
        let axes = Axes::new(py).unwrap();

        let lambda_equation = |x: f64| x.sin();
        let plotted_function = axes.plot(py, lambda_equation);

    })
}