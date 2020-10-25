#version 450
#define USE_NORMAL_MAPS
#include "uniforms.glsl"

// Inputs from vertex shader
layout(location=0) in VS_OUT fs_in;

// Output
layout(location=0) out vec4 out_color;

// Physically based rendering, copied from https://learnopengl.com/PBR/Lighting
float distribution_ggx(vec3 N, vec3 H, float roughness) {
    float a      = roughness*roughness;
    float a2     = a * a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH * NdotH;

    float num   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

float geometry_schlick_ggx(float NdotV, float roughness) {
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float num   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

float geometry_smith(vec3 N, vec3 V, vec3 L, float roughness) {
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2  = geometry_schlick_ggx(NdotV, roughness);
    float ggx1  = geometry_schlick_ggx(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 fresnel_schlick(float cos_theta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(1.0 - cos_theta, 5.0);
}

vec3 get_normal() {
    return normalize(texture(sampler2D(t_normal_map, s_normal_map), fs_in.tex_coords).rgb * 2.0 - 1.0);
}

vec3 get_view_dir() {
    return normalize(fs_in.tangent_view_pos - fs_in.tangent_position);
}

vec4 lighting_model(
    Light light,
    vec3 light_dir,
    float attenuation
) {
    vec3 albedo = pow(texture(sampler2D(t_base_color, s_base_color), fs_in.tex_coords).rgb, vec3(2.2));
    vec3 normal = get_normal();
    vec3 view_dir = get_view_dir();

    vec3 material_map = texture(sampler2D(t_pbr, s_pbr), fs_in.tex_coords).rgb;
    float metallic = material_map.b * uniforms.metallic_factor;
    float roughness = material_map.g * uniforms.roughness_factor;
    float occlusion = material_map.r;

    vec3 f0 = mix(vec3(0.04), albedo, metallic);
    vec3 half_dir = normalize(view_dir + light_dir);
    vec3 radiance = light.diffuse.rgb * attenuation;

    // Cook-Torrance bidirectional reflectance distribution function
    float ndf = distribution_ggx(normal, half_dir, roughness);
    float geometry = geometry_smith(normal, view_dir, light_dir, roughness);
    vec3 fresnel = fresnel_schlick(max(dot(half_dir, view_dir), 0.0), f0);

    vec3 kD = (vec3(1.0) - fresnel) * (1.0 - metallic);

    vec3 numerator = ndf * geometry * fresnel;
    float n_dot_l = max(dot(normal, light_dir), 0.0);
    float denominator = 4.0 * max(dot(normal, view_dir), 0.0) * n_dot_l;
    vec3 specular = numerator / max(denominator, 0.001);

    vec4 emission = texture(sampler2D(t_emission, s_emission), fs_in.tex_coords);

    vec3 result = (kD * albedo / PI + specular) * radiance * n_dot_l + emission.rgb;
    return vec4(result, 1.0);
}

#include "light_caster.glsl" // Import calculate_light()

void main() {
    vec4 result;
    uint number_of_lights = min(uniforms.number_of_lights, MAX_NUMBER_OF_LIGHTS);
    for (int i = 0; i < number_of_lights; i++) {
        result += calculate_light(
            lights[i],
            fs_in.tangent_light_pos[i],
            fs_in.tangent_light_dir[i]);
    }
    out_color = result;
}
