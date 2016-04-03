use std::sync::{Arc};
use std::error::Error;
use std::fmt;
use std::collections::{HashMap};
use glium::Frame as GliumFrame;
use glium::{Surface, VertexBuffer, IndexBuffer, DrawParameters, Program, ProgramCreationError, DrawError};
use glium;

use components::{Renderable};
use graphics2::{MatrixData, Window};
use graphics2::vertex_color::{Vertex, init_vertex, Index};
use logic::{Id};

pub struct RendererVertexColor {
    vertex_buffers: HashMap<Id, VertexBuffer<Vertex>>,
    index_buffers: HashMap<Id, IndexBuffer<Index>>,
    draw_parameters: HashMap<Id, DrawParameters<'static>>,
    program: Program,
}

impl RendererVertexColor {
    pub fn new(window: &mut Window) -> Result<RendererVertexColor, RendererVertexColorErr> {
        init_vertex();
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coord;
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            out vec2 v_tex_coord;

            void main() {
                v_tex_coord = tex_coord;
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coord;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coord);
            }
        "#;
        Ok(
            RendererVertexColor {
                vertex_buffers: HashMap::new(),
                index_buffers: HashMap::new(),
                draw_parameters: HashMap::new(),
                program: match Program::from_source(window.get_facade(), vertex_shader_src, fragment_shader_src, None) {
                    Ok(program) => program,
                    Err(err) => return Err(RendererVertexColorErr::ProgramCreation("Program From Source", err)),
                }
            }
        )
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, matrix_data: &MatrixData) -> Result<(), RendererVertexColorErr> {
        let renderable_vertex = match renderable.get_vertex_color() {
            Some(vertex) => vertex,
            None => return Err(RendererVertexColorErr::Get("Renderable Get Vertex Color")),
        };
        match frame.draw(
            match self.vertex_buffers.get(&renderable_vertex.get_vertex_id()) {
                Some(vertices) => vertices,
                None => return Err(RendererVertexColorErr::Get("Self Vertex Buffers Get")),
            },
            match self.index_buffers.get(&renderable_vertex.get_index_id()) {
                Some(indices) => indices,
                None => return Err(RendererVertexColorErr::Get("Self Index Buffers Get")),
            },
            &self.program,
            &uniform!(

            ),
            match self.draw_parameters.get(&renderable_vertex.get_draw_method_id()) {
                Some(dp) => dp,
                None => return Err(RendererVertexColorErr::Get("Self Draw Parameters Get")),
            }
        ) {
            Ok(()) => (),
            Err(err) => return Err(RendererVertexColorErr::Draw("Frame Draw", err)),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum RendererVertexColorErr {
    Get(&'static str),
    Draw(&'static str, DrawError),
    ProgramCreation(&'static str, ProgramCreationError),
    VertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    IndexBufferCreation(&'static str, glium::index::BufferCreationError),
}

impl fmt::Display for RendererVertexColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererVertexColorErr::Get(_) => write!(f, "Get was None"),
            RendererVertexColorErr::Draw(_, ref err) => err.fmt(f),
            RendererVertexColorErr::ProgramCreation(_, ref err) => err.fmt(f),
            RendererVertexColorErr::VertexBufferCreation(_, ref err) => err.fmt(f),
            RendererVertexColorErr::IndexBufferCreation(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RendererVertexColorErr {
    fn description(&self) -> &str {
        match *self {
            RendererVertexColorErr::Get(_) => "Get was None",
            RendererVertexColorErr::Draw(_, ref err) => err.description(),
            RendererVertexColorErr::ProgramCreation(_, ref err) => err.description(),
            RendererVertexColorErr::VertexBufferCreation(_, ref err) => err.description(),
            RendererVertexColorErr::IndexBufferCreation(_, ref err) => err.description(),
        }
    }
}
