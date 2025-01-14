#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

out vec4 outColor;

mat4 projection;

uniform float elapsed; // NOTE: The uniform can also be passed in the VAO as a vertex attribute

void main()
{
    projection[0] = vec4(1.0f, 0.f, 0.0f, 0.0f);
    projection[1] = vec4(0.0f, 1.0f, 0.0f, 0.0f);
    projection[2] = vec4(0.0f, 0.0f, 1.0f, 0.0f);
    projection[3] = vec4(0.0f, 0.0f, 0.0f, 1.0f);
    gl_Position = projection * vec4(position, 1.0f);
    outColor = color;
}