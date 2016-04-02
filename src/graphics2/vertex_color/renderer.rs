use std::sync::{Arc};
use std::error::Error;
use std::fmt;
use glium::Frame as GliumFrame;
use glium::{Surface};

use components::{Renderable};
use graphics2::{MatrixData};

pub struct RendererVertexColor;

impl RendererVertexColor {
    pub fn new() -> RendererVertexColor {
        RendererVertexColor
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, matrix_data: &MatrixData) {
        //frame.draw();
    }

    pub fn init_vertex() {

    }
}

#[derive(Debug)]
pub enum RendererVertexColorErr {
    Get(&'static str),
    // Draw(&'static str, DrawError),
}

impl fmt::Display for RendererVertexColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererVertexColorErr::Get(_) => write!(f, "Get was None"),
            // RendererTex2Err::Draw(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RendererVertexColorErr {
    fn description(&self) -> &str {
        match *self {
            RendererVertexColorErr::Get(_) => "Get was None",
            // RendererTex2Err::Draw(_, ref err) => err.description(),
        }
    }
}
