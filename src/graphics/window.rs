use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::texture::texture2d::{Texture2d};
use glium::texture::{RawImage2d};
use glium::glutin::{WindowBuilder, get_primary_monitor};
use glium::{Surface, DisplayBuild, Program, VertexBuffer, IndexBuffer, DrawParameters};
use glium;
use image::{load_from_memory};
use std::collections::{HashMap};
use std::sync::{Arc, RwLock};

use math::{Vec2};
use logic::{Id};
use graphics::{Vertex, Index, Transforms, DrawMethod, method_to_parameters};
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
    pub fn new(args: WindowArgs) -> Window {
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
                let facade = WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(width, height)
                    .with_decorations(true)
                    .with_depth_buffer(24)
                    .with_vsync()
                    .build_glium()
                    .expect("Unable to make Facade");
                facade.get_window()
                    .expect("Unable to find the Window")
                    .set_position(((resolution.0 - width) / 2) as i32, ((resolution.1 - height) / 2) as i32);
                facade
            },
            WindowArgs::Borderless(title) => {
                let facade = WindowBuilder::new()
                    .with_title(title)
                    .with_dimensions(resolution.0, resolution.1)
                    .with_decorations(false)
                    .with_depth_buffer(24)
                    .with_vsync()
                    .build_glium()
                    .expect("Unable to make Facade");
                facade.get_window()
                    .expect("Unable to find Window")
                    .set_position(0, 0);
                facade
            },
        };
        Window {
            program: Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None).expect("Unable to make Shader Program"),
            facade: facade,
            texture_buffers: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            draw_parameters: HashMap::new(),
            resolution: resolution,
        }
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

    pub fn set_vertices(&mut self, id: Id, vertices: Vec<Vertex>) {
        self.vertex_buffers.insert(id, VertexBuffer::new(&self.facade, &vertices).expect("Failed to Create Vertex Buffer"));
    }

    pub fn set_indices(&mut self, id: Id, indices: Vec<Index>) {
        self.index_buffers.insert(id, IndexBuffer::new(&self.facade, glium::index::PrimitiveType::TrianglesList, &indices).expect("Failed to Create Index Buffer"));
    }

    pub fn set_texture(&mut self, id: Id, data: &[u8]) {
        let texture = load_from_memory(data).expect("Error Loading Image").to_rgba();
        self.texture_buffers.insert(id, Texture2d::new(&self.facade, RawImage2d::from_raw_rgba_reversed(texture.clone().into_raw(), texture.dimensions())).expect("Unable to make Texture"));
    }

    pub fn set_draw_method(&mut self, id: Id, draw_method: DrawMethod) {
        self.draw_parameters.insert(id, method_to_parameters(draw_method));
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

    pub fn draw_entity(&mut self, entity_arc: Arc<RwLock<Renderable>>, transforms: Arc<RwLock<Transforms>>) {
        let entity = entity_arc.read().expect("Unable to Read Entity in Draw Entity");
        self.frame.draw(
            self.vertex_buffers.get(&entity.get_vertex_id()).expect("Unable to Get Vertex Buffer in Draw Entity"),
            self.index_buffers.get(&entity.get_index_id()).expect("Unable to Get Index Buffer in Draw Entity"),
            &self.program,
            &uniform!(
                tex: self.texture_buffers.get(&entity.get_texture_id()).expect("Unable to Get Texture Buffer in Draw Entity"),
                perspective: transforms.read().expect("Unable to Read Transforms in Draw Entity in Frame").get_perspective_matrix(&entity),
                view: transforms.read().expect("Unable to Read Transforms in Draw Entity In Frame").get_view_matrix(&entity),
                model: transforms.read().expect("Unable to Read Transforms in Draw Entity in Frame").get_model_matrix(&entity),
            ),
            self.draw_parameters.get(&entity.get_draw_method_id()).expect("Unable to Get Draw Method in Draw Entity"))
            .expect("Unable to draw Entity");
    }

    pub fn end(self) {
        self.frame.finish().expect("Unable to Finish Frame");
    }
}
