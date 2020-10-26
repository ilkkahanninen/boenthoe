#version 450
#include "uniforms.glsl"

void main() {
    vec4 color = texture(sampler2D(t_primary, s_primary), v_tex_coords);
    float luma = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
    if (luma > 0.5) {
        out_color = color;
    } else {
        out_color = vec4(0.0, 0.0, 0.0, color.a);
    }
}
