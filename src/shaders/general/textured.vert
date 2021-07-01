#version 420

uniform mat4 projection;

uniform mat4 view_rotation;
uniform mat4 view_translation;
uniform mat4 model_rotation;
uniform mat4 model_translation;

                

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;

out vec2 v_texcoord;
                
void main() {
    

    vec3 local_vertex_position = (view_rotation * view_translation * model_rotation * model_translation * vec4(position, 1.0)).xyz;
    gl_Position = projection * view_rotation * view_translation * model_rotation * model_translation * vec4(position, 1.0);



    v_texcoord = uv;
}