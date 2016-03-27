use std::fmt;
use std::sync::{Arc};
use std::error::Error;

use logic::{Id, IdManager, IdType};
use graphics::{
    Window, WindowErr, Vertex, Index, DrawMethod, MatrixData, MatrixDataErr, ProgramPreset
};
use math::{Mat4};

struct Changes {
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<Index>>,
    texture: Option<&'static [u8]>,
    draw_method: Option<DrawMethod>,
    program: Option<ProgramPreset>,
    perspective: Option<(Mat4, Mat4)>,
    view: Option<(Mat4, Mat4)>,
    model: Option<(Mat4, Mat4)>,
    dirty_render: bool,
}

impl Changes {
    pub fn new() -> Changes {
        Changes {
            vertices: None,
            indices: None,
            texture: None,
            draw_method: None,
            program: None,
            perspective: None,
            view: None,
            model: None,
            dirty_render: false,
        }
    }

    pub fn new_from(other: &Changes) -> Changes {
        Changes {
            vertices: other.vertices.clone(),
            indices: other.indices.clone(),
            texture: other.texture,
            draw_method: other.draw_method.clone(),
            program: other.program,
            perspective: other.perspective,
            view: other.view,
            model: other.model,
            dirty_render: other.dirty_render,
        }
    }
}

pub struct Renderable {
    texture_id: Id,
    vertex_id: Id,
    index_id: Id,
    draw_method_id: Id,
    program_id: Id,
    perspective_id: Id,
    view_id: Id,
    model_id: Id,
    changes: Changes,
    active: bool,
}

impl Renderable {
    pub fn new(manager: &mut IdManager) -> Renderable {
        Renderable {
            vertex_id: Id::new(manager, IdType::Vertex),
            index_id: Id::new(manager, IdType::Index),
            texture_id: Id::new(manager, IdType::Texture),
            draw_method_id: Id::new(manager, IdType::DrawMethod),
            program_id: Id::new(manager, IdType::Program),
            perspective_id: Id::new(manager, IdType::Perspective),
            view_id: Id::new(manager, IdType::View),
            model_id: Id::new(manager, IdType::Model),
            changes: Changes::new(),
            active: true,
        }
    }

    pub fn new_from(other: Arc<Renderable>) -> Renderable {
        Renderable {
            vertex_id: other.vertex_id,
            index_id: other.index_id,
            texture_id: other.texture_id,
            draw_method_id: other.draw_method_id,
            program_id: other.program_id,
            perspective_id: other.perspective_id,
            view_id: other.view_id,
            model_id: other.model_id,
            changes: Changes::new_from(&other.changes),
            active: other.active,
        }
    }

    pub fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), RenderableErr> {
        if self.changes.dirty_render {
            match self.changes.vertices.clone() {
                Some(vertices) => {
                    match window.set_vertices(self.vertex_id, vertices) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::Window("Window Set Vertices",err)),
                    }
                },
                None => (),
            }
            match self.changes.indices.clone() {
                Some(indices) => {
                    match window.set_indices(self.index_id, indices) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::Window("Window Set Indices", err)),
                    }
                },
                None => (),
            }
            match self.changes.texture {
                Some(texture) => {
                    match window.set_texture(self.texture_id, texture) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::Window("Window Set Texture", err)),
                    }
                },
                None => (),
            }
            match self.changes.draw_method.clone() {
                Some(draw_method) => {
                    window.set_draw_method(self.draw_method_id, draw_method);
                },
                None => (),
            }
            match self.changes.program {
                Some(program) => {
                    match window.get_program_preset(program) {
                        Ok(id) => self.set_program_id(id),
                        Err(err) => return Err(RenderableErr::Window("Window Get Program Preset Program", err)),
                    }
                },
                None => (),
            }
            match self.changes.perspective.clone() {
                Some(perspective) => {
                    match matrix_data.set_perspective_matrix(self.perspective_id, perspective.0, perspective.1) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::MatrixData("MatrixData Set Perspective Matrix", err)),
                    }
                },
                None => (),
            }
            match self.changes.view.clone() {
                Some(view) => {
                    match matrix_data.set_view_matrix(self.view_id, view.0, view.1) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::MatrixData("MatrixData Set View Matrix", err)),
                    }
                },
                None => (),
            }
            match self.changes.model.clone() {
                Some(model) => {
                    match matrix_data.set_model_matrix(self.model_id, model.0, model.1) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::MatrixData("MatrixData Set Model matrix", err)),
                    }
                },
                None => (),
            }
            self.changes.vertices = None;
            self.changes.indices = None;
            self.changes.texture = None;
            self.changes.draw_method = None;
            self.changes.perspective = None;
            self.changes.view = None;
            self.changes.model = None;
            self.changes.dirty_render = false;
        }
        Ok(())
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        self.changes.vertices = Some(vertices);
        self.changes.dirty_render = true;
    }

    pub fn set_indices(&mut self, indices: Vec<Index>) {
        self.changes.indices = Some(indices);
        self.changes.dirty_render = true;
    }

    pub fn set_texture(&mut self, texture: &'static [u8]) {
        self.changes.texture = Some(texture);
        self.changes.dirty_render = true;
    }

    pub fn set_draw_method(&mut self, draw_method: DrawMethod) {
        self.changes.draw_method = Some(draw_method);
        self.changes.dirty_render = true;
    }

    pub fn set_program(&mut self, program_preset: ProgramPreset) {
        self.changes.program = Some(program_preset);
        self.changes.dirty_render = true;
    }

    pub fn set_perspective(&mut self, matrix: Mat4) {
        self.changes.perspective = Some((matrix, matrix.to_inverse()));
        self.changes.dirty_render = true;
    }

    pub fn set_view(&mut self, matrix: Mat4) {
        self.changes.view = Some((matrix, matrix.to_inverse()));
        self.changes.dirty_render = true;
    }

    pub fn set_model(&mut self, matrix: Mat4) {
        self.changes.model = Some((matrix, matrix.to_inverse()));
        self.changes.dirty_render = true;
    }

    pub fn set_vertex_id(&mut self, id: Id) {
        self.vertex_id = id;
    }

    pub fn set_index_id(&mut self, id: Id) {
        self.index_id = id;
    }

    pub fn set_texture_id(&mut self, id: Id) {
        self.texture_id = id;
    }

    pub fn set_draw_method_id(&mut self, id: Id) {
        self.draw_method_id = id;
    }

    pub fn set_program_id(&mut self, id: Id) {
        self.program_id = id;
    }

    pub fn set_perspective_id(&mut self, id: Id) {
        self.perspective_id = id;
    }

    pub fn set_view_id(&mut self, id: Id) {
        self.view_id = id;
    }

    pub fn set_model_id(&mut self, id: Id) {
        self.model_id = id;
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn get_vertex_id(&self) -> Id {
        self.vertex_id
    }

    pub fn get_index_id(&self) -> Id {
        self.index_id
    }

    pub fn get_texture_id(&self) -> Id {
        self.texture_id
    }

    pub fn get_draw_method_id(&self) -> Id {
        self.draw_method_id
    }

    pub fn get_program_id(&self) -> Id {
        self.program_id
    }

    pub fn get_perspective_id(&self) -> Id {
        self.perspective_id
    }

    pub fn get_view_id(&self) -> Id {
        self.view_id
    }

    pub fn get_model_id(&self) -> Id {
        self.model_id
    }
}

#[derive(Debug)]
pub enum RenderableErr {
    Poison(&'static str),
    MatrixData(&'static str, MatrixDataErr),
    Window(&'static str, WindowErr),
}

impl fmt::Display for RenderableErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderableErr::Poison(_) => write!(f, "Thread was Poisoned During R/W"),
            RenderableErr::MatrixData(_, ref err) => err.fmt(f),
            RenderableErr::Window(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RenderableErr {
    fn description(&self) -> &str {
        match *self {
            RenderableErr::Poison(_) => "Thread was Poisoned",
            RenderableErr::MatrixData(_, ref err) => err.description(),
            RenderableErr::Window(_, ref err) => err.description(),
        }
    }
}
