#version 450
#include "uniforms.glsl"

void main() {
    float out_of_focus = abs(effect_layer.args2.y - texture(sampler2D(t_secondary, s_secondary), v_tex_coords).r);

    if (out_of_focus > effect_layer.args2.x) {
        vec4 color = texture(sampler2D(t_primary, s_primary), v_tex_coords) * 0.2270270270;
        color = texture(sampler2D(t_primary, s_primary), v_tex_coords + effect_layer.args.xy) * 0.3162162162 + color;
        color = texture(sampler2D(t_primary, s_primary), v_tex_coords - effect_layer.args.xy) * 0.3162162162 + color;
        color = texture(sampler2D(t_primary, s_primary), v_tex_coords + effect_layer.args.zw) * 0.0702702703 + color;
        color = texture(sampler2D(t_primary, s_primary), v_tex_coords - effect_layer.args.zw) * 0.0702702703 + color;
        out_color = color;
    } else {
        out_color = texture(sampler2D(t_primary, s_primary), v_tex_coords);
    }
}
