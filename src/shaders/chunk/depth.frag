#version 420

uniform float render_distance;

in vec2 v_texcoord;
in vec3 v_normal;
in vec3 v_local_pos;

out vec4 f_color;

void main() {
    float dist = length(v_local_pos);
    float d = dist / render_distance;

    f_color = vec4(d, d, d, 1.0);
}