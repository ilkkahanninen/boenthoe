#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_input;
layout(set = 0, binding = 1) uniform sampler s_input;
layout(set = 1, binding = 0) uniform texture2D t_vignette;
layout(set = 1, binding = 1) uniform sampler s_vignette;
layout(set = 2, binding = 0) uniform texture2D t_dust;
layout(set = 2, binding = 1) uniform sampler s_dust;
layout(set = 3, binding = 0) uniform Uniforms {
    vec2 u_dust_xy;
    float u_dust_scale;
    float u_dust_opacity;
    float u_fade;
};

void main() {
    float vignette = texture(sampler2D(t_vignette, s_vignette), v_tex_coords).r;
    vec4 dust = texture(sampler2D(t_dust, s_dust), v_tex_coords * u_dust_scale + u_dust_xy) * u_dust_opacity;
    vec4 inputti = texture(sampler2D(t_input, s_input), v_tex_coords);

    f_color = pow(
        (
            inputti
            * vec4(1.1, 1.0, 0.9, 1.0)
            + vec4(0.1, 0.05, 0.0, 0.0)
            + dust
        ) * vignette * u_fade
        + vec4(0.0, 0.02, 0.04, 0.0),
        vec4(1.2, 1.2, 1.2, 1.0)
    );
}
