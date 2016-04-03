use graphics2::texture2d::{RendererTex2, RendererTex2Err};
use graphics2::solid_color::{RendererSolidColor, RendererSolidColorErr};
use graphics2::vertex_color::{RendererVertexColor, RendererVertexColorErr};

pub struct Renderers {
    renderer_solid_color: RendererSolidColor,
    renderer_vertex_color: RendererVertexColor,
    renderer_texture2d: RendererTex2,
}

impl Renderers {
    pub fn new() -> Renderers {
        Renderers {
            renderer_solid_color: RendererSolidColor::new(),
            renderer_vertex_color: RendererVertexColor::new(),
            renderer_texture2d: RendererTex2::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum RendererType {
    SolidColor,
    VertexColor,
    Texture2d,
}
