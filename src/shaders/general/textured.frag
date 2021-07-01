#version 420

                
uniform sampler2D texture;
                

in vec2 v_texcoord;

out vec4 f_color;

void main() {
    f_color = texture(texture, v_texcoord);
    f_color = basecolor;
}