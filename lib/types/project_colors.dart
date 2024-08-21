import 'package:flutter/material.dart' show Color;

class ProjectColors {
  static Color Function(bool) accent = (bool main) => main
      ? const Color.fromRGBO(200, 168, 218, 1)
      : const Color.fromRGBO(200, 168, 218, 0.5);

  static Color Function(bool) text = (bool main) => main
      ? const Color.fromRGBO(200, 200, 200, 1)
      : const Color.fromRGBO(120, 120, 120, 1);

  static Color Function(bool) border = (bool main) => main
      ? const Color.fromRGBO(200, 200, 200, 1)
      : const Color.fromRGBO(120, 120, 120, 1);

  static Color Function(bool) background = (bool main) => main
      ? const Color.fromRGBO(33, 33, 33, 1)
      : const Color.fromRGBO(52, 52, 52, 1);

  static Color Function(bool) button = (bool main) => main
      ? const Color.fromRGBO(52, 52, 52, 1)
      : const Color.fromRGBO(100, 100, 100, 1);

  static Color Function(bool) header = (bool main) => main
      ? const Color.fromRGBO(20, 20, 20, 1)
      : const Color.fromRGBO(52, 52, 52, 1);
}
