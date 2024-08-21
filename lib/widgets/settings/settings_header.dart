import 'package:flutter/material.dart';

import '../../types/project_colors.dart';
import '../../types/project_sizes.dart';
import '../../types/settings_tab.dart';
import '../custom_button.dart';
import '../custom_icon_button.dart';

class SettingsHeader extends StatefulWidget {
  final void Function() closeSettings;
  final void Function(SettingsTab) showPage;

  const SettingsHeader({
    super.key,
    required this.closeSettings,
    required this.showPage,
  });

  @override
  SettingsHeaderState createState() => SettingsHeaderState();
}

class SettingsHeaderState extends State<SettingsHeader> {
  late void Function() _closeSettings;
  late void Function(SettingsTab) _showPage;

  SettingsTab _activeTab = SettingsTab.Accounts;

  @override
  void initState() {
    super.initState();

    _closeSettings = widget.closeSettings;
    _showPage = widget.showPage;
  }

  void _updatePage(SettingsTab tab) {
    if (tab == _activeTab) return;

    setState(() {
      _activeTab = tab;
    });

    _showPage(tab);
  }

  Widget headerButton(String text, bool active, void Function() onTap) {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 10),
      child: CustomButton(
        onTap: onTap,
        active: active,
        child: Container(
          padding:
              const EdgeInsets.only(left: 10, right: 10, top: 3, bottom: 5),
          child: Text(
            text,
            style: TextStyle(
              color: ProjectColors.text(true),
              fontSize: ProjectSizes.fontSize,
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: ProjectColors.header(true),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
      child: Row(
        children: [
          CustomIconButton(onTap: _closeSettings, icon: "chevron-left"),
          const Spacer(),
          for (var tab in SettingsTab.values)
            headerButton(tab.toString().split('.').last, _activeTab == tab,
                () => _updatePage(tab)),
          const Spacer(),
        ],
      ),
    );
  }
}
