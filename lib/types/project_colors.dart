import 'package:flutter/material.dart';

class ProjectColors {
  static Color Function(bool) main = (bool active) => active
      ? const Color.fromRGBO(33, 33, 33, 1)
      : const Color.fromRGBO(200, 200, 200, 1);
  static Color Function(bool) secondary = (bool active) => active
      ? const Color.fromRGBO(33, 33, 33, 1)
      : const Color.fromRGBO(120, 120, 120, 1);

  static Color Function(bool) accent = (bool active) => active
      ? const Color.fromRGBO(200, 168, 218, 1)
      : const Color.fromRGBO(200, 168, 218, 0.5);
}
