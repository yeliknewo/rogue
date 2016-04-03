use std::sync::{Arc};
use std::error::Error;
use std::fmt;
use std::collections::{HashMap};
use glium::Frame as GliumFrame;
use glium::{Surface, DrawError, VertexBuffer, IndexBuffer, DrawParameters, Program, ProgramCreationError};
use glium;

use components::{Renderable};
use logic::{Id};
use graphics2::{MatrixData, Window};
use graphics2::solid_color::{Vertex, init_vertex, Index};

pub struct RendererSolidColor {
    vertex_buffers: HashMap<Id, VertexBuffer<Vertex>>,
    index_buffers: HashMap<Id, IndexBuffer<Index>>,
    draw_parameters: HashMap<Id, DrawParameters<'static>>,
    program: Program,
}

impl RendererSolidColor {
    pub fn new(window: &mut Window) -> Result<RendererSolidColor, RendererSolidColorErr> {
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
            RendererSolidColor {
                vertex_buffers: HashMap::new(),
                index_buffers: HashMap::new(),
                draw_parameters: HashMap::new(),
                program: match Program::from_source(window.get_facade(), vertex_shader_src, fragment_shader_src, None) {
                    Ok(program) => program,
                    Err(err) => return Err(RendererSolidColorErr::ProgramCreation("Program From Source", err)),
                },
            }
        )
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, matrix_data: &MatrixData) -> Result<(), RendererSolidColorErr> {
        let renderable_solid = match renderable.get_solid_color() {
            Some(renderable) => renderable,
            None => return Err(RendererSolidColorErr::Get("Renderable Get Solid Color")),
        };
        match frame.draw(
            match self.vertex_buffers.get(&renderable_solid.get_vertex_id()) {
                Some(vertices) => vertices,
                None => return Err(RendererSolidColorErr::Get("Self Vertex Buffers Get")),
            },
            match self.index_buffers.get(&renderable_solid.get_index_id()) {
                Some(indices) => indices,
                None => return Err(RendererSolidColorErr::Get("Self Index Buffers Get")),
            },
            &self.program,
            &uniform!(

            ),
            match self.draw_parameters.get(&renderable_solid.get_draw_method_id()) {
                Some(dp) => dp,
                None => return Err(RendererSolidColorErr::Get("Self Draw Parameters Get")),
            }
        ) {
            Ok(()) => (),
            Err(err) => return Err(RendererSolidColorErr::Draw("Frame Draw", err)),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum RendererSolidColorErr {
    Get(&'static str),
    Draw(&'static str, DrawError),
    ProgramCreation(&'static str, ProgramCreationError),
    VertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    IndexBufferCreation(&'static str, glium::index::BufferCreationError),
}

impl fmt::Display for RendererSolidColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererSolidColorErr::Get(_) => write!(f, "Get was None"),
            RendererSolidColorErr::Draw(_, ref err) => err.fmt(f),
            RendererSolidColorErr::ProgramCreation(_, ref err) => err.fmt(f),
            RendererSolidColorErr::VertexBufferCreation(_, ref err) => err.fmt(f),
            RendererSolidColorErr::IndexBufferCreation(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RendererSolidColorErr {
    fn description(&self) -> &str {
        match *self {
            RendererSolidColorErr::Get(_) => "Get was None",
            RendererSolidColorErr::Draw(_, ref err) => err.description(),
            RendererSolidColorErr::ProgramCreation(_, ref err) => err.description(),
            RendererSolidColorErr::VertexBufferCreation(_, ref err) => err.description(),
            RendererSolidColorErr::IndexBufferCreation(_, ref err) => err.description(),
        }
    }
}
