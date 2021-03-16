#version 420

uniform float aspect_ratio;
uniform float scale;

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(vec3(position.x / aspect_ratio, position.y, 0.0) * scale, 1.0);
}