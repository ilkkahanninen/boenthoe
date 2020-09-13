#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_input;
layout(set = 0, binding = 1) uniform sampler s_input;

layout(set = 1, binding = 0) uniform texture2D t_vignette;
layout(set = 1, binding = 1) uniform sampler s_vignette;

void main() {
    float v_x = texture(sampler2D(t_vignette, s_vignette), v_tex_coords).r;
    vec4 v_input = texture(sampler2D(t_input, s_input), v_tex_coords);
    f_color = v_input * v_x;
}
