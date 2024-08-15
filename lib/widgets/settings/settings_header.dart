import 'package:flutter/material.dart';

import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_icon_button.dart';

class SettingsHeader extends StatefulWidget {
  final void Function() closeSettings;

  const SettingsHeader({
    super.key,
    required this.closeSettings,
  });

  @override
  SettingsHeaderState createState() => SettingsHeaderState();
}

class SettingsHeaderState extends State<SettingsHeader> {
  late void Function() _closeSettings;

  @override
  void initState() {
    super.initState();

    _closeSettings = widget.closeSettings;
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
        ],
      ),
    );
  }
}
