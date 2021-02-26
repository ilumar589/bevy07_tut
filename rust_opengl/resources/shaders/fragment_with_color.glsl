#version 330 core

in vec3 some_colors;
out vec4 FragColor;

void main() {
    FragColor = vec4(some_colors, 1.0f);
}