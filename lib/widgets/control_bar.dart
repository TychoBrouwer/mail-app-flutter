import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart' show SvgPicture;

import 'package:mail_app/types/message_flag.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/types/special_mailbox.dart';
import 'package:mail_app/widgets/custom_button.dart';

class ControlBar extends StatefulWidget {
  final void Function(MessageFlag) flagMessage;
  final void Function(SpecialMailboxType) moveMessage;
  final void Function() reply;
  final void Function() replyAll;
  final void Function() share;
  final void Function() settings;

  const ControlBar({
    super.key,
    required this.flagMessage,
    required this.moveMessage,
    required this.reply,
    required this.replyAll,
    required this.share,
    required this.settings,
  });

  @override
  ControlBarState createState() => ControlBarState();
}

class ControlBarState extends State<ControlBar> {
  late void Function(MessageFlag) _flagMessage;
  late void Function(SpecialMailboxType) _moveMessage;
  late void Function() _reply;
  late void Function() _replyAll;
  late void Function() _share;
  late void Function() _settings;

  @override
  void initState() {
    super.initState();

    _flagMessage = widget.flagMessage;
    _moveMessage = widget.moveMessage;
    _reply = widget.reply;
    _replyAll = widget.replyAll;
    _share = widget.share;
    _settings = widget.settings;
  }

  Widget controlWidget(Control control) {
    return CustomButton(
      onTap: () => control.function(),
      child: Padding(
        padding: const EdgeInsets.all(5),
        child: SvgPicture.asset(
          'assets/icons/${control.icon}.svg',
          colorFilter:
              ColorFilter.mode(ProjectColors.text(true), BlendMode.srcIn),
          alignment: Alignment.centerRight,
          height: ProjectSizes.iconSize,
          width: ProjectSizes.iconSize,
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
          controlWidget(
            Control(
                'box-archive', () => _moveMessage(SpecialMailboxType.archive)),
          ),
          controlWidget(
            Control(
                'circle-exclamation', () => _flagMessage(MessageFlag.Flagged)),
          ),
          controlWidget(
            Control('trash-can', () => _moveMessage(SpecialMailboxType.trash)),
          ),
          controlWidget(
            Control('envelope-dot', () => _flagMessage(MessageFlag.Seen)),
          ),
          controlWidget(Control('reply', _reply)),
          controlWidget(Control('reply-all', _replyAll)),
          controlWidget(Control('share', _share)),
          const Spacer(),
          controlWidget(Control('settings', _settings)),
        ],
      ),
    );
  }
}

class Control {
  final String icon;
  final void Function() function;

  Control(this.icon, this.function);
}
