import 'dart:io';
import 'dart:convert';

import 'package:path_provider/path_provider.dart';

class LocalFileStore {
  String _localPath = '';

  Future<String> _getLocalPath() async {
    _localPath = (await getApplicationSupportDirectory()).path;

    return _localPath;
  }

  Future<File> _getLocalFile(String filename,
      {final String directory = '\\'}) async {
    if (_localPath.isEmpty) await _getLocalPath();

    final file = File('$_localPath$directory$filename');

    return file;
  }

  Future<File> writeLocalFile(Map<String, dynamic> content, String filename,
      {final String directory = '\\'}) async {
    final file = await _getLocalFile(filename, directory: directory);

    String jsonData = const JsonEncoder.withIndent("  ").convert(content);

    return file.writeAsString(jsonData);
  }

  Future<Map<String, dynamic>> readLocalFile(String filename,
      {directory = '\\'}) async {
    try {
      final file = await _getLocalFile(filename, directory: directory);

      final contents = await file.readAsString();

      return jsonDecode(contents);
    } catch (e) {
      return {};
    }
  }
}
