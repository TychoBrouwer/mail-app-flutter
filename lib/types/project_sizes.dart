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

  static final Radius _radius =
      Radius.circular(Configuration().getValue<double>('appSizes:radius')!);
  static final Radius _radiusSmall = Radius.circular(
      Configuration().getValue<double>('appSizes:radiusSmall')!);
  static final Radius _radiusExtraSmall = Radius.circular(
      Configuration().getValue<double>('appSizes:radiusExtraSmall')!);
  static final Radius _radiusLarge = Radius.circular(
      Configuration().getValue<double>('appSizes:radiusLarge')!);

  static final BorderRadius borderRadius = BorderRadius.all(_radius);
  static final BorderRadius borderRadiusSmall = BorderRadius.all(_radiusSmall);
  static final BorderRadius borderRadiusExtraSmall =
      BorderRadius.all(_radiusExtraSmall);
  static final BorderRadius borderRadiusLarge = BorderRadius.all(_radiusLarge);
}
