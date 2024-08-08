#version 320 es
precision mediump float;

uniform vec2 uRes;
uniform vec2 uSize;

uniform vec4 uBGColor;
uniform vec4 uBDColor;
uniform float uBDWidth;
uniform float uBDRadius;

in vec2 fCoord;
out vec4 fColor;

float RndRectSDF(vec2 p, vec2 b, float r) {
    vec2 d = abs(p) - b + vec2(r);
    return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - r;
}

void main() {
    float hb = uBDWidth / 2.0;
    float r = uBDRadius - hb;
    vec2 center = fCoord * uSize / 2.0;
    vec2 size = uSize / 2.0 - hb;

    float d = RndRectSDF(center, size, r);

    vec4 bColor = uBDColor;
    vec4 color = vec4(0.0);

    if (d < 0.0) {
        color = uBGColor;
    }
    d = abs(d) - hb;

    float blend = smoothstep(-1.0, 1.0, d);

    fColor = mix(bColor, color, blend);
}
