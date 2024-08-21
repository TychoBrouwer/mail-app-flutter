import 'package:flutter/material.dart';

import '../types/project_colors.dart';
import '../types/settings_tab.dart';
import '../widgets/settings/settings_header.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  SettingsPageState createState() => SettingsPageState();
}

class SettingsPageState extends State<SettingsPage> {
  _closeSettings() {
    if (!mounted) return;

    Navigator.pop(context);
  }

  _showPage(SettingsTab tab) {
    // Show the page
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
            ],
          ),
        ),
      ),
    );
  }
}
