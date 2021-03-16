#version 420

                
uniform sampler2D atlas;
                

in vec2 v_texcoord;
in vec3 v_normal;

out vec4 fragColor;

void main() {
    float modifier = 1;

    if (abs(v_normal.x) > 0.1) {
        modifier = 0.9;
    }

    

    vec4 basecolor = texture(atlas, v_texcoord);
    basecolor.xyz *= modifier;
    fragColor = basecolor;
}