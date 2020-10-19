#version 450

// Bind group, index 0.0: Uniforms

layout(set=0, binding=0) uniform Uniforms {
    mat4 u_view_proj_matrix;
    mat4 u_model_matrix;
    vec3 u_eye_position;
};

// Bind group, index 1.0: Lights

layout(set=1, binding=0) uniform Lights {
    vec4 u_light_color;
    vec3 u_light_pos;
};

// Inputs from vertex shader

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coords;
layout(location=3) in vec4 a_color;

// Output

layout(location=0) out vec4 out_color;

vec4 phong_model(int light_index, float ambient_strength, float specular_strength) {
    vec4 light_color = u_light_color;
    vec3 light_pos = u_light_pos;

    // Ambient light
    vec3 ambient = ambient_strength * light_color.a * light_color.rgb;

    // Diffuse light
    vec3 norm = normalize(a_normal);
    vec3 light_dir = normalize(light_pos - a_position);
    float diffuse_strength = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diffuse_strength * light_color.a * light_color.rgb;

    // Specular light
    vec3 view_dir = normalize(u_eye_position - a_position);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light_color.a * light_color.rgb;

    // Mix lights
    vec3 result = (ambient + diffuse + specular) * a_color.rgb;
    return vec4(result, a_color.a);
}

void main() {
    out_color = phong_model(0, 0.1, 0.5);
}