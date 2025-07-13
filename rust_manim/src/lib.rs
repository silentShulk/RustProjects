use pyo3::prelude::*;
use pyo3::types::PyAny;
use clap::Parser;

#[derive(Parser)]
#[group(required = true, multiple = false)]
struct VideoQuality {
    #[arg(short='l', long="quality-low", help="Render the animation at 480p 15 fps", action = clap::ArgAction::SetTrue)]
    low: bool,
    #[arg(short='s', long="quality-high", help="Render the animation at 1080p 30 fps", action = clap::ArgAction::SetTrue)]
    high: bool,
}

impl VideoQuality {
    fn get_quality(&self) -> String {
        if self.low {
            "low_quality".to_string()
        } else if self.high {
            "high_quality".to_string()
        }
        else {
            "low-quality".to_string()
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short='p', long="preview", help="Show preview of animation after rendering", action = clap::ArgAction::SetTrue)]
    pub preview: bool,
    #[clap(flatten)]
    pub quality: VideoQuality,
}


#[pyclass]
pub struct Animation {
    inner: Py<PyAny>,
}
#[pyclass]
pub struct Circle {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct Square {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct Scene {
    inner: Py<PyAny>,
}


#[pymethods]
impl Circle {
    #[new]
    pub fn new (py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let circle_class  = manim.getattr("Circle")?;
        let circle_mobject = circle_class.call0()?;

        Ok( Circle{ instance: circle_mobject.into() } )
    }
}
#[pymethods]
impl Square {
    #[new]
    pub fn new (py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let square_class  = manim.getattr("Square")?;
        let square_mobject = square_class.call0()?;

        Ok( Square { instance: square_mobject.into() } )
    }
}

#[pymethods]
impl Animation {
    #[staticmethod]
    pub fn create (py: Python, mobject: Py<PyAny>) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let create_animation = manim.getattr("Create")?;
        let create_instance = create_animation.call1((mobject,))?;

        Ok(Animation {inner: create_instance.into()})
    }
    #[staticmethod]
    pub fn uncreate (py: Python, mobject: Py<PyAny>) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let uncreate_animation = manim.getattr("Uncreate")?;
        let uncreate_instance = uncreate_animation.call1((mobject,))?;

        Ok(Animation {inner: uncreate_instance.into()})
    }
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new(py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;

        let args = Args::parse();
        let config = manim.getattr("config")?;
        config.setattr("quality", args.quality.get_quality())?;
        config.setattr("media_dir", "../media")?;
        config.setattr("preview", args.preview)?;

        let cls = manim.getattr("Scene")?;
        let instance = cls.call0()?;

        Ok(Scene { inner: instance.into() })
    }

    pub fn play(&self, py: Python, animation: &Animation) -> PyResult<()> {
        self.inner.as_ref().call_method1(py, "play", (animation.inner.as_ref(),))?;
        Ok(())
    }

    pub fn render(&self, py: Python) -> PyResult<()> {
        self.inner.as_ref().call_method0(py, "render")?;
        Ok(())
    }
}

#[pymodule]
fn rust_manim(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Animation>()?;
    Ok(())
}

