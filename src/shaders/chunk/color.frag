#version 420

                
uniform sampler2D atlas;
                

in vec2 v_texcoord;
in vec3 v_normal;
in vec3 v_local_pos;

out vec4 f_color;

void main() {

    

    vec4 basecolor = texture(atlas, v_texcoord);
    f_color = basecolor;
}