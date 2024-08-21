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

  final _localPath = Platform.isWindows ? "~/AppData/Local/" : "~/.config/";

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

      _appConfig.addAll(configAsMap);
    } catch (e) {
      print("Error: $e");

      if (e is PathNotFoundException) {
        await save();
      }
    }

    return _singleton;
  }

  Future<void> save() async {
    final file = File(_localPath + configName);

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
      } else if (T == double) {
        value = double.parse(value);
      }

      if (T == int) {
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
      final posibleKey = line.substring(1, line.length - 2);

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
      parts[1] = parts[1].trim();

      if (parts[1].startsWith("\"") && parts[1].endsWith("\"")) {
        parts[1] = parts[1].substring(1, parts[1].length - 1);
      }

      if (key.isNotEmpty) {
        result[key]![parts[0]] = parts[1];
      } else {
        result[parts[0]] = parts[1];
      }
    }
  }

  return result;
}

String encodeToml(Map<String, dynamic> content) {
  String result = "";

  content.forEach((key, value) {
    result += "[$key]\n";

    if (value.runtimeType == Map) {
      value.forEach((key, value) {
        result += "$key = $value\n";
      });
    } else {
      result += "$value\n";
    }
  });

  return result;
}
