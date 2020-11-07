#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

void main() {
    f_color = vec4(v_tex_coords.x, v_tex_coords.y, (v_tex_coords.x + v_tex_coords.y) * 0.5, 1.0);
}
