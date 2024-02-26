use std::{collections::HashMap, fmt::Debug};

use ab_glyph::{point, Font, FontRef, ScaleFont};
use glium::{glutin::surface::WindowSurface, Display};
use image::Rgba;

#[derive(Copy, Clone, Debug)]
pub struct Character {
    pub id: u32,
    pub width: f32,
    pub height: f32,
    pub advance: f32,
    pub offset_top: f32,
    pub offset_left: f32,
}

#[derive(Debug)]
pub struct BitmapFont {
    pub texture: glium::Texture2d,
    pub char: HashMap<char, Character>,
    pub ascent: f32,
    pub descent: f32,
}

impl BitmapFont {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let scale = 24.0;

        let font =
            FontRef::try_from_slice(include_bytes!("./assets/FiraCode-Regular.ttf")).unwrap();

        let characters = "!\"#$%&'()*+,-./(){};+,0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

        let width: u32 = characters.chars().fold(0, |acc, c| {
            acc + font
                .outline_glyph(font.glyph_id(c).with_scale(scale))
                .unwrap()
                .px_bounds()
                .width() as u32
        });

        let height: u32 = characters.chars().fold(0, |acc, c| {
            acc.max(
                font.outline_glyph(font.glyph_id(c).with_scale(scale))
                    .unwrap()
                    .px_bounds()
                    .height() as u32,
            )
        });

        let mut image = image::RgbaImage::new(width, height);

        let mut x_offset = 0;

        let mut map: HashMap<char, Character> = HashMap::new();

        for char in characters.chars() {
            let glyph = font
                .glyph_id(char)
                .with_scale_and_position(scale, point(0.0, 0.0));

            if let Some(outline) = font.outline_glyph(glyph.clone()) {
                let outline_bounds = outline.px_bounds();

                outline.draw(|x: u32, y, c: f32| {
                    let clamp = c.max(0.0).min(1.0);

                    let new_x: u32 = x;
                    let new_y: u32 = y;

                    if new_x > height || new_y > height {
                        return;
                    }

                    image.put_pixel(
                        new_x + x_offset,
                        new_y,
                        Rgba::from([
                            (clamp * 255.0) as u8,
                            (clamp * 255.0) as u8,
                            (clamp * 255.0) as u8,
                            (clamp * 255.0) as u8,
                        ]),
                    );
                });

                map.insert(
                    char,
                    Character {
                        id: x_offset,
                        width: outline_bounds.width(),
                        height: outline_bounds.height(),
                        advance: font.as_scaled(scale).h_advance(glyph.id),
                        offset_top: outline_bounds.max.y,
                        offset_left: outline_bounds.min.x,
                    },
                );

                x_offset += outline_bounds.width() as u32;
            }
        }

        let image_dimensions = image.dimensions();

        let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);

        Self {
            texture: glium::texture::Texture2d::new(display, image).unwrap(),
            char: map,
            ascent: font.as_scaled(scale).ascent(),
            descent: font.as_scaled(scale).descent(),
        }
    }
}
