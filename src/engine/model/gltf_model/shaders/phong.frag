#version 450

struct Light {
    vec4 position;
    vec4 direction;
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    vec4 parameters;
    uint type; // 0 = off, 1 = directional, 2 = point, 3 = spotlight
};

// Bind group, index 0.0: Uniforms

layout(set=0, binding=0) uniform Uniforms {
    mat4 u_view_proj_matrix;
    mat4 u_model_matrix;
    vec4 u_eye_position;
    uint u_number_of_lights;
};

// Bind group, index 1.0: Lights

layout(set=1, binding=0) buffer Lights {
    Light u_lights[];
};

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
    float attenuation,
    float ambient_strength,
    float specular_strength
) {
    vec3 norm = normalize(a_normal);

    // Ambient light
    vec3 ambient = ambient_strength * light.ambient.rgb;

    // Diffuse light
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light.diffuse.rgb;

    // Specular light
    vec3 view_dir = normalize(u_eye_position.xyz - a_position);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light.specular.rgb;

    // Mix lights
    vec3 result = (ambient + diffuse + specular) * attenuation * a_color.rgb;
    return vec4(result, a_color.a);
}

vec4 calculate_light(Light light) {
    switch (light.type) {
        case 0: // Unlit
            return vec4(0.0);

        case 1: // Directional
            return phong_model(
                light,
                normalize(-light.direction.xyz),
                1.0,
                0.1,
                0.5
            );

        case 2: // Point
            vec3 light_vec = light.position.xyz - a_position;
            float distance = length(light_vec);
            return phong_model(
                light,
                normalize(light_vec),
                1.0 / (
                    light.parameters.x +
                    light.parameters.y * distance +
                    light.parameters.z * distance * distance),
                0.1,
                0.5
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
                intensity,
                0.1,
                0.5
            );

        default:
            return vec4(1.0, 0.0, 0.0, 0.0);
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
