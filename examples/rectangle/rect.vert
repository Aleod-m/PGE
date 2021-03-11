#version 330 core

layout (location = 0) in vec3 a_Pos;
layout (location = 1) in vec2 a_TextCoord;

out vec2 v_TextCoord;
void main() {
    v_TextCoord = a_TextCoord;
    gl_Position = vec4(a_Pos.x, a_Pos.y, a_Pos.z, 1.0);
}