#version 450

// Vertex buffer inputs

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coords;
layout(location=3) in vec4 a_color;

// Bind group, index 0.0: Uniforms

layout(set=0, binding=0) uniform Uniforms {
    mat4 u_view_proj_matrix;
    mat4 u_model_matrix;
    vec3 u_eye_position;
};

// Outputs to fragment shader

layout(location=0) out vec3 frag_position;
layout(location=1) out vec3 frag_normal;
layout(location=2) out vec2 frag_tex_coords;
layout(location=3) out vec4 frag_color;

void main() {
    // Position
    vec4 model_space = vec4(a_position, 1.0);
    vec4 world_space = u_model_matrix * model_space;
    frag_position = vec3(world_space);
    gl_Position = u_view_proj_matrix * world_space;

    // Normal
    // TODO: Inverse is an expensive operation, calculate it on CPU and move it to uniforms
    frag_normal = mat3(transpose(inverse(u_model_matrix))) * a_normal;

    // Color
    frag_color = a_color;

    // Texture coordinates
    frag_tex_coords = a_tex_coords;
}
