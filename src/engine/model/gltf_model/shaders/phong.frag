#version 450
#include "uniforms.glsl"

// Inputs from vertex shader

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coords;
layout(location=3) in vec4 a_color;

// Output

layout(location=0) out vec4 out_color;

vec4 phong_model(
    Light light,
    vec3 light_dir,
    float attenuation
) {
    float specular_strength = u_metallic_factor;
    vec3 norm = normalize(a_normal);
    vec4 model_base_color = u_base_color * texture(sampler2D(t_base_color, s_base_color), a_tex_coords);

    // Ambient light
    vec3 ambient = light.ambient.a * light.ambient.rgb;

    // Diffuse light
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light.diffuse.rgb;

    // Specular light
    vec3 view_dir = normalize(u_eye_position.xyz - a_position);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light.specular.rgb;

    // Mix lights
    vec3 light_result = (ambient + diffuse + specular) * attenuation * a_color.rgb;
    return vec4(light_result, a_color.a) * model_base_color;
}

vec4 calculate_light(Light light) {
    switch (light.type) {
        case 0: // Unlit
            return vec4(0.0, 0.0, 0.0, 1.0);

        case 1: // Directional
            return phong_model(
                light,
                normalize(-light.direction.xyz),
                1.0
            );

        case 2: // Point
            vec3 light_vec = light.position.xyz - a_position;

            float distance = length(light_vec);
            if (distance > light.parameters.x) {
                return vec4(0.0, 0.0, 0.0, 1.0);
            }

            return phong_model(
                light,
                normalize(light_vec),
                1.0 / (
                    light.parameters.y +
                    light.parameters.z * distance +
                    light.parameters.w * distance * distance)
            );

        case 3: // Spotlight
            float inner = light.parameters.x;
            float outer = light.parameters.y;

            vec3 light_dir = normalize(light.position.xyz - a_position);
            float theta = dot(light_dir, normalize(-light.direction.xyz));
            float epsilon = inner - outer;
            float intensity = clamp((theta - outer) / epsilon, 0.0, 1.0);

            return phong_model(
                light,
                light_dir,
                intensity
            );

        case 4: // Ambient
            return vec4(0.1 * light.ambient.rgb, 1.0);

        default:
            return vec4(1.0, 0.0, 0.0, 1.0);
    }
}

void main() {
    vec4 result;
    uint number_of_lights = min(u_number_of_lights, 64);
    for (int i = 0; i < number_of_lights; i++) {
        result += calculate_light(u_lights[i]);
    }
    out_color = result;
}
