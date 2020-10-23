#version 450
#include "uniforms.glsl"

#define USE_NORMAL_MAPS

// Inputs from vertex shader
layout(location=0) in VS_OUT fs_in;

// Output
layout(location=0) out vec4 out_color;

// Lighting
vec4 lighting_model(
    Light light,
    vec3 light_dir,
    float attenuation
) {
    #ifdef USE_NORMAL_MAPS
        vec3 norm = normalize(texture(sampler2D(t_normal_map, s_normal_map), fs_in.tex_coords).rgb * 2.0 - 1.0);
    #else
        vec3 norm = normalize(fs_in.normal);
    #endif

    float specular_strength = uniforms.metallic_factor;
    vec4 model_base_color = uniforms.base_color * texture(sampler2D(t_base_color, s_base_color), fs_in.tex_coords);

    // Ambient light
    vec3 ambient = light.ambient.a * light.ambient.rgb;

    // Diffuse light
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light.diffuse.rgb;

    // Specular light
    #ifdef USE_NORMAL_MAPS
        vec3 view_dir = normalize(fs_in.tangent_view_pos - fs_in.tangent_position);
    #else
        vec3 view_dir = normalize(uniforms.eye_position.xyz - fs_in.position);
    #endif
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light.specular.rgb;

    // Emission
    vec3 emission = texture(sampler2D(t_emission, s_emission), fs_in.tex_coords).rgb;

    // Mix lights
    vec3 light_result = (ambient + diffuse + specular) * attenuation * fs_in.color.rgb + emission;
    return vec4(light_result, fs_in.color.a) * model_base_color;
}

vec4 calculate_light(Light light, vec3 tangent_light_pos, vec3 tangent_light_dir) {
    switch (light.type) {
        case 0: { // Unlit
            return vec4(0.0, 0.0, 0.0, 1.0);
        }

        case 1: { // Directional
            #ifdef USE_NORMAL_MAPS
                vec3 light_dir = tangent_light_dir;
            #else
                vec3 light_dir = light.direction.xyz;
            #endif

            return lighting_model(
                light,
                normalize(-light_dir),
                1.0
            );
        }

        case 2: { // Point
            #ifdef USE_NORMAL_MAPS
                vec3 light_dir = tangent_light_pos - fs_in.tangent_position;
            #else
                vec3 light_dir = light.position.xyz - fs_in.position;
            #endif

            float distance = length(light_dir);
            if (distance > light.parameters.x) {
                return vec4(0.0, 0.0, 0.0, 1.0);
            }

            return lighting_model(
                light,
                normalize(light_dir),
                1.0 / (
                    light.parameters.y +
                    light.parameters.z * distance +
                    light.parameters.w * distance * distance)
            );
        }

        case 3: { // Spotlight
            float inner = light.parameters.x;
            float outer = light.parameters.y;

            #ifdef USE_NORMAL_MAPS
                vec3 light_dir = normalize(tangent_light_pos - fs_in.tangent_position);
                vec3 light_dir2 = normalize(light.position.xyz - fs_in.position);
            #else
                vec3 light_dir = normalize(light.position.xyz - fs_in.position);
                vec3 light_dir2 = light_dir;
            #endif

            float theta = dot(light_dir2, normalize(-light.direction.xyz));
            float epsilon = inner - outer;
            float intensity = clamp((theta - outer) / epsilon, 0.0, 1.0);

            return lighting_model(
                light,
                light_dir,
                intensity
            );
        }

        case 4: { // Ambient
            return vec4(0.1 * light.ambient.rgb, 1.0);
        }

        default:
            return vec4(1.0, 0.0, 0.0, 1.0);
    }
}

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
