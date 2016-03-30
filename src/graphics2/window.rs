use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use std::fmt;
use std::error::Error;
use std::collections::{HashMap};

use graphics2::{Renderer, RendererType};

pub struct Window<T: RendererType> {
    facade: GlutinFacade,
    renderers: HashMap<T, Box<Renderer>>,
}

impl<T: RendererType> Window<T> {

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

    pub fn build<T: RendererType>(mut self) -> Result<(Window<T>, (u32, u32)), WindowErr> {
        Window {
            facade: match self.windowed.as_ref() {
                Some(windowed) => {
                    match windowed {
                        Windowed::Windowed => {

                        },
                        Windowed::Fullscreen => {

                        },
                        Windowed::Borderless => {

                        },
                    }
                },
                None => {

                }
            },
        }
    }
}

enum Windowed {
    Windowed,
    Fullscreen,
    Borderless,
}

#[derive(Debug)]
pub enum WindowErr {
    Get(&'static str),
    // VertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    // IndexBufferCreation(&'static str, glium::index::BufferCreationError),
    // GliumCreation(&'static str, GliumCreationError<CreationError>),
    // ProgramCreation(&'static str, ProgramCreationError),
    // TextureCreation(&'static str, glium::texture::TextureCreationError),
    // Image(&'static str, ImageError),
}

impl fmt::Display for WindowErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WindowErr::Get(_) => write!(f, "Get was None"),
            // WindowErr::VertexBufferCreation(_, ref err) => err.fmt(f),
            // WindowErr::IndexBufferCreation(_, ref err) => err.fmt(f),
            // WindowErr::GliumCreation(_, ref err) => err.fmt(f),
            // WindowErr::ProgramCreation(_, ref err) => err.fmt(f),
            // WindowErr::TextureCreation(_, ref err) => err.fmt(f),
            // WindowErr::Image(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for WindowErr {
    fn description(&self) -> &str {
        match *self {
            WindowErr::Get(_) => "Get was None",
            // WindowErr::VertexBufferCreation(_, ref err) => err.description(),
            // WindowErr::IndexBufferCreation(_, ref err) => err.description(),
            // WindowErr::GliumCreation(_, ref err) => err.description(),
            // WindowErr::ProgramCreation(_, ref err) => err.description(),
            // WindowErr::TextureCreation(_, ref err) => err.description(),
            // WindowErr::Image(_, ref err) => err.description(),
        }
    }
}
