#version 140

in vec2 pos;

out vec2 col;

uniform mat4 matrix;

void main() {
    col = pos;
    gl_Position = matrix * vec4(pos, 0.0, 1.0);
}
