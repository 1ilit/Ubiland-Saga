#version 140

in vec2 v_tex_coords;
in vec4 ourColor;

out vec4 fragColor;

uniform bool isTex;
uniform sampler2D tex;

void main() {
    if(isTex){
        fragColor = texture(tex, v_tex_coords);
        if(v_tex_coords.x>0.5){
            discard;
        }
    }
    else
        fragColor = ourColor;
}