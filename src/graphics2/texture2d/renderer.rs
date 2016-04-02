use std::sync::{Arc};
use std::error::Error;
use std::fmt;
use std::collections::{HashMap};
use glium::Frame as GliumFrame;
use glium::{Surface, DrawError};

use logic::{Id};
use components::{Renderable};
use graphics2::{MatrixData};
use graphics2::texture2d::{Vertex, init_vertex};

pub struct RendererTex2 {
    vertex_buffers: HashMap<Id, Vertex>,
}

impl RendererTex2 {
    pub fn new() -> RendererTex2 {
        init_vertex();
        RendererTex2 {
            vertex_buffers: HashMap::new(),
        }
    }

    pub fn set_vertices(&mut self, id: Id, vertices: Vec<Vertex>) -> Result<(), RendererTex2Err> {

    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, matrix_data: &MatrixData) -> Result<(), RendererTex2Err> {
        let renderable = match renderable.get_tex2() {
            Some(renderable) => renderable,
            None => return Err(RendererTex2Err::Get("Renderable Get Tex2")),
        };
        match frame.draw(
            match self.vertex_buffers.get(renderable.get_vertex_id()) {
                Some(vertices) => vertices,
                None => return Err(RendererTex2Err::Get("Self VertexBuffers Get")),
            },
        ) {
            Ok(()) => (),
            Err(err) => return Err(RendererTex2Err::Draw("Frame Draw", err)),
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RendererTex2Err {
    Get(&'static str),
    Draw(&'static str, DrawError),
}

impl fmt::Display for RendererTex2Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererTex2Err::Get(_) => write!(f, "Get was None"),
            RendererTex2Err::Draw(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RendererTex2Err {
    fn description(&self) -> &str {
        match *self {
            RendererTex2Err::Get(_) => "Get was None",
            RendererTex2Err::Draw(_, ref err) => err.description(),
        }
    }
}
