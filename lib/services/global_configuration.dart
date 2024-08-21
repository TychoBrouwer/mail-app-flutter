import 'dart:io' show File, PathNotFoundException, Platform;
import 'dart:ui' show Color;

import 'package:flutter/services.dart' show rootBundle;

const List<String> validKeys = [
  "appColors",
  "appSizes",
];

const configName = "config.toml";

class Configuration {
  static final Configuration _singleton = Configuration._internal();

  factory Configuration() {
    return _singleton;
  }

  Configuration._internal();

  final Map<String, dynamic> _appConfig = {};

  final _localPath = Platform.isWindows
      ? "${Platform.environment["APPDATA"]}\\mail_app\\"
      : "${Platform.environment["HOME"]}/.config/mail_app/";

  Future<Configuration> loadFromAsset() async {
    String content = await rootBundle.loadString("assets/cfg/$configName");
    Map<String, dynamic> configAsMap = decodeToml(content);

    _appConfig.addAll(configAsMap);

    return _singleton;
  }

  Future<Configuration> loadFromLocal() async {
    await loadFromAsset();

    try {
      final file = File(_localPath + configName);

      String content = await file.readAsString();
      Map<String, dynamic> configAsMap = decodeToml(content);

      _appConfig.updateAll((key, value) => configAsMap[key] ?? value);
    } catch (e) {
      print("Error: $e");

      if (e is PathNotFoundException) {
        await save();
      }
    }

    return _singleton;
  }

  Future<void> save() async {
    final file = await File(_localPath + configName).create(recursive: true);

    await file.writeAsString(encodeToml(_appConfig));
  }

  T? getValue<T>(String keyPath) {
    dynamic value;

    keyPath.split(":").forEach((element) {
      if (value == null) {
        value = _appConfig[element];
      } else {
        value = value[element];
      }
    });

    if (value != null) {
      if (T == Color) {
        value = value as String;
        final rgbo = value.split(',');

        value = Color.fromRGBO(
          int.parse(rgbo[0]),
          int.parse(rgbo[1]),
          int.parse(rgbo[2]),
          double.parse(rgbo[3]),
        );
      } else if (T is double) {
        value = double.parse(value);
      } else if (T is int) {
        value = int.parse(value);
      }

      return value as T;
    }

    return null;
  }

  void updateValue(String key, dynamic value) {
    if (_appConfig[key] != null &&
        value.runtimeType != _appConfig[key].runtimeType) {
      throw ("The persistent type of ${_appConfig[key].runtimeType} does not match the given type ${value.runtimeType}");
    }

    if (value.runtimeType == Color) {
      final color = value as Color;
      value = "${color.red}, ${color.green}, ${color.blue}, ${color.opacity}";
    }

    _appConfig.update(key, (dynamic) => value);

    save();
  }
}

Map<String, dynamic> decodeToml(String content) {
  String key = "";
  Map<String, dynamic> result = {};

  for (var line in content.split("\n")) {
    if (line.startsWith("[")) {
      var posibleKey = line.substring(1, line.length - 1);

      if (posibleKey.endsWith("]")) {
        posibleKey = posibleKey.substring(0, posibleKey.length - 1);
      }

      if (validKeys.contains(posibleKey)) {
        key = posibleKey;
        result[key] = {};
      } else {
        key = "unknown";
        result[key] = {};
      }
    } else if (line.contains("=")) {
      var parts = line.split("=");

      parts[0] = parts[0].trim();
      dynamic value = parts[1].trim();

      try {
        value = double.parse(parts[1]);
      } catch (e) {
        if (value.startsWith("\"") && value.endsWith("\"")) {
          value = value.substring(1, value.length - 1);
        }
      }

      if (key.isNotEmpty) {
        result[key]![parts[0]] = value;
      } else {
        result[parts[0]] = value;
      }
    }
  }

  return result;
}

String encodeToml(Map<String, dynamic> content) {
  String result = "";

  content.forEach((key, value) {
    if (value is Map) {
      result += "[$key]\n";

      value.forEach((key, value) {
        if (value is String) {
          value = "\"$value\"";
        }
        result += "$key = $value\n";
      });

      result += "\n";
    } else {
      if (value is String) {
        value = "\"$value\"";
      }

      result += "$key = $value\n";
    }
  });

  return result;
}
