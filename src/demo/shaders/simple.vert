#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

// layout(set = 0, binding = 0) uniform SimpleUniforms {
//     float time;
//     vec3 _padding;
// } uniforms;

void main() {
    // float zoom = sin(uniforms.time) + 1.0;
    gl_Position = vec4(a_position, 1.0);
    v_tex_coords = a_tex_coords;
}