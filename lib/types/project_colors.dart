import 'package:flutter/material.dart';

class ProjectColors {
  static Color Function(bool) main =
      (bool active) => active ? Colors.black87 : Colors.white60;
  static Color Function(bool) secondary =
      (bool active) => active ? Colors.black87 : Colors.white38;

  static Color color = const Color.fromARGB(255, 200, 168, 218);
  static MaterialColor accent = MaterialColorGenerator.from(color);
}

class MaterialColorGenerator {
  static MaterialColor from(Color color) {
    return MaterialColor(color.value, <int, Color>{
      50: color.withOpacity(0.1),
      100: color.withOpacity(0.2),
      200: color.withOpacity(0.3),
      300: color.withOpacity(0.4),
      400: color.withOpacity(0.5),
      500: color.withOpacity(0.6),
      600: color.withOpacity(0.7),
      700: color.withOpacity(0.8),
      800: color.withOpacity(0.9),
      900: color.withOpacity(1.0),
    });
  }
}
