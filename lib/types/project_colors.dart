import 'package:flutter/material.dart' show Color;
import 'package:mail_app/services/global_configuration.dart';

class ProjectColors {
  static Color Function(bool) get accent => (bool main) => main
      ? Configuration().getValue<Color>('appColors:accent')!
      : Configuration().getValue<Color>('appColors:accentSecondary')!;

  static Color Function(bool) text = (bool main) => main
      ? Configuration().getValue<Color>('appColors:text')!
      : Configuration().getValue<Color>('appColors:textSecondary')!;

  static Color Function(bool) border = (bool main) => main
      ? Configuration().getValue<Color>('appColors:border')!
      : Configuration().getValue<Color>('appColors:borderSecondary')!;

  static Color Function(bool) background = (bool main) => main
      ? Configuration().getValue<Color>('appColors:background')!
      : Configuration().getValue<Color>('appColors:backgroundSecondary')!;

  static Color Function(bool) button = (bool main) => main
      ? Configuration().getValue<Color>('appColors:button')!
      : Configuration().getValue<Color>('appColors:buttonSecondary')!;

  static Color Function(bool) header = (bool main) => main
      ? Configuration().getValue<Color>('appColors:header')!
      : Configuration().getValue<Color>('appColors:headerSecondary')!;
}
