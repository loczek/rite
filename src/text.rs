use core::panic;

use winit::window::Window;

use crate::{font::BitmapFont, scalable, vertex::TextureVertex, Rectangle};

pub struct TextRenderer<'a> {
    bitmap: &'a BitmapFont,
}

impl<'a> TextRenderer<'a> {
    pub fn new(bitmap: &'a BitmapFont) -> Self {
        TextRenderer { bitmap }
    }

    fn generate_shapes(&self, string: &mut String, x: f32, y: f32) -> Vec<TextureVertex> {
        let mut shapes: Vec<_> = Vec::new();

        let mut curr_x = x;
        let mut curr_y = y;

        for letter in string.chars() {
            if letter == '\n' {
                curr_x = x;
                curr_y -= self.bitmap.ascent.abs() + self.bitmap.descent.abs();
                continue;
            }

            if letter == ' ' {
                curr_x += 12.0;
                continue;
            }

            let char = self
                .bitmap
                .char
                .get(&letter)
                .unwrap_or_else(|| panic!("character {} not included in bitmap", letter));

            let rect = Rectangle {
                bottom: curr_y - (char.offset_top),
                left: curr_x + char.offset_left,
                width: char.width,
                height: char.height,
            };

            let texture_rect = Rectangle {
                bottom: char.height,
                left: char.id as f32,
                width: char.width,
                height: char.height,
            };

            shapes.extend_from_slice(&TextureVertex::from(rect, texture_rect));

            curr_x += char.advance;
        }

        shapes
    }

    fn scale_shapes(&self, shapes: &mut Vec<TextureVertex>, window: &Window) {
        let bitmap_width = self.bitmap.texture.width();
        let bitmap_height = self.bitmap.texture.height();

        let size = window.inner_size();

        let mut shape_iter = shapes.iter_mut();

        while let Some(vertex) = shape_iter.next() {
            scalable::scale(
                vertex,
                bitmap_height as f32,
                bitmap_width as f32,
                size.height as f32,
                size.width as f32,
            );
        }
    }

    pub fn render(
        &self,
        string: &mut String,
        x: f32,
        y: f32,
        window: &Window,
    ) -> Vec<TextureVertex> {
        let mut shapes = self.generate_shapes(string, x, y);

        self.scale_shapes(&mut shapes, window);

        shapes
    }
}
