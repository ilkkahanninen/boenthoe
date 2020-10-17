#version 450

// Inputs from vertex shader

layout(location=0) in vec3 a_normal;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 a_color;

// Output

layout(location=0) out vec4 out_color;

void main() {
    out_color = vec4(1.0, 0.8, 0.2, 1.0);
}