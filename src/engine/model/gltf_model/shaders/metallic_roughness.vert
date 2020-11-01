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
    vec4 pos_model_space = vec4(a_position, 1.0);
    vec4 pos_world_space = uniforms.model_matrix * pos_model_space;
    gl_Position = uniforms.view_proj_matrix * pos_world_space;

    // Texture coordinates
    vs_out.tex_coords = a_tex_coords;

    // Export inversed TBN matrix for normal mapping
    vec3 N = normalize(vec3(uniforms.model_matrix * vec4(a_normal, 0.0)));
    vec3 T = normalize(vec3(uniforms.model_matrix * vec4(a_tangent.xyz, 0.0)));
    vec3 B = cross(N, T); // TODO: Precalc this
    mat3 TBN = transpose(mat3(T, B, N)); // transpose of an orthogonal matrix equals its inverse

    vs_out.tangent_position = TBN * pos_world_space.xyz;
    vs_out.tangent_view_pos = TBN * uniforms.eye_position.xyz;

    uint number_of_lights = min(uniforms.number_of_lights, MAX_NUMBER_OF_LIGHTS);
    for (int i = 0; i < number_of_lights; i++) {
        vs_out.tangent_light_pos[i] = TBN * lights[i].position.xyz;
        vs_out.tangent_light_dir[i] = TBN * lights[i].direction.xyz;
    }
}
