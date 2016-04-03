use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::glutin::WindowBuilder as GlutinWindowBuilder;
use glium::glutin::{CreationError, get_primary_monitor};
use glium::{Surface, DisplayBuild, GliumCreationError, SwapBuffersError};
use glium::Frame as GliumFrame;
use std::fmt;
use std::error::Error;
use std::sync::{Arc};

use logic::{EntityData};

use graphics2::texture2d::{RendererTex2, RendererTex2Err};
use graphics2::solid_color::{RendererSolidColor, RendererSolidColorErr};
use graphics2::vertex_color::{RendererVertexColor, RendererVertexColorErr};
use graphics2::{RendererType, MatrixData};

pub struct Frame<'a> {
    frame: GliumFrame,
    renderer_solid_color: &'a mut RendererSolidColor,
    renderer_vertex_color: &'a mut RendererVertexColor,
    renderer_texture2d: &'a mut RendererTex2,
}

impl<'a> Frame<'a> {
    fn new(facade: &'a mut GlutinFacade, renderer_solid_color: &'a mut RendererSolidColor, renderer_vertex_color: &'a mut RendererVertexColor, renderer_texture2d: &'a mut RendererTex2) -> Frame<'a> {
        let mut frame  = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            renderer_solid_color: renderer_solid_color,
            renderer_vertex_color: renderer_vertex_color,
            renderer_texture2d: renderer_texture2d,
        }
    }

    pub fn draw_entity<Y: EntityData<Y>>(&mut self, entity: &Y, matrix_data: &MatrixData) -> Result<(), FrameErr> {
        match entity.get_renderable() {
            Some(renderable) => {
                match renderable.get_renderer_type() {
                    RendererType::SolidColor => match self.renderer_solid_color.render(&mut self.frame, renderable, matrix_data) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(FrameErr::RendererSolidColor("Self RendererSolidColor Render", err)),
                    },
                    RendererType::VertexColor => match self.renderer_vertex_color.render(&mut self.frame, renderable, matrix_data) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(FrameErr::RendererVertexColor("Self RendererVertexColor Render", err)),
                    },
                    RendererType::Texture2d => match self.renderer_texture2d.render(&mut self.frame, renderable, matrix_data) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(FrameErr::RendererTex2("Self Renderer Texture2d Render", err)),
                    },
                }
            },
            None => Ok(()),
        }
    }

    pub fn end(self) -> Result<(), FrameErr> {
        match self.frame.finish() {
            Ok(()) => Ok(()),
            Err(err) => Err(FrameErr::SwapBuffers("Self Frame Finish", err)),
        }
    }
}

#[derive(Debug)]
pub enum FrameErr {
    SwapBuffers(&'static str, SwapBuffersError),
    RendererTex2(&'static str, RendererTex2Err),
    RendererSolidColor(&'static str, RendererSolidColorErr),
    RendererVertexColor(&'static str, RendererVertexColorErr),
}

impl fmt::Display for FrameErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FrameErr::SwapBuffers(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for FrameErr {
    fn description(&self) -> &str {
        match *self {
            FrameErr::SwapBuffers(_, ref err) => err.description(),
        }
    }
}

pub struct Window {
    facade: GlutinFacade,

}

impl<'a> Window {
    pub fn frame(&mut self) -> Frame {
        Frame::new(&mut self.facade, &mut self.renderer_solid_color, &mut self.renderer_vertex_color, &mut self.renderer_texture2d)
    }

    pub fn poll_events(&self) -> PollEventsIter {
        self.facade.poll_events()
    }
}

pub struct WindowBuilder {
    windowed: Windowed,
    dimensions: (u32, u32),
    title: String,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            windowed: Windowed::Windowed,
            dimensions: (640, 480),
            title: "Untitled".to_string(),
        }
    }

    pub fn build(mut self) -> Result<(Window, (u32, u32)), WindowErr> {
        let resolution: (u32, u32) = get_primary_monitor().get_dimensions();
        Ok(
            (
                Window {
                    facade: match self.windowed {
                        Windowed::Windowed => {
                            let facade = match GlutinWindowBuilder::new()
                                .with_title(self.title)
                                .with_dimensions(self.dimensions.0, self.dimensions.1)
                                .with_decorations(true)
                                .with_depth_buffer(24)
                                .with_vsync()
                                .build_glium() {
                                    Ok(facade) => facade,
                                    Err(err) => return Err(WindowErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                };
                            match facade.get_window() {
                                Some(window) => window,
                                None => return Err(WindowErr::Get("Facade Get Window")),
                            }.set_position(((resolution.0 - self.dimensions.0) / 2) as i32, ((resolution.1 - self.dimensions.1) / 2) as i32);
                            facade
                        },
                        // Windowed::Fullscreen => {
                        //
                        // },
                        Windowed::Borderless => {
                            let facade = match GlutinWindowBuilder::new()
                                .with_title(self.title)
                                .with_dimensions(resolution.0, resolution.1)
                                .with_decorations(false)
                                .with_depth_buffer(24)
                                .with_vsync()
                                .build_glium() {
                                    Ok(facade) => facade,
                                    Err(err) => return Err(WindowErr::GliumCreation("GlutinWindowBuilder Build Glium", err)),
                                };
                            match facade.get_window() {
                                Some(window) => window,
                                None => return Err(WindowErr::Get("Facade Get Window")),
                            }.set_position(0, 0);
                            facade
                        },
                    },

                },
                self.dimensions
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
