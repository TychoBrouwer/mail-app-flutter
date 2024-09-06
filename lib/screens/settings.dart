import 'package:flutter/material.dart';

import '../types/project_colors.dart';
import '../types/settings_tab.dart';
import '../widgets/settings/settings_account.dart';
import '../widgets/settings/settings_theme.dart';
import '../widgets/settings/settings_header.dart';
import '../widgets/settings/settings_customisation.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  SettingsPageState createState() => SettingsPageState();
}

class SettingsPageState extends State<SettingsPage> {
  Widget settingsPage = const SettingsAccount();

  _closeSettings() {
    if (!mounted) return;

    Navigator.pop(context);
  }

  _showPage(SettingsTab tab) {
    if (!mounted) return;

    switch (tab) {
      case SettingsTab.Theme:
        setState(() {
          settingsPage = const SettingsTheme();
        });
        break;
      case SettingsTab.Customisation:
        setState(() {
          settingsPage = const SettingsCustomisation();
        });
      case SettingsTab.Accounts:
        setState(() {
          settingsPage = const SettingsAccount();
        });
        break;
      default:
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.background(true),
        ),
        child: Center(
          child: Column(
            children: [
              SettingsHeader(
                closeSettings: _closeSettings,
                showPage: _showPage,
              ),
              Expanded(
                child: settingsPage,
              )
            ],
          ),
        ),
      ),
    );
  }
}
