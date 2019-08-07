#include <cmath>

struct ColorRGB {
  int r;
  int g;
  int b;
};

struct ColorLAB {
  double l;
  double a;
  double b;
};

ColorLAB rgb2lab(const ColorRGB& color) {
  auto r = (double)color.r / 255;
  auto g = (double)color.g / 255;
  auto b = (double)color.b / 255;

  r = (r > 0.04045) ? pow((r + 0.055) / 1.055, 2.4) : r / 12.92;
  g = (g > 0.04045) ? pow((g + 0.055) / 1.055, 2.4) : g / 12.92;
  b = (b > 0.04045) ? pow((b + 0.055) / 1.055, 2.4) : b / 12.92;

  auto x = (r * 0.4124 + g * 0.3576 + b * 0.1805) / 0.95047;
  auto y = (r * 0.2126 + g * 0.7152 + b * 0.0722) / 1.00000;
  auto z = (r * 0.0193 + g * 0.1192 + b * 0.9505) / 1.08883;

  x = (x > 0.008856) ? pow(x, 1.0 / 3) : (7.787 * x) + 16.0 / 116;
  y = (y > 0.008856) ? pow(y, 1.0 / 3) : (7.787 * y) + 16.0 / 116;
  z = (z > 0.008856) ? pow(z, 1.0 / 3) : (7.787 * z) + 16.0 / 116;

  return { (116 * y) - 16, 500 * (x - y), 200 * (y - z) };
}

double deltaE(const ColorLAB& labA, const ColorLAB& labB) {
  auto deltaL = labA.l - labB.l;
  auto deltaA = labA.a - labB.a;
  auto deltaB = labA.b - labB.b;

  auto c1 = sqrt(labA.a * labA.a + labA.b * labA.b);
  auto c2 = sqrt(labB.a * labB.a + labB.b * labB.b);

  auto deltaC = c1 - c2;
  auto deltaH = deltaA * deltaA + deltaB * deltaB - deltaC * deltaC;

  deltaH = deltaH < 0 ? 0 : sqrt(deltaH);
  auto sc = 1.0 + 0.045 * c1;
  auto sh = 1.0 + 0.015 * c1;
  auto deltaLKlsl = deltaL / (1.0);
  auto deltaCkcsc = deltaC / (sc);
  auto deltaHkhsh = deltaH / (sh);
  auto i = deltaLKlsl * deltaLKlsl + deltaCkcsc * deltaCkcsc + deltaHkhsh * deltaHkhsh;
  return i < 0 ? 0 : sqrt(i);
}

extern "C" double colorDiff(int r1, int g1, int b1, int r2, int g2, int b2) {
  auto labA = rgb2lab({ r1, g1, b1 });
  auto labB = rgb2lab({ r2, g2, b2 });

  return deltaE(labA, labB);
}
