#version 450

// in
layout(location = 0) in vec4 inColor;

// out
layout(location = 0) out vec4 outColor;

void main()
{
    outColor = inColor;
}