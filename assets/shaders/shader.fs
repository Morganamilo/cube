#version 320 es

precision highp float;

// Interpolated values from the vertex shaders
in vec2 UV;
in vec3 normal;
in vec3 fragPos;

// Ouput data
out vec4 color;

// Values that stay constant for the whole mesh.
uniform sampler2D TextureSampler;
uniform vec3 ambient;
uniform vec3 diffuse;
uniform vec3 specular;
uniform vec3 viewPos;
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform float shininess;

void main() {
    // ambient
    vec3 ambient = lightColor * ambient;
    ambient = ambient / 3.0;

    // diffuse
    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(lightPos - fragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = lightColor * (diff * diffuse);

    // specular
    vec3 viewDir = normalize(viewPos - fragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = lightColor * (spec * specular);

    vec3 result = ambient + diffuse + specular;
    color = vec4(result, 1.0);
    color = color * vec4(texture( TextureSampler, UV).rgb, 1.0);
}
