#version 420


uniform sampler2D color;
uniform sampler2D depth;
uniform sampler2D normal;

in vec2 uv;

out vec4 f_color;

void main() {
    vec4 frag_color = texture(color, uv);
    float frag_depth = texture(depth, uv).x;
    vec3 frag_normal = (texture(normal, uv).xyz * 2) - vec3(1,1,1);

    float a = frag_depth + 0.2;
    a = clamp(0, 1, a*a*a);

    float light = clamp(0,1, clamp(0, 1, dot(vec3(0, 1, 0), frag_normal)) + 0.4);


    if (frag_color.w == 1) {

    }

    frag_color.xyz *= light;

    f_color = frag_color;
}