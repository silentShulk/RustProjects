use pyo3::Python;
use rust_manim::{Scene, Circle, Square, Animation, Axes};

fn main() {
    Python::with_gil(|py| {
        let scene = Scene::new(py);

        let axes = Axes::new();
        let lambda_equation = py.import("math");
    })
}