#version 320 es
precision mediump float;

uniform vec2 uRes;
uniform vec2 uSize;

in vec2 fCoord;
out vec4 fColor;

void main() {
    // fColor = vec4(1.0, 0.0, 0.0, 1.0);
    fColor = vec4(1., 1.0, 1.0, 1.0);
}
