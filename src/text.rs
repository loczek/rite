use core::panic;

use winit::window::Window;

use crate::{font::BitmapFont, scalable, Rectangle, Vertex};

pub struct TextRenderer<'a> {
    bitmap: &'a BitmapFont,
}

impl<'a> TextRenderer<'a> {
    pub fn new(bitmap: &'a BitmapFont) -> Self {
        TextRenderer { bitmap }
    }

    pub fn render(&self, string: &mut String, x: i32, y: i32, window: &Window) -> Vec<Vertex> {
        let mut shapes: Vec<Vertex> = Vec::new();

        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();

        let window_width = size.width as f32;
        let window_height = size.height as f32;

        let mut curr_x = x;
        let mut curr_y = y;

        let bitmap_width = self.bitmap.texture.width();
        let bitmap_height = self.bitmap.texture.height();

        for letter in string.chars() {
            if letter == '\r' {
                curr_x = x;
                continue;
            }

            if letter == '\n' {
                // TODO: this probably should not be bitmap_height but the line height
                curr_y -= bitmap_height as i32;
                continue;
            }

            if letter == ' ' {
                curr_x += 12;
                continue;
            }

            let char = self
                .bitmap
                .char
                .get(&letter)
                .unwrap_or_else(|| panic!("character {} not included in bitmap", letter));

            let rect = Rectangle {
                bottom: curr_y - (char.offset_top as i32),
                left: x + curr_x + char.offset_left,
                width: char.width,
                height: char.height,
            };

            let texture_rect = Rectangle {
                bottom: char.height as i32,
                left: char.id as i32,
                width: char.width,
                height: char.height,
            };

            shapes.extend_from_slice(&Vertex::from(rect, texture_rect));

            curr_x += char.advance as i32;
        }

        let mut shape_iter = shapes.iter_mut();

        while let Some(vertex) = shape_iter.next() {
            scalable::scale(
                vertex,
                bitmap_height as f32,
                bitmap_width as f32,
                window_height,
                window_width,
            );
        }

        shapes
    }
}
