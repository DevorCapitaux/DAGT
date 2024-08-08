#version 320 es
precision mediump float;

in vec2 aPos;
out vec2 fCoord;

void main() {
    fCoord = aPos;
    gl_Position = vec4(aPos, 1.0, 1.0);
}
