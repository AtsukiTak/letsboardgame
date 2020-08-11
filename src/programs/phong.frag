precision mediump float;

uniform mat4 invMMatrix; // モデル座標変換行列の逆行列
uniform int  lightType; // 0: 光源なし, 1: 平行光源, 2: 点光源
uniform vec3 lightVal; // 平行光源のときdirection, 点光源のときposition
uniform vec3 eyeDirection;
uniform vec4 ambientColor;
varying vec3 vPosition;     // World座標系での位置
varying vec3 vNormal;       // Local座標系での法線ベクトル
varying vec4 vColor;

vec3 invLight() {
  // lightType == 0 のとき、このパスを通らないようにする
  vec3 lightDir = (lightType == 1) ? lightVal : vPosition - lightVal;
    return normalize(invMMatrix * vec4(-lightDir, 0.0)).xyz;
}

vec4 diffuse() {
    float diffuseVal = clamp(dot(vNormal, invLight()), 0.0, 1.0);
    return vec4(vec3(diffuseVal), 1.0);
}

vec4 specular() {
    vec3  invEye    = normalize(invMMatrix * vec4(-eyeDirection, 0.0)).xyz;
    vec3  halfLE    = normalize(invLight() + invEye);
    float specularVal = pow(clamp(dot(vNormal, halfLE), 0.0, 1.0), 50.0);
    return vec4(vec3(specularVal), 1.0);
}

void main(void) {
  gl_FragColor = (lightType == 0)
    ? vColor + ambientColor
    : vColor * diffuse() + specular() + ambientColor;
}
