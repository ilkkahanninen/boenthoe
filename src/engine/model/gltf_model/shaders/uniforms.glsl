// Bind group, index 0.0: Uniforms

layout(set=0, binding=0) uniform Uniforms {
    mat4 u_view_proj_matrix;
    mat4 u_model_matrix;
    vec4 u_eye_position;
    vec4 u_base_color;

    uint u_number_of_lights;
    float u_metallic_factor;
};

// Bind group, index 1.0: Lights

struct Light {
    vec4 position;
    vec4 direction;
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    vec4 parameters;
    uint type; // 0 = off, 1 = directional, 2 = point, 3 = spotlight
};

layout(set=1, binding=0) buffer Lights {
    Light u_lights[];
};

// Base color texture

layout(set = 2, binding = 0) uniform texture2D t_base_color;
layout(set = 2, binding = 1) uniform sampler s_base_color;
