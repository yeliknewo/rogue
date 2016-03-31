use std::sync::{Arc};
use glium::Frame as GliumFrame;
use glium::{Surface};

use components::{Renderable};
use graphics2::{MatrixData};

pub struct RendererOneColor;

impl RendererOneColor {
    pub fn new() -> RendererOneColor {
        RendererOneColor
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, matrix_data: &MatrixData) {
        // frame.draw();
    }

    pub fn init_vertex() {

    }
}
