#version 420

uniform sampler2D color;
uniform sampler2D depth;

in vec2 uv;

out vec4 f_color;

void main() {
    vec4 col = texture(color, uv);
    float depth = texture(depth, uv).x;

    depth = (depth - 0.8) * 5;

    f_color = vec4(col.xyz, 1.0) * depth;
}