#version 140

in vec2 col;

out vec4 color;

void main() {
    color = vec4(col, 1.0, 1.0);
}
