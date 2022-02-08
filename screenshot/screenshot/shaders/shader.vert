#version 450

// in
layout(set = 0, binding = 0) readonly buffer PositionsBuffer
{
    vec4 data[];
} positionsBuffer;

layout(set = 0, binding = 1) readonly buffer ColorsBuffer
{
    vec4 data[];
} colorsBuffer;

// out
out gl_PerVertex
{
    vec4 gl_Position;
};

layout (location = 0) out vec4 outColor;

void main()
{
    gl_Position = positionsBuffer.data[gl_VertexIndex];
    outColor = colorsBuffer.data[gl_VertexIndex];
}