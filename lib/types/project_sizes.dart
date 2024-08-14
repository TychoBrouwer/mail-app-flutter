import 'package:flutter/material.dart' show Radius, BorderRadius;

class ProjectSizes {
  static const double fontSize = 12;
  static const double fontSizeSmall = 10;
  static const double fontSizeLarge = 15;
  static const double fontSizeExtraLarge = 22;

  static const double iconSize = 16;

  static const Radius radius = Radius.circular(10);
  static const Radius radiusSmall = Radius.circular(5);
  static const Radius radiusExtraSmall = Radius.circular(3);
  static const Radius radiusLarge = Radius.circular(15);

  static const BorderRadius borderRadius = BorderRadius.all(radius);
  static const BorderRadius borderRadiusSmall = BorderRadius.all(radiusSmall);
  static const BorderRadius borderRadiusExtraSmall =
      BorderRadius.all(radiusExtraSmall);
  static const BorderRadius borderRadiusLarge = BorderRadius.all(radiusLarge);
}
