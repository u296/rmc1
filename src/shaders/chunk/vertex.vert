#version 420

uniform mat4 projection;

uniform mat4 view_rotation;
uniform mat4 view_translation;
uniform mat4 model_rotation;
uniform mat4 model_translation;

                

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec3 normal;

out vec2 v_texcoord;
out vec3 v_normal;
out vec3 v_local_pos;
                
void main() {
    

    vec3 local_vertex_position = (view_rotation * view_translation * model_rotation * model_translation * vec4(position, 1.0)).xyz;
    gl_Position = projection * view_rotation * view_translation * model_rotation * model_translation * vec4(position, 1.0);



    v_texcoord = uv;
    v_normal = normalize(normal);//normalize((view_rotation * model_rotation * vec4(normal, 1.0)).xyz);
    v_local_pos = local_vertex_position;
}