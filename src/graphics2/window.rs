use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::glutin::WindowBuilder as GlutinWindowBuilder;
use glium::glutin::{CreationError};
use glium::{Surface, DisplayBuild, GliumCreationError, SwapBuffersError};
use glium::Frame as GliumFrame;
use std::fmt;
use std::error::Error;
use std::sync::{Arc};
use std::collections::{HashMap};

use logic::{EntityData, Id};
use math::{Mat4};
use graphics2::{RendererTex2, RendererColor, Renderers};

pub struct Frame<'a> {
    frame: GliumFrame,
    facade: &'a mut GlutinFacade,
    renderer_color: &'a mut RendererColor,
    renderer_texture2d: &'a mut RendererTex2,
}

impl<'a> Frame<'a> {
    fn new(facade: &'a mut GlutinFacade, renderer_color: &'a mut RendererColor, renderer_texture2d: &'a mut RendererTex2) -> Frame<'a> {
        let mut frame  = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            facade: facade,
            renderer_color: renderer_color,
            renderer_texture2d: renderer_texture2d,
        }
    }

    pub fn draw_entity<Y: EntityData<Y>>(&mut self, entity: &Y, matrix_data: Arc<HashMap<Id, Mat4>>) {
        match entity.get_renderable() {
            Some(renderable) => {
                match renderable.get_renderer_type() {
                    Renderers::Color => self.renderer_color.render(renderable),
                    Renderers::Texture2d => self.renderer_texture2d.render(renderable),
                }
            }
        }
    }

    pub fn end(mut self) -> Result<(), FrameErr> {
        match self.frame.finish() {
            Ok(()) => Ok(()),
            Err(err) => Err(FrameErr::SwapBuffers("Self Frame Finish", err)),
        }
    }
}

#[derive(Debug)]
pub enum FrameErr {
    Get(&'static str),
    SwapBuffers(&'static str, SwapBuffersError),
}

impl fmt::Display for FrameErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FrameErr::Get(_) => write!(f, "Get was None"),
            FrameErr::SwapBuffers(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for FrameErr {
    fn description(&self) -> &str {
        match *self {
            FrameErr::Get(_) => "Get was None",
            FrameErr::SwapBuffers(_, ref err) => err.description(),
        }
    }
}

pub struct Window {
    facade: GlutinFacade,
    renderer_color: RendererColor,
    renderer_texture2d: RendererTex2,
}

impl<'a> Window {
    pub fn frame(&mut self) -> Frame {
        Frame::new(&mut self.facade, &mut self.renderer_color, &mut self.renderer_texture2d)
    }

    pub fn poll_events(&self) -> PollEventsIter {
        self.facade.poll_events()
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

    pub fn build(mut self) -> Result<(Window, (u32, u32)), WindowErr> {
        Ok(
            (
                Window {
                    facade: match self.windowed {
                        Some(windowed) => {
                            match windowed {
                                Windowed::Windowed => {
                                    let facade = match GlutinWindowBuilder::new()
                                        .build_glium() {
                                            Ok(facade) => facade,
                                            Err(err) => return Err(WindowErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                        };
                                    facade
                                },
                                // Windowed::Fullscreen => {
                                //
                                // },
                                Windowed::Borderless => {
                                    let facade = match GlutinWindowBuilder::new()
                                        .build_glium() {
                                            Ok(facade) => facade,
                                            Err(err) => return Err(WindowErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                        };
                                    facade
                                },
                            }
                        },
                        None => {
                            let facade = match GlutinWindowBuilder::new()
                                .build_glium() {
                                    Ok(facade) => facade,
                                    Err(err) => return Err(WindowErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                };
                            facade
                        }
                    },
                    renderer_color: RendererColor::new(),
                    renderer_texture2d: RendererTex2::new(),
                },
                (
                    640,
                    480
                )
            )
        )
    }
}

enum Windowed {
    Windowed,
    //Fullscreen,
    Borderless,
}

#[derive(Debug)]
pub enum WindowErr {
    Get(&'static str),
    // VertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    // IndexBufferCreation(&'static str, glium::index::BufferCreationError),
    GliumCreation(&'static str, GliumCreationError<CreationError>),
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
            WindowErr::GliumCreation(_, ref err) => err.fmt(f),
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
            WindowErr::GliumCreation(_, ref err) => err.description(),
            // WindowErr::ProgramCreation(_, ref err) => err.description(),
            // WindowErr::TextureCreation(_, ref err) => err.description(),
            // WindowErr::Image(_, ref err) => err.description(),
        }
    }
}
