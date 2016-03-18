use std::sync::{Arc, RwLock};

use logic::{ID, IDManager, IDType};

#[derive(Clone)]
pub struct EntityDataGraphics {
    texture_id: ID,
    vertex_id: ID,
    index_id: ID,
    draw_parameters_id: ID,
    perspective_id: ID,
    view_id: ID,
    model_id: ID,
}

impl EntityDataGraphics {
    pub fn new(manager: Arc<RwLock<IDManager>>) -> EntityDataGraphics {
        EntityDataGraphics {
            texture_id: ID::new(manager.clone(), IDType::Texture),
            vertex_id: ID::new(manager.clone(), IDType::Vertex),
            index_id: ID::new(manager.clone(), IDType::Index),
            draw_parameters_id: ID::new(manager.clone(), IDType::DrawParameter),
            perspective_id: ID::new(manager.clone(), IDType::Perspective),
            view_id: ID::new(manager.clone(), IDType::View),
            model_id: ID::new(manager.clone(), IDType::Model),
        }
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

    pub fn set_draw_parameters_id(&mut self, id: ID) {
        self.draw_parameters_id = id;
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

    pub fn get_draw_parameters_id(&self) -> ID {
        self.draw_parameters_id
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
