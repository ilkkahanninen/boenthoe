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
