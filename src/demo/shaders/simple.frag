#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform SimpleUniforms {
    float time;
};

void main() {
    f_color = vec4(
        0.5 + sin(v_tex_coords.x * 3.0 + v_tex_coords.y * 4.0 + time * 10.2) * 0.5,
        0.5 + sin(v_tex_coords.x * 4.0 + v_tex_coords.y * 3.0 + time * 10.3) * 0.5,
        0.5 + sin(v_tex_coords.x * 2.5 + v_tex_coords.y * 3.5 + time * 10.4) * 0.5,
        1.0);
}
