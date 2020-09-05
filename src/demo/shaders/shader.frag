#version 450

// Inputs from shader.vert
layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec3 v_position;

// Bindings
layout(set=0, binding=0) uniform Uniforms {
    vec3 u_view_position;
    mat4 u_view_proj;
};

layout(set = 1, binding = 0) uniform texture2D t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;

// set=2 would be instance buffer

layout(set = 3, binding = 0) uniform Light {
    vec3 light_position;
    vec3 light_color;
};

// Output
layout(location=0) out vec4 f_color;

void main() {
    vec4 object_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);

    // Ambient light
    float ambient_strength = 0.02;
    vec3 ambient_color = light_color * ambient_strength;

    // Diffuse light
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(light_position - v_position);

    float diffuse_strength = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = light_color * diffuse_strength;

    // Specular light
    vec3 view_dir = normalize(u_view_position - v_position);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular_strength = pow(max(dot(view_dir, reflect_dir), 0.0), 16);
    vec3 specular_color = specular_strength * light_color;

    // Mix lights
    vec3 result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;

    f_color = vec4(result, object_color.a);
}
