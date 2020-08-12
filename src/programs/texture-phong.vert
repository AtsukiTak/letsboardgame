attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;
attribute vec2 texCoord;

uniform   mat4 mvpMatrix;
uniform   mat4 mMatrix; // モデル座標変換行列

varying   vec3 vPosition; // World座標系での位置
varying   vec3 vNormal; // Local座標系での法線ベクトル
varying   vec4 vColor;
varying   vec2 vTexCoord;

void main(void) {
  vPosition   = (mMatrix * vec4(position, 1.0)).xyz;
  vNormal     = normal;
  vColor      = color;
  vTexCoord   = texCoord;
  gl_Position = mvpMatrix * vec4(position, 1.0);
}
