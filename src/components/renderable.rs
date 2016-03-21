use logic::{Id, IdManager, IdType, WorldErr};
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
    pub fn new(manager: &mut IdManager) -> Renderable {
        Renderable {
            texture_id: Id::new(manager, IdType::Texture),
            vertex_id: Id::new(manager, IdType::Vertex),
            index_id: Id::new(manager, IdType::Index),
            draw_method_id: Id::new(manager, IdType::DrawMethod),
            perspective_id: Id::new(manager, IdType::Perspective),
            view_id: Id::new(manager, IdType::View),
            model_id: Id::new(manager, IdType::Model),
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

    pub fn render(&mut self, window: &mut Window) {
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

    pub fn with_new_vertices(mut self, vertices: Vec<Vertex>, manager: &mut IdManager) -> Renderable {
        self.set_new_vertices(vertices, manager);
        self
    }

    pub fn with_new_indices(mut self, indices: Vec<Index>, manager: &mut IdManager) -> Renderable {
        self.set_new_indices(indices, manager);
        self
    }

    pub fn with_new_texture(mut self, texture: &'static [u8], manager: &mut IdManager) -> Renderable {
        self.set_new_texture(texture, manager);
        self
    }

    pub fn with_new_draw_method(mut self, draw_method: DrawMethod, manager: &mut IdManager) -> Renderable {
        self.set_new_draw_method(draw_method, manager);
        self
    }

    pub fn with_new_perspective(mut self, matrix: Mat4, manager: &mut IdManager) -> Renderable {
        self.set_new_perspective(matrix, manager);
        self
    }

    pub fn with_new_view(mut self, matrix: Mat4, manager: &mut IdManager) -> Renderable {
        self.set_new_view(matrix, manager);
        self
    }

    pub fn with_new_model(mut self, matrix: Mat4, manager: &mut IdManager) -> Renderable {
        self.set_new_model(matrix, manager);
        self
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Renderable {
        self.set_vertices(vertices);
        self
    }

    pub fn with_indices(mut self, indices: Vec<Index>) -> Renderable {
        self.set_indices(indices);
        self
    }

    pub fn with_texture(mut self, texture: &'static [u8]) -> Renderable {
        self.set_texture(texture);
        self
    }

    pub fn with_draw_method(mut self, draw_method: DrawMethod) -> Renderable {
        self.set_draw_method(draw_method);
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

    pub fn set_new_vertices(&mut self, vertices: Vec<Vertex>, manager: &mut IdManager) {
        self.set_vertex_id(Id::new(manager, IdType::Vertex));
        self.set_vertices(vertices);
    }

    pub fn set_new_indices(&mut self, indices: Vec<Index>, manager: &mut IdManager) {
        self.set_index_id(Id::new(manager, IdType::Index));
        self.set_indices(indices);
    }

    pub fn set_new_texture(&mut self, texture: &'static [u8], manager: &mut IdManager) {
        self.set_texture_id(Id::new(manager, IdType::Texture));
        self.set_texture(texture);
    }

    pub fn set_new_draw_method(&mut self, draw_method: DrawMethod, manager: &mut IdManager) {
        self.set_draw_method_id(Id::new(manager, IdType::DrawMethod));
        self.set_draw_method(draw_method);
    }

    pub fn set_new_perspective(&mut self, matrix: Mat4, manager: &mut IdManager) {
        self.set_perspective_id(Id::new(manager, IdType::Perspective));
        self.set_perspective(matrix);
    }

    pub fn set_new_view(&mut self, matrix: Mat4, manager: &mut IdManager) {
        self.set_view_id(Id::new(manager, IdType::View));
        self.set_view(matrix);
    }

    pub fn set_new_model(&mut self, matrix: Mat4, manager: &mut IdManager) {
        self.set_model_id(Id::new(manager, IdType::Model));
        self.set_model(matrix);
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

pub enum RenderableErr {
    RenderSync(WorldErr),
}
