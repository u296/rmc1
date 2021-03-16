#version 420

in vec3 v_position;

out vec4 f_color;

void main() {
    vec3 sky_color = vec3(0.2, 0.5, 0.8);
    vec3 horizon_color = vec3(0.6, 0.7, 0.92);
    float t = clamp(normalize(v_position).y, 0, 1);
    f_color = vec4(mix(horizon_color, sky_color, t), 1.0);
}