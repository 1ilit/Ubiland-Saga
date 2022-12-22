#version 140

in vec2 position;
in vec4 color; 
in vec2 tex_coords;

out vec4 ourColor;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {
    v_tex_coords = tex_coords;
    ourColor = color;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}