use pyo3::prelude::*;
use pyo3::types::PyAny;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short='p', long="preview", help="Show preview of animation after rendering", action = clap::ArgAction::SetTrue)]
    pub preview: bool,
    #[arg(short='h', long="high", help="Set rendering quality to 1080p 60fps", action = clap::ArgAction::SetTrue)]
    pub quality_high: bool,
    #[arg(short='m', long="medium", help="Set rendering quality to 720p 30fps", action = clap::ArgAction::SetTrue)]
    pub quality_medium: bool,
    #[arg(short='l', long="low", help="Set rendering quality to 480p 15fps", action = clap::ArgAction::SetTrue)]
    pub quality_low: bool,
}
impl Args {
    fn get_quality(&self) -> String {
        if self.quality_high == true {
            return String::from("high_quality")
        } else if self.quality_medium == true {
            return String::from("medium_quality")
        } else {
            return String::from("low_quality")
        }
    }
}

// TYPES IMPORTED FROM MANIM
#[pyclass]
pub struct Circle {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct Square {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct Dot {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct Axes {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct ParametricFunction {
    pub instance: Py<PyAny>,
}
#[pyclass]
pub struct Animation {
    inner: Py<PyAny>,
}
#[pyclass]
pub struct Scene {
    inner: Py<PyAny>,
}


// IMPLEMENTING METHODS FOR THE ADDED TYPES
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
impl Dot {
    #[new]
    pub fn new (py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let dot_class  = manim.getattr("Dot")?;
        let dot_mobject = dot_class.call0()?;

        Ok( Dot { instance: dot_mobject.into() } )
    }
}

#[pymethods]
impl Axes {
    #[new]
    pub fn new (py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let axes_class  = manim.getattr("Axes")?;
        let axes_mobject = axes_class.call0()?;

        Ok( Axes { instance: axes_mobject.into() } )
    }

    pub fn plot<'py>(&self, py: Python<'py>, function: ) -> PyResult<ParametricFunction> {
        let plotted_function = self.instance.call_method1(py, "plot", (function,))?;
        Ok( ParametricFunction { instance: plotted_function.into() } )
    }
}

#[pymethods]
impl ParametricFunction {
    #[new]
    pub fn new (py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let parametricunction_class  = manim.getattr("ParametricFunction")?;
        let parametricfunction_mobject = parametricunction_class.call0()?;

        Ok( ParametricFunction { instance: parametricfunction_mobject.into() } )
    }
}

#[pymethods]
impl Animation {
    #[staticmethod]
    pub fn create<'py>(py: Python<'py>, mobject: &Bound<'py, PyAny>) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let creation_of_object = manim.call_method1("Create", (mobject,))?;

        Ok(Animation {inner: creation_of_object.into()})
    }
    #[staticmethod]
    pub fn uncreate<'py>(py: Python<'py>, mobject: &Bound<'py, PyAny>) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let uncreation_of_object = manim.call_method1("Uncreate", (mobject,))?;

        Ok(Animation {inner: uncreation_of_object.into()})
    }
    #[staticmethod]
    pub fn fade_in<'py>(py: Python<'py>, mobject: Py<PyAny>) -> PyResult<Self> {
        let manim = py.import("manim")?;
        let fadein_of_object = manim.call_method1("FadeIn", (mobject,))?;

        Ok(Animation {inner: fadein_of_object.into()})
    }
}

#[pymethods]
impl Scene {
    #[new]
    pub fn new(py: Python) -> PyResult<Self> {
        let manim = py.import("manim")?;

        let args = Args::parse();
        let config = manim.getattr("config")?;
        config.setattr("quality", args.get_quality())?;
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


// IMPORTING EVERYTHING ADDED IN THE PYTHON MODULE
#[pymodule]
fn rust_manim(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Scene>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Axes>()?;
    m.add_class::<ParametricFunction>()?;
    m.add_class::<Animation>()?;
    m.add_class::<Square>()?;
    Ok(())
}
