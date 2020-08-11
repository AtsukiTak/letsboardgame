precision mediump float;

uniform sampler2D uTexture;

varying vec4 vColor;
varying vec2 vTexCoord;

void main(void) {
  gl_FragColor = vColor * texture2D(uTexture, vTexCoord);
}
