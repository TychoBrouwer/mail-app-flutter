import 'package:flutter/material.dart';

import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/settings_tab.dart';
import 'package:mail_app/widgets/settings/settings_header.dart';

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
