#version 450

// Inputs from vertex buffer
layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 a_normal;

// Bindings
layout(set=0, binding=0) uniform Uniforms {
    vec3 u_view_position;
    mat4 u_view_proj;
};
// set=1 would be material
layout(set=2, binding=0) buffer Instances {
    mat4 s_models[];
};
// set=3 would be light

// Outputs
layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_position;

void main() {
    v_tex_coords = a_tex_coords;

    mat4 model_matrix = s_models[gl_InstanceIndex];
    mat3 normal_matrix = mat3(transpose(inverse(model_matrix)));
    v_normal = a_normal * normal_matrix;

    vec4 model_space = model_matrix  * vec4(a_position, 1.0);
    v_position = model_space.xyz;
    gl_Position = u_view_proj  * model_space;
}
