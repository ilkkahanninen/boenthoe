#version 450
#include "uniforms.glsl"

void main() {
    vec2 direction = vec2(effect_layer.args.w, 1.0 - effect_layer.args.w);
    vec2 delta = direction * vec2(effect_layer.args.x);
    int samples = int(effect_layer.args.z);

    vec2 uv = v_tex_coords + direction * vec2(effect_layer.args.y);

    vec4 color = vec4(0.0);
    for (int i = 0; i < samples; i++, uv += delta) {
        color += texture(sampler2D(t_primary, s_primary), uv);
    }

    out_color = color / float(samples);
}
