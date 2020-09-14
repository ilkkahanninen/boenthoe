#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

int samples = 16;
float size = 0.05;

void main() {
    float delta = size / float(samples - 1);
    vec4 acc_color = vec4(0.0, 0.0, 0.0, 0.0);
    float x = -size / 2.0;
    for (int i = 0; i < samples; i++, x += delta) {
        acc_color += texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords + vec2(x, 0.0));
    }
    f_color = acc_color / float(samples);
}
