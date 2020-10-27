#version 450
#include "uniforms.glsl"

void main() {
    vec4 color = texture(sampler2D(t_primary, s_primary), v_tex_coords) * 0.2270270270;
    color = texture(sampler2D(t_primary, s_primary), v_tex_coords + effect_layer.args.xy) * 0.3162162162 + color;
    color = texture(sampler2D(t_primary, s_primary), v_tex_coords - effect_layer.args.xy) * 0.3162162162 + color;
    color = texture(sampler2D(t_primary, s_primary), v_tex_coords + effect_layer.args.zw) * 0.0702702703 + color;
    color = texture(sampler2D(t_primary, s_primary), v_tex_coords - effect_layer.args.zw) * 0.0702702703 + color;

    #ifdef BLEND_WITH_SECONDARY
        color += texture(sampler2D(t_secondary, s_secondary), v_tex_coords);
    #endif

    out_color = color;
}
