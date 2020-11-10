#version 450

layout(set = 0, binding = 0) uniform SimpleUniforms {
    float time;
    vec3 _padding;
} uniforms;

layout(location=0) out vec4 f_color;
layout(location=0) in vec2 v_tex_coords;

void main() {
    f_color = vec4(
        0.5 + sin(v_tex_coords.x * 3.0 + v_tex_coords.y * 4.0 + uniforms.time * 10.2) * 0.5,
        0.5 + sin(v_tex_coords.x * 14.0 + v_tex_coords.y * 13.0 + uniforms.time * 10.3) * 0.5,
        0.5 + sin(v_tex_coords.x * 2.5 + v_tex_coords.y * 3.5 + uniforms.time * 10.4) * 0.5,
        1.0);
}
