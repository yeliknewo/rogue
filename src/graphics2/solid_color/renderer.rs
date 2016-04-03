use std::sync::{Arc};
use std::error::Error;
use std::fmt;
use glium::Frame as GliumFrame;
use glium::{Surface};

use components::{Renderable};
use graphics2::{MatrixData};

pub struct RendererSolidColor;

impl RendererSolidColor {
    pub fn new() -> RendererSolidColor {
        RendererSolidColor
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, matrix_data: &MatrixData) -> Result<(), RendererSolidColorErr> {
        // frame.draw();
    }

    pub fn init_vertex() {

    }
}

#[derive(Debug)]
pub enum RendererSolidColorErr {
    Get(&'static str),
    // Draw(&'static str, DrawError),
}

impl fmt::Display for RendererSolidColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererSolidColorErr::Get(_) => write!(f, "Get was None"),
            // RendererTex2Err::Draw(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RendererSolidColorErr {
    fn description(&self) -> &str {
        match *self {
            RendererSolidColorErr::Get(_) => "Get was None",
            // RendererTex2Err::Draw(_, ref err) => err.description(),
        }
    }
}
