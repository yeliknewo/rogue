use std::fmt;
use std::error::Error;

use graphics2::texture2d::{RendererTex2, RendererTex2Err};
use graphics2::solid_color::{RendererSolidColor, RendererSolidColorErr};
use graphics2::vertex_color::{RendererVertexColor, RendererVertexColorErr};
use graphics2::{Window};

pub struct Renderers {
    renderer_solid_color: RendererSolidColor,
    renderer_vertex_color: RendererVertexColor,
    renderer_texture2d: RendererTex2,
}

impl Renderers {
    pub fn new(window: &mut Window) -> Result<Renderers, RenderersErr> {
        Ok(
            Renderers {
                renderer_solid_color: RendererSolidColor::new(),
                renderer_vertex_color: RendererVertexColor::new(),
                renderer_texture2d: match RendererTex2::new(window) {
                    Ok(tex2) => tex2,
                    Err(err) => return Err(RenderersErr::RendererTexture2d("RendererTex2 New", err)),
                },
            }
        )
    }

    pub fn get_mut_solid_color(&mut self) -> &mut RendererSolidColor {
        &mut self.renderer_solid_color
    }

    pub fn get_mut_vertex_color(&mut self) -> &mut RendererVertexColor {
        &mut self.renderer_vertex_color
    }

    pub fn get_mut_texture2d(&mut self) -> &mut RendererTex2 {
        &mut self.renderer_texture2d
    }
}

#[derive(Copy, Clone)]
pub enum RendererType {
    SolidColor,
    VertexColor,
    Texture2d,
}

#[derive(Debug)]
pub enum RenderersErr {
    RendererSolidColor(&'static str, RendererSolidColorErr),
    RendererVertexColor(&'static str, RendererVertexColorErr),
    RendererTexture2d(&'static str, RendererTex2Err),
}

impl fmt::Display for RenderersErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderersErr::RendererSolidColor(_, ref err) => err.fmt(f),
            RenderersErr::RendererVertexColor(_, ref err) => err.fmt(f),
            RenderersErr::RendererTexture2d(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RenderersErr {
    fn description(&self) -> &str {
        match *self {
            RenderersErr::RendererSolidColor(_, ref err) => err.description(),
            RenderersErr::RendererVertexColor(_, ref err) => err.description(),
            RenderersErr::RendererTexture2d(_, ref err) => err.description(),
        }
    }
}
