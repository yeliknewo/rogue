use std::fmt;
use std::sync::{Arc, RwLock};

use logic::{Id, IdManager, IdType};
use graphics::{Window, Vertex, Index, DrawMethod, MatrixData, MatrixDataErr};
use math::{Mat4};

struct Changes {
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<Index>>,
    texture: Option<&'static [u8]>,
    draw_method: Option<DrawMethod>,
    perspective: Option<(Mat4, Mat4)>,
    view: Option<(Mat4, Mat4)>,
    model: Option<(Mat4, Mat4)>,
    dirty: bool,
    dirty_sync: bool,
}

impl Changes {
    pub fn new() -> Changes {
        Changes {
            vertices: None,
            indices: None,
            texture: None,
            draw_method: None,
            perspective: None,
            view: None,
            model: None,
            dirty: false,
            dirty_sync: false,
        }
    }
}

#[derive(Clone)]
pub struct Renderable {
    texture_id: Id,
    vertex_id: Id,
    index_id: Id,
    draw_method_id: Id,
    perspective_id: Id,
    view_id: Id,
    model_id: Id,
    changes: Arc<RwLock<Changes>>,
}

impl Renderable {
    pub fn new(manager: &mut IdManager) -> Renderable {
        Renderable {
            texture_id: Id::new(manager, IdType::Texture),
            vertex_id: Id::new(manager, IdType::Vertex),
            index_id: Id::new(manager, IdType::Index),
            draw_method_id: Id::new(manager, IdType::DrawMethod),
            perspective_id: Id::new(manager, IdType::Perspective),
            view_id: Id::new(manager, IdType::View),
            model_id: Id::new(manager, IdType::Model),
            changes: Arc::new(RwLock::new(Changes::new())),
        }
    }

    pub fn render(&mut self, window: &mut Window, matrix_data: &mut MatrixData) -> Result<(), RenderableErr> {
        if match self.changes.read(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::Render("Unable to Read Changes")),
        }.dirty {
            match {
                match self.changes.read() {
                    Ok(changes) => changes,
                    Err(_) => return Err(RenderableErr::Render("Unable to Read Changes")),
                }
            }.vertices.clone() {
                Some(vertices) => {
                    window.set_vertices(self.vertex_id, vertices);
                    match self.changes.write() {
                        Ok(changes) => changes,
                        Err(_) => return Err(RenderableErr::Render("Unable to Write Changes")),
                    }.vertices = None;
                },
                None => (),
            }
            match {
                match self.changes.read() {
                    Ok(changes) => changes,
                    Err(_) => return Err(RenderableErr::Render("Unable to Read Changes")),
                }
            }.indices.clone() {
                Some(indices) => {
                    window.set_indices(self.index_id, indices);
                    match self.changes.write() {
                        Ok(changes) => changes,
                        Err(_) => return Err(RenderableErr::Render("Unable to Write Changes")),
                    }.indices = None;
                },
                None => (),
            }
            match {
                match self.changes.read() {
                    Ok(changes) => changes,
                    Err(_) => return Err(RenderableErr::Render("Unable to Read Changes")),
                }
            }.texture {
                Some(texture) => {
                    window.set_texture(self.texture_id, texture);
                    match self.changes.write() {
                        Ok(changes) => changes,
                        Err(_) => return Err(RenderableErr::Render("Unable to Write Changes")),
                    }.texture = None;
                },
                None => (),
            }
            match {
                match self.changes.read() {
                    Ok(changes) => changes,
                    Err(_) => return Err(RenderableErr::Render("Unable to Read Changes")),
                }
            }.draw_method.clone() {
                Some(draw_method) => {
                    window.set_draw_method(self.draw_method_id, draw_method);
                    match self.changes.write() {
                        Ok(changes) => changes,
                        Err(_) => return Err(RenderableErr::Render("Unable to Write Changes")),
                    }.draw_method = None;
                },
                None => (),
            }
            match {
                match self.changes.read() {
                    Ok(changes) => changes,
                    Err(_) => return Err(RenderableErr::Render("Unable to Read Changes")),
                }
            }.perspective.clone() {
                Some(perspective) => {
                    match matrix_data.set_perspective_matrix(self.perspective_id, perspective.0, perspective.1) {
                        Ok(()) => (),
                        Err(err) => return Err(RenderableErr::RenderMatrixData(err)),
                    }
                },
                None => (),
            }
            match self.changes.write() {
                Ok(changes) => changes,
                Err(_) => return Err(RenderableErr::Render("Unable to Write Changes")),
            }.dirty = false;
        }
        Ok(())
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetVertices("Unable to Write Changes")),
        }.vertices = Some(vertices);
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetVertices("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    pub fn set_indices(&mut self, indices: Vec<Index>) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetIndices("Unable to Write Changes")),
        }.indices = Some(indices);
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetIndices("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    pub fn set_texture(&mut self, texture: &'static [u8]) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetTexture("Unable to Write Changes")),
        }.texture = Some(texture);
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetTexture("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    pub fn set_draw_method(&mut self, draw_method: DrawMethod) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetDrawMethod("Unable to Write Changes")),
        }.draw_method = Some(draw_method);
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetDrawMethod("Unable to Write Changes")),
        }.dirty = true;
        Ok(())
    }

    pub fn set_perspective(&mut self, matrix: Mat4) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetPerspective("Unable to Write Changes")),
        }.perspective = Some((matrix, matrix.to_inverse()));
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetPerspective("Unable to Write Changes")),
        }.dirty_sync = true;
        Ok(())
    }

    pub fn set_view(&mut self, matrix: Mat4) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetView("Unable to Write Changes")),
        }.view = Some((matrix, matrix.to_inverse()));
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetView("Unable to Write Changes")),
        }.dirty_sync = true;
        Ok(())
    }

    pub fn set_model(&mut self, matrix: Mat4) -> Result<(), RenderableErr> {
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetModel("Unable to Write Changes")),
        }.model = Some((matrix, matrix.to_inverse()));
        match self.changes.write(){
            Ok(changes) => changes,
            Err(_) => return Err(RenderableErr::SetModel("Unable to Write Changes")),
        }.dirty_sync = true;
        Ok(())
    }

    pub fn with_vertex_id(mut self, id: Id) -> Renderable {
        self.set_vertex_id(id);
        self
    }

    pub fn with_index_id(mut self, id: Id) -> Renderable {
        self.set_index_id(id);
        self
    }

    pub fn with_texture_id(mut self, id: Id) -> Renderable {
        self.set_texture_id(id);
        self
    }

    pub fn with_draw_method_id(mut self, id: Id) -> Renderable {
        self.set_draw_method_id(id);
        self
    }

    pub fn with_perspective_id(mut self, id: Id) -> Renderable {
        self.set_perspective_id(id);
        self
    }

    pub fn with_view_id(mut self, id: Id) -> Renderable {
        self.set_view_id(id);
        self
    }

    pub fn with_model_id(mut self, id: Id) -> Renderable {
        self.set_model_id(id);
        self
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

    pub fn set_perspective_id(&mut self, id: Id) {
        self.perspective_id = id;
    }

    pub fn set_view_id(&mut self, id: Id) {
        self.view_id = id;
    }

    pub fn set_model_id(&mut self, id: Id) {
        self.model_id = id;
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

pub enum RenderableErr {
    Render(&'static str),
    RenderMatrixData(MatrixDataErr),
    SetVertices(&'static str),
    SetIndices(&'static str),
    SetTexture(&'static str),
    SetDrawMethod(&'static str),
    SetPerspective(&'static str),
    SetView(&'static str),
    SetModel(&'static str),
}

impl fmt::Display for RenderableErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RenderableErr::Render(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::RenderMatrixData(ref err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetVertices(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetIndices(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetTexture(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetDrawMethod(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetPerspective(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetView(err) => {
                write!(f, "{}", err);
            },
            &RenderableErr::SetModel(err) => {
                write!(f, "{}", err);
            },
        }
        Ok(())
    }
}
