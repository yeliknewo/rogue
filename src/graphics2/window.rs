use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use std::collections::{HashMap};

use graphics2::{Renderer, RendererType};

pub struct Window<T: RendererType> {
    facade: GlutinFacade,
    renderers: HashMap<T, Box<Renderer>>,
}

impl<T: RendererType> Window<T> {
    fn new() -> Window<T> {

    }
}

pub struct WindowBuilder {
    windowed: Option<Windowed>,
    resolution: Option<(u32, u32)>,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            windowed: None,
            resolution: None,
        }

    }
}

enum Windowed {
    Windowed,
    Fullscreen,
    Borderless,
}
