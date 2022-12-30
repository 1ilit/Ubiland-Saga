#version 140

in vec2 v_tex_coords;
in vec4 ourColor;

out vec4 fragColor;

uniform bool isTex;
uniform sampler2D tex;
uniform bool clipped;
uniform vec2 start;
uniform vec2 size;

void main() {
    if(isTex) {
        fragColor = texture(tex, v_tex_coords);
        if(clipped) {
            vec2 min_coords = start / textureSize(tex, 0);
            vec2 max_coords = (start + size) / textureSize(tex, 0);

            if(v_tex_coords.x < min_coords.x || v_tex_coords.x > max_coords.x || 1 - v_tex_coords.y < min_coords.y || 1 - v_tex_coords.y > max_coords.y) {
                discard;
            }
        }
    } else
        fragColor = ourColor;
}