#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set = 1, binding = 0) uniform Uniforms {
    float u_zoom;
    float u_brightness;
    float u_lsd;
    float u_fade;
};

float rand(float n){return fract(sin(n) * 43758.5453123);}

void main() {
    vec2 zoom = (vec2(0.5, 0.5) - v_tex_coords) * u_zoom;

    vec2 distort1 = reflect(v_tex_coords, sin(zoom * length(zoom) * 16.0)) * length(zoom) * u_lsd;
    vec2 distort2 = sin(zoom) * cos(length(zoom) * u_lsd) * u_lsd;
    vec2 distort = mix(distort1, distort2, vec2(u_lsd / 2.0));

    vec3 color = pow(
        texture(
            sampler2D(t_diffuse, s_diffuse),
            v_tex_coords + zoom + distort
        ).xyz,
        vec3(1.25 - u_brightness, 1.2 - u_brightness, 1.1 - u_brightness)
    );
    f_color = vec4(color * u_fade * (0.95 + 0.1 * rand(u_zoom + u_brightness + u_lsd + sin(v_tex_coords.x * 1231.2) + sin(v_tex_coords.y * 3213.2) )), 1.0);
}
