import 'package:flutter/material.dart' show Radius, BorderRadius;

class ProjectSizes {
  static const double fontSize = 12;
  static const double fontSizeSmall = 10;
  static const double fontSizeLarge = 15;
  static const double fontSizeExtraLarge = 22;

  static const double iconSize = 16;

  static const Radius _radius = Radius.circular(10);
  static const Radius _radiusSmall = Radius.circular(5);
  static const Radius _radiusExtraSmall = Radius.circular(3);
  static const Radius _radiusLarge = Radius.circular(15);

  static const BorderRadius borderRadius = BorderRadius.all(_radius);
  static const BorderRadius borderRadiusSmall = BorderRadius.all(_radiusSmall);
  static const BorderRadius borderRadiusExtraSmall =
      BorderRadius.all(_radiusExtraSmall);
  static const BorderRadius borderRadiusLarge = BorderRadius.all(_radiusLarge);
}
