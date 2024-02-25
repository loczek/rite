use crate::vertex::{ColorVertex, TextureVertex};

pub fn scale(
    vertex: &mut TextureVertex,
    from_height: f32,
    from_width: f32,
    to_height: f32,
    to_width: f32,
) {
    let w_factor = vertex.position[0] / (to_width / 2.0);
    let h_factor = vertex.position[1] / (to_height / 2.0);
    vertex.position[0] = w_factor - 1.0;
    vertex.position[1] = h_factor - 1.0;

    vertex.tex_coords[0] = vertex.tex_coords[0] / from_width as f32;
    vertex.tex_coords[1] = vertex.tex_coords[1] / from_height as f32;
}

pub fn rescale_texture(vertex: &mut TextureVertex, from_height: f32, from_width: f32) {
    vertex.tex_coords[0] = vertex.tex_coords[0] / from_width as f32;
    vertex.tex_coords[1] = vertex.tex_coords[1] / from_height as f32;
}

pub fn rescale_position(vertex: &mut ColorVertex, to_height: f32, to_width: f32) {
    let w_factor = vertex.position[0] / (to_width / 2.0);
    let h_factor = vertex.position[1] / (to_height / 2.0);
    vertex.position[0] = w_factor - 1.0;
    vertex.position[1] = h_factor - 1.0;
}
