#version 300 es

precision highp float;

in vec2 v_tex_coords;

uniform sampler2D tex;

out vec4 color;

void main() {
    if(texture(tex, v_tex_coords).a < 0.2f)
        discard;
    color = texture(tex, v_tex_coords);
}