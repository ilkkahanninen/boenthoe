#version 450

// Vertex buffer inputs

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coords;
layout(location=3) in vec4 a_color;

// Bind group, index 0: Uniforms

layout(set=0, binding=0) uniform Uniforms {
    mat4 u_view_proj;
    mat4 u_space;
};

// Outputs to fragment shader

layout(location=0) out vec3 out_normal;
layout(location=1) out vec2 out_tex_coords;
layout(location=2) out vec4 out_color;

void main() {
    // Position
    vec4 model_space = u_space * vec4(a_position, 1.0);
    gl_Position = u_view_proj * model_space;

    // Texture coordinates
    out_tex_coords = a_tex_coords;

    // Normal
    out_normal = a_normal;

    // Color
    out_color = a_color;
}
