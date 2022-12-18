import 'dart:io';

import 'package:mail_app/types/dimensions.dart';
import 'package:mail_app/types/mail_account.dart';
import 'package:mail_app/utils/wait_until.dart';
import '../utils/local_file_store.dart';

class LocalSettings {
  late LocalFileStore _fileStore;
  late Settings _settings;

  bool _loaded = false;

  LocalSettings(LocalFileStore fileStore) {
    _fileStore = fileStore;

    _loadIni();
  }

  Future<void> _loadIni() async {
    final settingsFile = await _fileStore.readLocalFile('settings.json');

    if (settingsFile.isEmpty) await createSettingsFile();

    final dimensions = Dimensions(settingsFile['dimensions']['width'],
        settingsFile['dimensions']['height']);

    List<dynamic> accountsObject = settingsFile['accounts'];
    final accounts = accountsObject
        .map((account) => MailAccount(
            account['email'],
            account['password'],
            account['imapAddress'],
            account['imapPort'],
            account['smtpAddress'],
            account['smtpPort']))
        .toList();

    _settings =
        Settings(dimensions, accounts: accounts, theme: settingsFile['theme']);

    _loaded = true;
  }

  Future<bool> loaded() async {
    return await waitUntil(() => _loaded);
  }

  Future<void> createSettingsFile() async {
    Dimensions dimensions = Dimensions(600, 400);
    List<MailAccount> accounts = [
      MailAccount('test1928346534@gmail.com', 'xsccljyfbfrgvtjw',
              'imap.gmail.com', 993, 'smtp.gmail.com', 993)
          .accountJson(),
    ];
    const theme = 'dark';

    _settings = Settings(dimensions, accounts: accounts, theme: theme);

    saveSettings();

    return;
  }

  Settings getSettings() {
    return _settings;
  }

  Future<File> saveSettings() async {
    return await _fileStore.writeLocalFile(
        _settings.settingsMap(), 'settings.json');
  }

  Future<File> saveIniFromObject(Map<String, dynamic> newIni) async {
    return await _fileStore.writeLocalFile(newIni, 'settings.json');
  }

  void addAccount(MailAccount accountToAdd) {}
}

class Settings {
  final Dimensions _dimensions;
  final List<MailAccount> _accounts;
  late String _theme;

  Settings(dimensions, {accounts = const [], theme = 'dark'})
      : _dimensions = dimensions,
        _accounts = accounts,
        _theme = theme;

  Map<String, dynamic> settingsMap() {
    return {
      'dimensions': _dimensions,
      'accounts': _accounts,
      'theme': _theme,
    };
  }

  List<MailAccount> accounts() {
    return _accounts;
  }

  void addAccount(MailAccount account) {
    _accounts.add(account);
  }

  void removeAccount(String email) {
    _accounts.removeWhere((account) => account.email == email);
  }

  void setTheme(String theme) {
    _theme = theme;
  }

  void setDimensions(int width, int height) {
    _dimensions.width = width;
    _dimensions.height = height;
  }
}
