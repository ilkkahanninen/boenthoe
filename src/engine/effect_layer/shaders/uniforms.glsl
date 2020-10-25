// Uniforms

layout(set = 0, binding = 0) struct EffectLayerUniforms {
    vec4 args;
    uint number_of_inputs;
    float time;
} effect_layer;

// Textures

layout(set = 1, binding = 0) uniform texture2D t_primary;
layout(set = 1, binding = 1) uniform sampler s_primary;
layout(set = 2, binding = 0) uniform texture2D t_secondary;
layout(set = 2, binding = 1) uniform sampler s_secondary;
layout(set = 3, binding = 0) uniform texture2D t_tertiary;
layout(set = 3, binding = 1) uniform sampler s_tertiary;
layout(set = 4, binding = 0) uniform texture2D t_quaternary;
layout(set = 4, binding = 1) uniform sampler s_quaternary;
