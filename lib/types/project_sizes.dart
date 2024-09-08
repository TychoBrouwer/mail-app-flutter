import 'package:flutter/material.dart' show Radius, BorderRadius;

import '../services/global_configuration.dart';

class ProjectSizes {
  static final double fontSize =
      Configuration().getValue<double>('appSizes:fontSize')!;
  static final double fontSizeSmall =
      Configuration().getValue<double>('appSizes:fontSizeSmall')!;
  static final double fontSizeLarge =
      Configuration().getValue<double>('appSizes:fontSizeLarge')!;
  static final double fontSizeExtraLarge =
      Configuration().getValue<double>('appSizes:fontSizeExtraLarge')!;

  static final double iconSize =
      Configuration().getValue<double>('appSizes:iconSize')!;

  static final double radius =
      Configuration().getValue<double>('appSizes:radius')!;
  static final double radiusSmall =
      Configuration().getValue<double>('appSizes:radiusSmall')!;
  static final double radiusExtraSmall =
      Configuration().getValue<double>('appSizes:radiusExtraSmall')!;
  static final double radiusLarge =
      Configuration().getValue<double>('appSizes:radiusLarge')!;

  static final BorderRadius borderRadius =
      BorderRadius.all(Radius.circular(radius));
  static final BorderRadius borderRadiusSmall =
      BorderRadius.all(Radius.circular(radiusSmall));
  static final BorderRadius borderRadiusExtraSmall =
      BorderRadius.all(Radius.circular(radiusExtraSmall));
  static final BorderRadius borderRadiusLarge =
      BorderRadius.all(Radius.circular(radiusLarge));
}
