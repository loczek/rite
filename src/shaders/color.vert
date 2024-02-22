#version 300 es

in vec2 position;
in vec3 color;

out vec3 vertex_color;

void main() {
    vertex_color = color;
    gl_Position = vec4(position, 0.0f, 1.0f);
}