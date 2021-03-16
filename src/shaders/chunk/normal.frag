#version 420

in vec2 v_texcoord;
in vec3 v_normal;
in vec3 v_local_pos;

out vec4 f_color;

void main() {
    f_color = vec4(v_normal, 1.0);
}