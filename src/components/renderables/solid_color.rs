use std::sync::{Arc};

use logic::{Id, IdManager, IdType};

pub struct RenderableSolidColor {
    vertex_id: Id,
    index_id: Id,
    draw_method_id: Id,
}

impl RenderableSolidColor {
    pub fn new(manager: &mut IdManager) -> RenderableSolidColor {
        RenderableSolidColor {
            vertex_id: Id::new(manager, IdType::Vertex),
            index_id: Id::new(manager, IdType::Index),
            draw_method_id: Id::new(manager, IdType::DrawMethod),
        }
    }

    pub fn new_from(other: Arc<RenderableSolidColor>) -> RenderableSolidColor {
        RenderableSolidColor {
            vertex_id: other.vertex_id,
            index_id: other.index_id,
            draw_method_id: other.draw_method_id,
        }
    }

    pub fn get_vertex_id(&self) -> Id {
        self.vertex_id
    }

    pub fn get_index_id(&self) -> Id {
        self.index_id
    }

    pub fn get_draw_method_id(&self) -> Id {
        self.draw_method_id
    }
}
