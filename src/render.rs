use glium::{
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    index::NoIndices,
    uniforms::{AsUniformValue, Uniforms},
    Frame, Program, Surface, VertexBuffer,
};
use winit::window::Window;

use crate::{
    font::BitmapFont,
    vertex::{ColorVertex, TextureVertex},
};

pub struct Application<'a> {
    pub window: &'a Window,
    pub display: &'a Display<WindowSurface>,
    pub indices: NoIndices,
    pub program: &'a Program,
    pub font: &'a BitmapFont,
}

impl<'a> Application<'a> {
    pub fn new(
        window: &'a Window,
        display: &'a Display<WindowSurface>,
        program: &'a Program,
        font: &'a BitmapFont,
    ) -> Self {
        Application {
            window,
            display,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            program,
            font,
        }
    }

    pub fn render<'n, T: Copy, U: AsUniformValue, R: Uniforms>(
        &self,
        vertex_buffer: &VertexBuffer<T>,
        frame: &mut Frame,
        uniforms: glium::uniforms::UniformsStorage<'n, U, R>,
    ) {
        frame
            .draw(
                vertex_buffer,
                &self.indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

trait Scale {
    fn scale(&mut self, application: Application)
    where
        Self: Sized;
}

impl Scale for TextureVertex {
    fn scale(&mut self, application: Application)
    where
        Self: Sized + ScalePosition + ScaleTexture,
    {
        self.scale_position(
            application.window.inner_size().width as f32,
            application.window.inner_size().height as f32,
        );
        self.scale_texture(
            application.font.texture.width() as f32,
            application.font.texture.height() as f32,
        );
    }
}

impl Scale for ColorVertex {
    fn scale(&mut self, application: Application)
    where
        Self: Sized + ScalePosition,
    {
        self.scale_position(
            application.window.inner_size().width as f32,
            application.window.inner_size().height as f32,
        );
    }
}

trait ScalePosition {
    fn scale_position(&mut self, width: f32, height: f32);
}

impl ScalePosition for ColorVertex {
    fn scale_position(&mut self, width: f32, height: f32) {
        let w_factor = self.position[0] / (width / 2.0);
        let h_factor = self.position[1] / (height / 2.0);
        self.position[0] = w_factor - 1.0;
        self.position[1] = h_factor - 1.0;
    }
}

impl ScalePosition for TextureVertex {
    fn scale_position(&mut self, width: f32, height: f32) {
        let w_factor = self.position[0] / (width / 2.0);
        let h_factor = self.position[1] / (height / 2.0);
        self.position[0] = w_factor - 1.0;
        self.position[1] = h_factor - 1.0;
    }
}

trait ScaleTexture {
    fn scale_texture(&mut self, width: f32, height: f32);
}

impl ScaleTexture for TextureVertex {
    fn scale_texture(&mut self, width: f32, height: f32) {
        self.tex_coords[0] = self.tex_coords[0] / width;
        self.tex_coords[1] = self.tex_coords[1] / height;
    }
}
