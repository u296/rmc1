#version 420

uniform mat4 persp_matrix;
uniform mat4 view_translation_matrix;
uniform mat4 model_matrix;

uniform mat4 combined_view_rot;

                

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 normal;

out vec2 v_texcoord;
out vec3 v_normal;
                
void main() {
    gl_Position = persp_matrix *  combined_view_rot * view_translation_matrix * model_matrix * vec4(position, 1.0);
    v_texcoord = uv;
    v_normal = normal;
}