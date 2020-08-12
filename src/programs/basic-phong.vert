attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;
uniform   mat4 mvpMatrix;
uniform   mat4 mMatrix; // モデル座標変換行列。frag shaderと共有
varying   vec3 vPosition; // World座標系での位置
varying   vec3 vNormal;   // Local座標系での法線ベクトル
varying   vec4 vColor;

void main(void){
  vPosition   = (mMatrix * vec4(position, 1.0)).xyz;
  vNormal     = normal;
  vColor      = color;
  gl_Position = mvpMatrix * vec4(position, 1.0);
}
