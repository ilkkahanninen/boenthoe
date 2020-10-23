#version 450
#include "uniforms.glsl"

// Vertex buffer inputs

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coords;
layout(location=3) in vec4 a_color;
layout(location=4) in vec4 a_tangent;

// Outputs to fragment shader

layout(location=0) out VS_OUT vs_out;

void main() {
    // Position
    vec4 model_space = vec4(a_position, 1.0);
    vec4 world_space = uniforms.model_matrix * model_space;
    vs_out.position = vec3(world_space);
    gl_Position = uniforms.view_proj_matrix * world_space;

    // Normal
    // TODO: Inverse is an expensive operation, calculate it on CPU and move it to uniforms
    vs_out.normal = mat3(transpose(inverse(uniforms.model_matrix))) * a_normal;

    // Color
    vs_out.color = a_color;

    // Texture coordinates
    vs_out.tex_coords = a_tex_coords;

    // Export inversed TBN matrix for normal mapping
    vec3 N = normalize(vec3(uniforms.model_matrix * vec4(a_normal, 0.0)));
    vec3 T = normalize(vec3(uniforms.model_matrix * vec4(a_tangent.xyz, 0.0)));
    vec3 B = cross(N, T); // TODO: Precalc this

    vs_out.tbn = transpose(mat3(T, B, N)); // transpose of an orthogonal matrix equals its inverse
}
