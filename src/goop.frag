#version 150

uniform float circle_spacing;

// The coordinate of the pixel we're drawing, in the texture space.
in vec2 frag_texture;

// This is automatically assigned to be the color and transparency of the pixel
// we're responsible for.
out vec4 color;

void main() {
  // The portion of the plane to the left of the y axis we leave alone.
  if (frag_texture.x < 0)
    discard;

  // Which circle are we on?
  int circle = int(frag_texture.x / circle_spacing + 0.5) - 1;
  if (circle < 0 || circle >= 4096) {
    color = vec4(1.0, 1.0, 0.0, 1.0); // yellow: circle number out of range.
    return;
  }

  // Find the position of frag_texture relative to the circle's center.
  vec2 frag_circle = frag_texture;
  frag_circle.x -= (circle + 1) * circle_spacing;

  // Pixels outside the circle we leave alone.
  if (length(frag_circle) > 1.0)
    discard;

  // The circle index is between 0 and 4095. Treat it as a twelve-bit number,
  // break it into three groups of four bits each, and treat them as the red,
  // green, and blue values.
  float red = (circle >> 8) / 15.0;
  float blue = ((circle >> 4) & 0xf) / 15.0;
  float green = (circle & 0xf) / 15.0;
  color = vec4(red, blue, green, 1.0);
}