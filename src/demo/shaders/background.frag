#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set = 1, binding = 0) uniform Uniforms {
    float u_zoom;
    float u_brightness;
};

void main() {
    vec2 zoom = (vec2(0.5, 0.5) - v_tex_coords) * u_zoom; // todo: make this unlinear
    // vec2 distort = sin(zoom) * length(zoom);
    vec2 distort = reflect(v_tex_coords, sin(zoom * length(zoom) * 16.0)) * length(zoom) * 0.05;
    vec3 color = pow(
        texture(
            sampler2D(t_diffuse, s_diffuse),
            v_tex_coords + zoom + distort
        ).xyz,
        vec3(1.25 - u_brightness)
    );
    f_color = vec4(color, 1.0);
}
