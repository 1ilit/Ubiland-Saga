#version 140

in vec2 v_tex_coords;
in vec4 ourColor;

out vec4 fragColor;

uniform bool isTex;
uniform sampler2D tex;
uniform bool clipped;
uniform float c_x;
uniform float c_y;
uniform float c_w;
uniform float c_h;

void main() {
    if(isTex){
        fragColor = texture(tex, v_tex_coords);
        if(clipped){
            if(v_tex_coords.x<0||
               v_tex_coords.x>c_x+c_w || 
               v_tex_coords.y<0||
               v_tex_coords.y<1-(c_y+c_h))
                discard;
        }
    }
    else
        fragColor = ourColor;
}