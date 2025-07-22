use pyo3::Python;
use rust_manim::{Scene, Circle, Square, Animation};

fn main() {
    Python::with_gil(|py| {
        let scene = Scene::new(py).unwrap();

        let circle = Circle::new(py).unwrap();
        let square = Square::new(py).unwrap();

        scene.play(py, &Animation::create(py, circle.instance).unwrap()).unwrap();
        scene.play(py, &Animation::uncreate(py, circle.instance).unwrap()).unwrap();
        
        scene.play(py, &Animation::create(py, square.instance).unwrap()).unwrap();
        scene.play(py, &Animation::uncreate(py, square.instance).unwrap()).unwrap();

        scene.render(py).unwrap();
    })
}