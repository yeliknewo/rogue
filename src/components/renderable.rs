use std::sync::{Arc, RwLock};

use logic::{Id, IdManager, IdType, World, EntityData};
use graphics::{Window, Vertex, Index, DrawMethod};
use math::{Mat4};

#[derive(Clone)]
pub struct Renderable {
    texture_id: Id,
    vertex_id: Id,
    index_id: Id,
    draw_method_id: Id,
    perspective_id: Id,
    view_id: Id,
    model_id: Id,
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

impl Renderable {
    pub fn new(manager: Arc<RwLock<IdManager>>) -> Renderable {
        Renderable {
            texture_id: Id::new(manager.clone(), IdType::Texture),
            vertex_id: Id::new(manager.clone(), IdType::Vertex),
            index_id: Id::new(manager.clone(), IdType::Index),
            draw_method_id: Id::new(manager.clone(), IdType::DrawMethod),
            perspective_id: Id::new(manager.clone(), IdType::Perspective),
            view_id: Id::new(manager.clone(), IdType::View),
            model_id: Id::new(manager.clone(), IdType::Model),
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

    pub fn render_sync<T: EntityData<T>>(&mut self, world: Arc<World<T>>) {
        if self.dirty_sync {
            match self.perspective.clone() {
                Some(perspective) => {
                    let transforms = world.get_transforms();
                    transforms.read().expect("Unable to Read Transforms in Render in Renderable").set_perspective_matrix(self.perspective_id, perspective.0, perspective.1);
                    self.perspective = None;
                },
                None => (),
            }
            match self.view.clone() {
                Some(view) => {
                    let transforms = world.get_transforms();
                    transforms.read().expect("Unable to Read Transforms in Render in Renderable").set_view_matrix(self.view_id, view.0, view.1);
                    self.view = None;
                },
                None => (),
            }
            match self.model.clone() {
                Some(model) => {
                    let transforms = world.get_transforms();
                    transforms.read().expect("Unable to Read Transforms in Render in Renderable").set_model_matrix(self.model_id, model.0, model.1);
                    self.model = None;
                },
                None => (),
            }
            self.dirty_sync = false;
        }
    }

    pub fn render<T: EntityData<T>>(&mut self, window: &mut Window, world: Arc<World<T>>) {
        if self.dirty {
            match self.vertices.clone() {
                Some(vertices) => {
                    window.set_vertices(self.vertex_id, vertices);
                    self.vertices = None;
                },
                None => (),
            }
            match self.indices.clone() {
                Some(indices) => {
                    window.set_indices(self.index_id, indices);
                    self.indices = None;
                },
                None => (),
            }
            match self.texture {
                Some(texture) => {
                    window.set_texture(self.texture_id, texture);
                    self.texture = None;
                },
                None => (),
            }
            match self.draw_method.clone() {
                Some(draw_method) => {
                    window.set_draw_method(self.draw_method_id, draw_method);
                    self.draw_method = None;
                },
                None => (),
            }
            self.dirty = false;
        }
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Renderable {
        self.vertices = Some(vertices);
        self
    }

    pub fn with_indices(mut self, indices: Vec<Index>) -> Renderable {
        self.indices = Some(indices);
        self
    }

    pub fn with_texture(mut self, texture: &'static [u8]) -> Renderable {
        self.texture = Some(texture);
        self
    }

    pub fn with_draw_method(mut self, draw_method: DrawMethod) -> Renderable {
        self.draw_method = Some(draw_method);
        self
    }

    pub fn with_perspective(mut self, matrix: Mat4) -> Renderable {
        self.set_perspective(matrix);
        self
    }

    pub fn with_view(mut self, matrix: Mat4) -> Renderable {
        self.set_view(matrix);
        self
    }

    pub fn with_model(mut self, matrix: Mat4) -> Renderable {
        self.set_model(matrix);
        self
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        self.vertices = Some(vertices);
        self.dirty = true;
    }

    pub fn set_indices(&mut self, indices: Vec<Index>) {
        self.indices = Some(indices);
        self.dirty = true;
    }

    pub fn set_texture(&mut self, texture: &'static [u8]) {
        self.texture = Some(texture);
        self.dirty = true;
    }

    pub fn set_draw_method(&mut self, draw_method: DrawMethod) {
        self.draw_method = Some(draw_method);
        self.dirty = true;
    }

    pub fn set_perspective(&mut self, matrix: Mat4) {
        self.perspective = Some((matrix, matrix.to_inverse()));
        self.dirty_sync = true;
    }

    pub fn set_view(&mut self, matrix: Mat4) {
        self.view = Some((matrix, matrix.to_inverse()));
        self.dirty_sync = true;
    }

    pub fn set_model(&mut self, matrix: Mat4) {
        self.model = Some((matrix, matrix.to_inverse()));
        self.dirty_sync = true;
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
