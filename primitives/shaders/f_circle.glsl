#version 320 es
precision mediump float;

uniform vec2 uRes;
uniform vec2 uSize;

uniform vec4 uBGColor;
uniform vec4 uBDColor;
uniform float uBDWidth;

in vec2 fCoord;
out vec4 fColor;

void main() {
    float fade = 3.0 / min(uSize.x, uSize.y);

    float d = 1.0 - length(fCoord);
    float dOut = smoothstep(0.0, fade, d);

    float dIn = smoothstep(1.0 - fade, 1.0, length(fCoord * (1.0 + uBDWidth / uRes * 2.0)));
    float dWhole = dOut * dIn;

    vec3 col = vec3(uBGColor.rgb * (1.0 - dIn));
    col += vec3(uBDColor.rgb * dWhole);

    if (dOut == 0.0)
        discard;

    fColor = vec4(col, dOut);
}
