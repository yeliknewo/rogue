use std::error::Error;
use std::fmt;

use graphics2::{RendererType};
use components::renderables::{RenderableTex2, RenderableVertexColor, RenderableSolidColor};

pub struct Renderable {
    renderer_type: RendererType,
    tex2: Option<Box<RenderableTex2>>,
    vertex_color: Option<Box<RenderableVertexColor>>,
    solid_color: Option<Box<RenderableSolidColor>>,
}

impl Renderable {
    pub fn new(renderer_type: RendererType) -> Renderable {
        Renderable {
            renderer_type: renderer_type,
            tex2: None,
            vertex_color: None,
            solid_color: None,
        }
    }

    pub fn get_renderer_type(&self) -> RendererType {
        self.renderer_type
    }

    pub fn get_tex2(&self) -> Option<Box<RenderableTex2>> {
        self.tex2
    }
}

#[derive(Debug)]
pub enum RenderableErr {
    Poison(&'static str),
    //Transform(&'static str, Box<TransformErr>),
    //Renderable(&'static str, RenderableErr),
}

impl fmt::Display for RenderableErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderableErr::Poison(_) => write!(f, "Thread was Poisoned During R/W"),
            //TransformErr::Transform(_, ref err) => err.fmt(f),
            //TransformErr::Renderable(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RenderableErr {
    fn description(&self) -> &str {
        match *self {
            RenderableErr::Poison(_) => "Thread was Poisoned",
            //TransformErr::Transform(_, ref err) => err.description(),
            //TransformErr::Renderable(_, ref err) => err.description(),
        }
    }
}
