#version 300 es

precision highp float;

in vec3 vertex_color;

out vec4 color;

void main() {
    color = vec4(vertex_color, 1.0f);
}