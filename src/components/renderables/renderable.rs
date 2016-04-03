use std::error::Error;
use std::fmt;
use std::sync::{Arc};

use graphics2::{RendererType};
use components::renderables::{RenderableTex2, RenderableVertexColor, RenderableSolidColor};

pub struct Renderable {
    renderer_type: RendererType,
    texture2d: Option<Arc<RenderableTex2>>,
    vertex_color: Option<Arc<RenderableVertexColor>>,
    solid_color: Option<Arc<RenderableSolidColor>>,
}

impl Renderable {
    pub fn new() -> Renderable {
        Renderable {
            renderer_type: RendererType::Empty,
            texture2d: None,
            vertex_color: None,
            solid_color: None,
        }
    }

    pub fn new_from(other: Arc<Renderable>) -> Renderable {
        Renderable {
            renderer_type: other.renderer_type,
            texture2d: match other.texture2d.clone() {
                Some(tex2) => {
                    Some(Arc::new(RenderableTex2::new_from(tex2)))
                },
                None => None,
            },
            vertex_color: match other.vertex_color.clone() {
                Some(vertex) => {
                    Some(Arc::new(RenderableVertexColor::new_from(vertex)))
                },
                None => None,
            },
            solid_color: match other.solid_color.clone() {
                Some(solid) => {
                    Some(Arc::new(RenderableSolidColor::new_from(solid)))
                },
                None => None,
            }
        }
    }

    pub fn set_texture2d(&mut self, texture2d: RenderableTex2) {
        self.texture2d = Some(Arc::new(texture2d));
        self.renderer_type = RendererType::Texture2d;
    }

    pub fn set_vertex_color(&mut self, vertex_color: RenderableVertexColor) {
        self.vertex_color = Some(Arc::new(vertex_color));
        self.renderer_type = RendererType::VertexColor;
    }

    pub fn set_solid_color(&mut self, solid_color: RenderableSolidColor) {
        self.solid_color = Some(Arc::new(solid_color));
        self.renderer_type = RendererType::SolidColor;
    }

    pub fn get_renderer_type(&self) -> RendererType {
        self.renderer_type
    }

    pub fn get_texture2d(&self) -> Option<Arc<RenderableTex2>> {
        self.texture2d.clone()
    }

    pub fn get_solid_color(&self) -> Option<Arc<RenderableSolidColor>> {
        self.solid_color.clone()
    }

    pub fn get_vertex_color(&self) -> Option<Arc<RenderableVertexColor>> {
        self.vertex_color.clone()
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
