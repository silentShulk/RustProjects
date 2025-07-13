use pyo3::Python;
use rust_manim::{Scene, Circle, Square, Animation};

fn main() {
    Python::with_gil(|py| {
        let scene = Scene::new(py).unwrap();

        let circle = Circle::new(py).unwrap();
        let square = Square::new(py).unwrap();

        scene.play(py, &Animation::create(py, circle.inner).unwrap()).unwrap();
        scene.play(py, &Animation::uncreate(py, circle.inner).unwrap()).unwrap();
        
        scene.play(py, &Animation::create(py, square.inner).unwrap()).unwrap();
        scene.play(py, &Animation::uncreate(py, square.inner).unwrap()).unwrap();

        scene.render(py).unwrap();
    })
}