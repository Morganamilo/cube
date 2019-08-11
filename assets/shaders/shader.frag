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
uniform float shininess;

void main() {

    // Output color = color of the texture at the specified UV
    //color = texture( TextureSampler, UV ).rgb;

    //vec3 lightColor = vec3(1.0, 1.0, 1.0);
    vec3 lightColor = vec3(0.2, 0.2, 0.2);
    vec3 lightPos = vec3(0.0, -2.0, 2.0);
    vec3 viewPos = vec3(0.0);

    // ambient
    vec3 ambient = lightColor * ambient;

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
