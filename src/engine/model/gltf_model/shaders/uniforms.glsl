// Constants
const uint MAX_NUMBER_OF_LIGHTS = 8;

// Bind group, index 0.0: Uniforms

layout(set=0, binding=0) uniform Uniforms {
    mat4 view_proj_matrix;
    mat4 model_matrix;
    vec4 eye_position;
    vec4 base_color;

    uint number_of_lights;
    float metallic_factor;
} uniforms;

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
    Light lights[];
};

// Textures

layout(set = 2, binding = 0) uniform texture2D t_base_color;
layout(set = 2, binding = 1) uniform sampler s_base_color;
layout(set = 2, binding = 2) uniform texture2D t_normal_map;
layout(set = 2, binding = 3) uniform sampler s_normal_map;
layout(set = 2, binding = 4) uniform texture2D t_emission;
layout(set = 2, binding = 5) uniform sampler s_emission;

// Vertex shader -> fragment shader data

struct VS_OUT {
    vec3 position;
    vec3 normal;
    vec2 tex_coords;
    vec4 color;

    vec3 tangent_position;
    vec3 tangent_view_pos;
    vec3 tangent_light_pos[MAX_NUMBER_OF_LIGHTS];
    vec3 tangent_light_dir[MAX_NUMBER_OF_LIGHTS];
};
