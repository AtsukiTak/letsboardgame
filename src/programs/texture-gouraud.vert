attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;
attribute vec2 texCoord;

uniform   mat4 mvpMatrix;
uniform   mat4 mMatrix; // モデル座標変換行列
uniform   mat4 invMMatrix; // mMatrixの逆行列。WebGL1.0ではinverse関数をサポートしていない
uniform   int  lightType;  // 0: 光源なし, 1: 平行光源, 2: 点光源
uniform   vec3 lightVal;   // 平行光源のときdirection, 点光源のときposition
uniform   vec3 eyeDirection; // カメラの視線方向
uniform   vec4 ambientColor; // 環境光

varying   vec4 vColor; // 各頂点における色
varying   vec2 vTexCoord; // 各頂点におけるテクスチャの座標

vec3 invLight() {
  // World座標系における頂点座標
  vec3 worldPos = (mMatrix * vec4(position, 1.0)).xyz;
  vec3 lightDir = (lightType == 1) ? lightVal : worldPos - lightVal;
  return normalize(invMMatrix * vec4(-lightDir, 0.0)).xyz;
}

vec4 diffuse() {
  float diffuseVal = clamp(dot(normal, invLight()), 0.0, 1.0);
  return vec4(vec3(diffuseVal), 1.0);
}

vec4 specular() {
  vec3 invEye = normalize(invMMatrix * vec4(-eyeDirection, 0.0)).xyz;
  vec3 halfLE = normalize(invLight() + invEye);
  float specularVal = pow(clamp(dot(normal, halfLE), 0.0, 1.0), 50.0);
  return vec4(vec3(specularVal), 0.0);
}

void main(void) {
  // World座標系での頂点座標
  gl_Position = mvpMatrix * vec4(position, 1.0);

  vColor = (lightType == 0)
    ? color + ambientColor
    : color * diffuse() + specular() + ambientColor;

  vTexCoord = texCoord;
}
