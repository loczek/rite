use glium::implement_vertex;

use crate::Rectangle;

#[derive(Copy, Clone, Debug)]
pub struct TextureVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
implement_vertex!(TextureVertex, position, tex_coords);

impl TextureVertex {
    pub fn from(rect: Rectangle, texture_rect: Rectangle) -> Vec<TextureVertex> {
        let bottom = rect.bottom as f32;
        let left = rect.left as f32;
        let height = rect.height as f32;
        let width = rect.width as f32;

        let texture_bottom = texture_rect.bottom as f32;
        let texture_left = texture_rect.left as f32;
        let texture_width = texture_rect.width as f32;

        let shape = vec![
            TextureVertex {
                // top left
                position: [left, bottom + height],
                tex_coords: [texture_left, 0.0],
            },
            TextureVertex {
                // top right
                position: [left + width, bottom + height],
                tex_coords: [texture_left + texture_width, 0.0],
            },
            TextureVertex {
                // bottom right
                position: [left + width, bottom],
                tex_coords: [texture_left + texture_width, texture_bottom],
            },
            TextureVertex {
                // bottom right
                position: [left + width, bottom],
                tex_coords: [texture_left + texture_width, texture_bottom],
            },
            TextureVertex {
                // bottom left
                position: [left, bottom],
                tex_coords: [texture_left, texture_bottom],
            },
            TextureVertex {
                // top left
                position: [left, bottom + height],
                tex_coords: [texture_left, 0.0],
            },
        ];
        shape
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}
implement_vertex!(ColorVertex, position, color);

impl ColorVertex {
    pub fn from(rect: Rectangle, color: [f32; 3]) -> Vec<ColorVertex> {
        let bottom = rect.bottom as f32;
        let left = rect.left as f32;
        let height = rect.height as f32;
        let width = rect.width as f32;

        let shape = vec![
            ColorVertex {
                // top left
                position: [left, bottom + height],
                color,
            },
            ColorVertex {
                // top right
                position: [left + width, bottom + height],
                color,
            },
            ColorVertex {
                // bottom right
                position: [left + width, bottom],
                color,
            },
            ColorVertex {
                // bottom right
                position: [left + width, bottom],
                color,
            },
            ColorVertex {
                // bottom left
                position: [left, bottom],
                color,
            },
            ColorVertex {
                // top left
                position: [left, bottom + height],
                color,
            },
        ];
        shape
    }
}
