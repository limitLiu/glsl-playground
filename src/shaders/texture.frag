#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D textureObj;
uniform float time;

void main() {
    float duration = 0.6;
    float maxScale = 1.05;
    float offset = 0.02;

    float progress = mod(time, duration) / duration;
    vec2 offsetCoords = vec2(offset, offset) * progress;
    float scale = 1.0 + (maxScale - 1.0) * progress;
    vec2 scaleTextureCoords = vec2(0.5, 0.5) + (TexCoord - vec2(0.5, 0.5)) / scale;

    vec4 maskR = texture(textureObj, scaleTextureCoords + offsetCoords);
    vec4 maskB = texture(textureObj, scaleTextureCoords - offsetCoords);
    vec4 mask = texture(textureObj, scaleTextureCoords);

    FragColor = vec4(maskR.r, mask.g, maskB.b, mask.a);
}
