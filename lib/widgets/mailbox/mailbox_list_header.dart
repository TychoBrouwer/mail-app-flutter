import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart' show SvgPicture;

import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/widgets/custom_button.dart';

class MailboxHeader extends StatefulWidget {
  final Function composeMessage;
  final Function addMailAccount;

  const MailboxHeader({
    super.key,
    required this.composeMessage,
    required this.addMailAccount,
  });

  @override
  MailboxHeaderState createState() => MailboxHeaderState();
}

class MailboxHeaderState extends State<MailboxHeader> {
  late Function _composeMessage;
  late Function _addMailAccount;

  @override
  void initState() {
    super.initState();

    _composeMessage = widget.composeMessage;
    _addMailAccount = widget.addMailAccount;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: ProjectColors.header(true),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          CustomButton(
            onTap: () => _addMailAccount(),
            child: Padding(
              padding: const EdgeInsets.all(5),
              child: SvgPicture.asset(
                'assets/icons/square-plus.svg',
                colorFilter:
                    ColorFilter.mode(ProjectColors.text(true), BlendMode.srcIn),
                height: ProjectSizes.iconSize,
                width: ProjectSizes.iconSize,
              ),
            ),
          ),
          CustomButton(
            onTap: () => _composeMessage(),
            child: Padding(
              padding: const EdgeInsets.all(5),
              child: SvgPicture.asset(
                'assets/icons/paper-plane.svg',
                colorFilter:
                    ColorFilter.mode(ProjectColors.text(true), BlendMode.srcIn),
                height: ProjectSizes.iconSize,
                width: ProjectSizes.iconSize,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
