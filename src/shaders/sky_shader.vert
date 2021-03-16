#version 420

uniform mat4 view_rotation;
uniform mat4 projection;

layout(location = 0) in vec3 position;

out vec3 v_position;

void main() {
    v_position = position;
    gl_Position = projection * view_rotation * vec4(position, 1.0);
}