use glium::backend::glutin_backend::{GlutinFacade, PollEventsIter};
use glium::glutin::WindowBuilder as GlutinWindowBuilder;
use glium::glutin::{CreationError, get_primary_monitor};
use glium::{Surface, DisplayBuild, GliumCreationError, SwapBuffersError};
use glium::Frame as GliumFrame;
use std::fmt;
use std::error::Error;

use logic::{EntityData};
use graphics2::{RendererTex2, RendererOneColor, RendererMultiColor, Renderers, MatrixData};

pub struct Frame<'a> {
    frame: GliumFrame,
    renderer_one_color: &'a mut RendererOneColor,
    renderer_multi_color: &'a mut RendererMultiColor,
    renderer_texture2d: &'a mut RendererTex2,
}

impl<'a> Frame<'a> {
    fn new(facade: &'a mut GlutinFacade, renderer_one_color: &'a mut RendererOneColor, renderer_multi_color: &'a mut RendererMultiColor, renderer_texture2d: &'a mut RendererTex2) -> Frame<'a> {
        let mut frame  = facade.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        Frame {
            frame: frame,
            renderer_one_color: renderer_one_color,
            renderer_multi_color: renderer_multi_color,
            renderer_texture2d: renderer_texture2d,
        }
    }

    pub fn draw_entity<Y: EntityData<Y>>(&mut self, entity: &Y, matrix_data: &MatrixData) {
        match entity.get_renderable() {
            Some(renderable) => {
                match renderable.get_renderer_type() {
                    Renderers::OneColor => self.renderer_one_color.render(&mut self.frame, renderable, matrix_data),
                    Renderers::MultiColor => self.renderer_multi_color.render(&mut self.frame, renderable, matrix_data),
                    Renderers::Texture2d => self.renderer_texture2d.render(&mut self.frame, renderable, matrix_data),
                }
            },
            None => (),
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
    renderer_one_color: RendererOneColor,
    renderer_multi_color: RendererMultiColor,
    renderer_texture2d: RendererTex2,
}

impl<'a> Window {
    pub fn frame(&mut self) -> Frame {
        Frame::new(&mut self.facade, &mut self.renderer_one_color, &mut self.renderer_multi_color, &mut self.renderer_texture2d)
    }

    pub fn get_mut_tex2(&mut self) -> &mut RendererTex2 {
        &mut self.renderer_texture2d
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
                    renderer_one_color: RendererOneColor::new(),
                    renderer_multi_color: RendererMultiColor::new(),
                    renderer_texture2d: RendererTex2::new(),
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
