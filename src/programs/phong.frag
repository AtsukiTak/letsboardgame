precision mediump float;

uniform mat4 invMatrix;     // モデル座標変換行列の逆行列
uniform int  lightType; // 0: 光源なし, 1: 平行光源, 2: 点光源
uniform vec3 lightVal; // 平行光源のときdirection, 点光源のときposition
uniform vec3 eyeDirection;
uniform vec4 ambientColor;
varying vec3 vPosition;     // World座標系での位置
varying vec3 vNormal;       // Local座標系での法線ベクトル
varying vec4 vColor;

void main(void) {
  if (lightType == 0) {
    // 光源なし
    gl_FragColor    = vColor;
  } else {
    vec3  lightDir = lightVal; // 平行光源の場合
    if (lightType == 2) {
      // 点光源の場合
      lightDir = vPosition - lightVal;
    }

    vec3  invLight  = normalize(invMatrix * vec4(-lightDir, 0.0)).xyz;
    vec3  invEye    = normalize(invMatrix * vec4(-eyeDirection, 0.0)).xyz;
    vec3  halfLE    = normalize(invLight + invEye);
    float diffuse   = clamp(dot(vNormal, invLight), 0.0, 1.0);
    float specular  = pow(clamp(dot(vNormal, halfLE), 0.0, 1.0), 50.0);
    vec4  destColor = vColor * vec4(vec3(diffuse), 1.0) + vec4(vec3(specular), 1.0) + ambientColor;
    gl_FragColor    = destColor;
  }
}
