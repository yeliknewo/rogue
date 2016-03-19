use std::sync::{Arc, RwLock};

use logic::{ID, IDManager, IDType, World, EntityData};
use graphics::{Window, Vertex, Index, DrawMethod};
use math::{Mat4};

#[derive(Clone)]
pub struct Renderable {
    texture_id: ID,
    vertex_id: ID,
    index_id: ID,
    draw_method_id: ID,
    perspective_id: ID,
    view_id: ID,
    model_id: ID,
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<Index>>,
    texture: Option<&'static [u8]>,
    draw_method: Option<DrawMethod>,
    perspective: Option<(Mat4, Mat4)>,
    view: Option<(Mat4, Mat4)>,
    model: Option<(Mat4, Mat4)>,
}

impl Renderable {
    pub fn new(manager: Arc<RwLock<IDManager>>) -> Renderable {
        Renderable {
            texture_id: ID::new(manager.clone(), IDType::Texture),
            vertex_id: ID::new(manager.clone(), IDType::Vertex),
            index_id: ID::new(manager.clone(), IDType::Index),
            draw_method_id: ID::new(manager.clone(), IDType::DrawMethod),
            perspective_id: ID::new(manager.clone(), IDType::Perspective),
            view_id: ID::new(manager.clone(), IDType::View),
            model_id: ID::new(manager.clone(), IDType::Model),
            vertices: None,
            indices: None,
            texture: None,
            draw_method: None,
            perspective: None,
            view: None,
            model: None,
        }
    }

    pub fn new_from(renderable: &Renderable) -> Renderable {
        Renderable {
            vertex_id: renderable.vertex_id,
            index_id: renderable.index_id,
            texture_id: renderable.texture_id,
            draw_method_id: renderable.draw_method_id,
            perspective_id: renderable.perspective_id,
            view_id: renderable.view_id,
            model_id: renderable.model_id,
            vertices: None,
            indices: None,
            texture: None,
            draw_method: None,
            perspective: None,
            view: None,
            model: None,
        }
    }

    pub fn render<T: EntityData<T>>(&mut self, window: &mut Window, world: Arc<World<T>>) {
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
    }

    pub fn set_indices(&mut self, indices: Vec<Index>) {
        self.indices = Some(indices);
    }

    pub fn set_texture(&mut self, texture: &'static [u8]) {
        self.texture = Some(texture);
    }

    pub fn set_draw_method(&mut self, draw_method: DrawMethod) {
        self.draw_method = Some(draw_method);
    }

    pub fn set_perspective(&mut self, matrix: Mat4) {
        self.perspective = Some((matrix, matrix.to_inverse()));
    }

    pub fn set_view(&mut self, matrix: Mat4) {
        self.view = Some((matrix, matrix.to_inverse()));
    }

    pub fn set_model(&mut self, matrix: Mat4) {
        self.model = Some((matrix, matrix.to_inverse()));
    }

    pub fn set_vertex_id(&mut self, id: ID) {
        self.vertex_id = id;
    }

    pub fn set_index_id(&mut self, id: ID) {
        self.index_id = id;
    }

    pub fn set_texture_id(&mut self, id: ID) {
        self.texture_id = id;
    }

    pub fn set_draw_method_id(&mut self, id: ID) {
        self.draw_method_id = id;
    }

    pub fn set_perspective_id(&mut self, id: ID) {
        self.perspective_id = id;
    }

    pub fn set_view_id(&mut self, id: ID) {
        self.view_id = id;
    }

    pub fn set_model_id(&mut self, id: ID) {
        self.model_id = id;
    }

    pub fn get_vertex_id(&self) -> ID {
        self.vertex_id
    }

    pub fn get_index_id(&self) -> ID {
        self.index_id
    }

    pub fn get_texture_id(&self) -> ID {
        self.texture_id
    }

    pub fn get_draw_method_id(&self) -> ID {
        self.draw_method_id
    }

    pub fn get_perspective_id(&self) -> ID {
        self.perspective_id
    }

    pub fn get_view_id(&self) -> ID {
        self.view_id
    }

    pub fn get_model_id(&self) -> ID {
        self.model_id
    }
}
