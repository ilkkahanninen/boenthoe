#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform Uniforms {
    float u_time;
    float u_scale;
};
layout(set = 1, binding = 0) uniform texture2D t_diffuse1;
layout(set = 1, binding = 1) uniform sampler s_diffuse1;
layout(set = 2, binding = 0) uniform texture2D t_diffuse2;
layout(set = 2, binding = 1) uniform sampler s_diffuse2;

void main() {
    vec4 color1 = texture(sampler2D(t_diffuse1, s_diffuse1), v_tex_coords);
    vec4 color2 = texture(sampler2D(t_diffuse2, s_diffuse2), v_tex_coords);
    float p = sin((v_tex_coords.x * 1.77778 + v_tex_coords.y) * u_scale + u_time);
    float f = smoothstep(-0.1, 0.1, p);
    f_color = f * color1 + (1.0 - f) * color2;
}
