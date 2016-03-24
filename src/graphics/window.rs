use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::texture::texture2d::{Texture2d};
use glium::texture::{RawImage2d};
use glium::glutin::{WindowBuilder, get_primary_monitor, CreationError};
use glium::{Surface, DisplayBuild, Program, VertexBuffer, IndexBuffer, DrawParameters, GliumCreationError, SwapBuffersError, DrawError, ProgramCreationError};
use glium;
use image::{load_from_memory, ImageError};
use std::collections::{HashMap};
use std::sync::{Arc};
use std::fmt;
use std::error::Error;

use math::{Vec2};
use logic::{Id};
use graphics::{Vertex, Index, MatrixData, MatrixDataErr, DrawMethod, method_to_parameters};
use components::{Renderable};

pub struct Window {
    facade: GlutinFacade,
    program: Program,
    texture_buffers: HashMap<Id, Texture2d>,
    vertex_buffers: HashMap<Id, VertexBuffer<Vertex>>,
    index_buffers: HashMap<Id, IndexBuffer<Index>>,
    draw_parameters: HashMap<Id, DrawParameters<'static>>,
    resolution: (u32, u32),
}

impl Window {
    pub fn new(args: WindowArgs) -> Result<Window, WindowErr> {
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

        let resolution: (u32, u32) = get_primary_monitor().get_dimensions();

        let facade = match args {
            WindowArgs::Windowed(width, height, title) => {
                let facade = match WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(width, height)
                    .with_decorations(true)
                    .with_depth_buffer(24)
                    .with_vsync()
                    .build_glium() {
                        Ok(facade) => facade,
                        Err(err) => return Err(WindowErr::GliumCreation("WindowBuilder Build Glium", err)),
                    };
                match facade.get_window() {
                    Some(window) => window,
                    None => return Err(WindowErr::Get("Facade Get Window")),
                }.set_position(((resolution.0 - width) / 2) as i32, ((resolution.1 - height) / 2) as i32);
                facade
            },
            WindowArgs::Borderless(title) => {
                let facade = match WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(resolution.0, resolution.1)
                    .with_decorations(false)
                    .with_depth_buffer(24)
                    .with_vsync()
                    .build_glium() {
                        Ok(facade) => facade,
                        Err(err) => return Err(WindowErr::GliumCreation("WindowBuilder Build Glium", err)),
                    };
                match facade.get_window() {
                    Some(window) => window,
                    None => return Err(WindowErr::Get("Facade Get Window")),
                }.set_position(0, 0);
                facade
            },
        };
        Ok(
            Window {
                program: match Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None) {
                    Ok(program) => program,
                    Err(err) => return Err(WindowErr::ProgramCreation("Program From Source", err)),
                },
                facade: facade,
                texture_buffers: HashMap::new(),
                vertex_buffers: HashMap::new(),
                index_buffers: HashMap::new(),
                draw_parameters: HashMap::new(),
                resolution: resolution,
            }
        )
    }

    pub fn get_resolution_vec2(&self) -> Vec2 {
        Vec2::from([self.resolution.0 as f32, self.resolution.1 as f32])
    }

    pub fn frame(&mut self) -> Frame {
        Frame::new(&mut self.facade, &mut self.program, &mut self.texture_buffers, &mut self.vertex_buffers, &mut self.index_buffers, &mut self.draw_parameters)
    }

    pub fn poll_events(&self) -> PollEventsIter {
        self.facade.poll_events()
    }

    pub fn set_vertices(&mut self, id: Id, vertices: Vec<Vertex>) -> Result<(), WindowErr> {
        self.vertex_buffers.insert(id, match VertexBuffer::new(&self.facade, &vertices) {
            Ok(buffer) => buffer,
            Err(err) => return Err(WindowErr::VertexBufferCreation("VertexBuffer New", err)),
        });
        Ok(())
    }

    pub fn set_indices(&mut self, id: Id, indices: Vec<Index>) -> Result<(), WindowErr> {
        self.index_buffers.insert(id, match IndexBuffer::new(&self.facade, glium::index::PrimitiveType::TrianglesList, &indices) {
            Ok(buffer) => buffer,
            Err(err) => return Err(WindowErr::IndexBufferCreation("IndexBuffer New", err)),
        });
        Ok(())
    }

    pub fn set_texture(&mut self, id: Id, data: &[u8]) -> Result<(), WindowErr> {
        let texture = match load_from_memory(data) {
            Ok(texture) => texture,
            Err(err) => return Err(WindowErr::Image("Load From Memory Data", err)),
        }.to_rgba();
        self.texture_buffers.insert(id, match Texture2d::new(&self.facade, RawImage2d::from_raw_rgba_reversed(texture.clone().into_raw(), texture.dimensions())) {
            Ok(texture) => texture,
            Err(err) => return Err(WindowErr::TextureCreation("Texture2d New", err)),
        });
        Ok(())
    }

    pub fn set_draw_method(&mut self, id: Id, draw_method: DrawMethod) {
        self.draw_parameters.insert(id, method_to_parameters(draw_method));
    }
}

#[derive(Debug)]
pub enum WindowErr {
    Get(&'static str),
    VertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    IndexBufferCreation(&'static str, glium::index::BufferCreationError),
    GliumCreation(&'static str, GliumCreationError<CreationError>),
    ProgramCreation(&'static str, ProgramCreationError),
    TextureCreation(&'static str, glium::texture::TextureCreationError),
    Image(&'static str, ImageError),
}

impl fmt::Display for WindowErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindowErr::Get(_) => write!(f, "Get was None"),
            WindowErr::VertexBufferCreation(_, ref err) => err.fmt(f),
            WindowErr::IndexBufferCreation(_, ref err) => err.fmt(f),
            WindowErr::GliumCreation(_, ref err) => err.fmt(f),
            WindowErr::ProgramCreation(_, ref err) => err.fmt(f),
            WindowErr::TextureCreation(_, ref err) => err.fmt(f),
            WindowErr::Image(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for WindowErr {
    fn description(&self) -> &str {
        match *self {
            WindowErr::Get(_) => "Get was None",
            WindowErr::VertexBufferCreation(_, ref err) => err.description(),
            WindowErr::IndexBufferCreation(_, ref err) => err.description(),
            WindowErr::GliumCreation(_, ref err) => err.description(),
            WindowErr::ProgramCreation(_, ref err) => err.description(),
            WindowErr::TextureCreation(_, ref err) => err.description(),
            WindowErr::Image(_, ref err) => err.description(),
        }
    }
}


pub enum WindowArgs {
    Windowed(u32, u32, String),
    Borderless(String),
}

pub struct Frame<'a> {
    program: &'a mut Program,
    texture_buffers: &'a mut HashMap<Id, Texture2d>,
    vertex_buffers: &'a mut HashMap<Id, VertexBuffer<Vertex>>,
    index_buffers: &'a mut HashMap<Id, IndexBuffer<Index>>,
    draw_parameters: &'a mut HashMap<Id, DrawParameters<'static>>,
    frame: glium::Frame,
}

impl<'a> Frame<'a> {
    fn new(
        facade: &'a mut GlutinFacade,
        program: &'a mut Program,
        texture_buffers: &'a mut HashMap<Id, Texture2d>,
        vertex_buffers: &'a mut HashMap<Id, VertexBuffer<Vertex>>,
        index_buffers: &'a mut HashMap<Id, IndexBuffer<Index>>,
        draw_parameters: &'a mut HashMap<Id, DrawParameters<'static>>,
    ) -> Frame<'a> {
        let mut frame = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            program: program,
            texture_buffers: texture_buffers,
            vertex_buffers: vertex_buffers,
            index_buffers: index_buffers,
            draw_parameters: draw_parameters,
        }
    }

    pub fn draw_entity(&mut self, entity: Arc<Renderable>, matrix_data: Arc<MatrixData>) -> Result<(), FrameErr> {
        match self.frame.draw(
            match self.vertex_buffers.get(&entity.get_vertex_id()) {
                Some(vertices) => vertices,
                None => return Err(FrameErr::Get("Self VertexBuffers Get")),
            },
            match self.index_buffers.get(&entity.get_index_id()) {
                Some(indices) => indices,
                None => return Err(FrameErr::Get("Self IndexBuffers Get")),
            },
            &self.program,
            &uniform!(
                tex: match self.texture_buffers.get(&entity.get_texture_id()) {
                    Some(texture) => texture,
                    None => return Err(FrameErr::Get("Self TextureBuffers Get")),
                },
                perspective: match matrix_data.get_perspective_matrix(&entity) {
                    Ok(matrix) => matrix,
                    Err(err) => return Err(FrameErr::MatrixData("MatrixData Get Perspective Matrix", err)),
                },
                view: match matrix_data.get_view_matrix(&entity) {
                    Ok(matrix) => matrix,
                    Err(err) => return Err(FrameErr::MatrixData("MatrixData Get View Matrix", err)),
                },
                model: match matrix_data.get_model_matrix(&entity) {
                    Ok(matrix) => matrix,
                    Err(err) => return Err(FrameErr::MatrixData("MatrixData Get Model Matrix", err)),
                },
            ),
            match self.draw_parameters.get(&entity.get_draw_method_id()) {
                Some(dp) => dp,
                None => return Err(FrameErr::Get("Self Draw Parameters Get")),
            }
        ) {
            Ok(()) => Ok(()),
            Err(err) => Err(FrameErr::Draw("Self Frame Draw", err)),
        }
    }

    pub fn end(self) -> Result<(), FrameErr> {
        match self.frame.finish() {
            Ok(()) => Ok(()),
            Err(err) => Err(FrameErr::SwapBuffers("Self Frame Finish", err)),
        }
    }
}


#[derive(Debug)]
pub enum FrameErr {
    Get(&'static str),
    MatrixData(&'static str, MatrixDataErr),
    SwapBuffers(&'static str, SwapBuffersError),
    Draw(&'static str, DrawError),
}

impl fmt::Display for FrameErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FrameErr::Get(_) => write!(f, "Get was None"),
            FrameErr::MatrixData(_, ref err) => err.fmt(f),
            FrameErr::SwapBuffers(_, ref err) => err.fmt(f),
            FrameErr::Draw(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for FrameErr {
    fn description(&self) -> &str {
        match *self {
            FrameErr::Get(_) => "Get was None",
            FrameErr::MatrixData(_, ref err) => err.description(),
            FrameErr::SwapBuffers(_, ref err) => err.description(),
            FrameErr::Draw(_, ref err) => err.description(),
        }
    }
}
