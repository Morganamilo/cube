#version 320 es
#
// Input vertex data, different for all executions of this shader.
layout(location = 0) in vec3 vertex;
layout(location = 1) in vec2 vertexUV;
layout(location = 2) in vec3 vertexNormal;

// Output data ; will be interpolated for each fragment.
out vec2 UV;
out vec3 normal;
out vec3 fragPos;

// Values that stay constant for the whole mesh.
uniform mat4 MVP;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {

    // Output position of the vertex, in clip space : MVP * position
    fragPos = vec3(model * vec4(vertex, 1.0));
    normal = mat3(transpose(inverse(model))) * vertexNormal;
    gl_Position = projection * view * vec4(fragPos, 1.0);
    UV = vertexUV;
}
