#version 450
#include "uniforms.glsl"

// Inputs from vertex shader
layout(location=0) in VS_OUT fs_in;

// Output
layout(location=0) out vec4 out_color;

vec3 get_normal() {
    #ifdef USE_NORMAL_MAPS
        return normalize(texture(sampler2D(t_normal_map, s_normal_map), fs_in.tex_coords).rgb * 2.0 - 1.0);
    #else
        return normalize(fs_in.normal);
    #endif
}

vec3 get_view_dir() {
    #ifdef USE_NORMAL_MAPS
        return normalize(fs_in.tangent_view_pos - fs_in.tangent_position);
    #else
        return normalize(uniforms.eye_position.xyz - fs_in.position);
    #endif
}

// Phong lighting model
vec4 lighting_model(
    Light light,
    vec3 light_dir,
    float attenuation
) {
    vec3 norm = get_normal();
    float specular_strength = uniforms.metallic_factor * texture(sampler2D(t_pbr, s_pbr), fs_in.tex_coords).b;
    vec4 model_base_color = uniforms.base_color * texture(sampler2D(t_base_color, s_base_color), fs_in.tex_coords);

    // Ambient light
    vec3 ambient = light.ambient.a * light.ambient.rgb;

    // Diffuse light
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light.diffuse.rgb;

    // Specular light
    vec3 view_dir = get_view_dir();
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light.specular.rgb;

    // Emission
    vec4 emission = texture(sampler2D(t_emission, s_emission), fs_in.tex_coords);

    // Mix lights
    vec3 light_result = (ambient + diffuse + specular) * attenuation * fs_in.color.rgb;
    return vec4(light_result, fs_in.color.a) * model_base_color + emission;
}

#include "light_caster.glsl" // Import calculate_light()

void main() {
    vec4 result = vec4(0.0);
    uint number_of_lights = min(uniforms.number_of_lights, MAX_NUMBER_OF_LIGHTS);
    for (int i = 0; i < number_of_lights; i++) {
        result += calculate_light(
            lights[i],
            fs_in.tangent_light_pos[i],
            fs_in.tangent_light_dir[i]);
    }
    out_color = result;
}
