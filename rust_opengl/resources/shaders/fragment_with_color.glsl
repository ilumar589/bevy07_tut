#version 330 core

in vec3 out_colors;
out vec4 FragColor;

void main() {
    FragColor = vec4(out_colors, 1.0f);
}