// Effect layer uniforms

layout(set = 0, binding = 0) uniform EffectLayerUniforms {
    vec4 args;
    vec4 args2;
    float time;
} effect_layer;

// Inputs from vertex shader

layout(location=0) in vec2 v_tex_coords;

// Textures

layout(set = 1, binding = 0) uniform texture2D t_primary;
layout(set = 1, binding = 1) uniform sampler s_primary;
layout(set = 2, binding = 0) uniform texture2D t_secondary;
layout(set = 2, binding = 1) uniform sampler s_secondary;
layout(set = 3, binding = 0) uniform texture2D t_tertiary;
layout(set = 3, binding = 1) uniform sampler s_tertiary;
layout(set = 4, binding = 0) uniform texture2D t_quaternary;
layout(set = 4, binding = 1) uniform sampler s_quaternary;

// Output

layout(location=0) out vec4 out_color;
