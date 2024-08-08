import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/types/message_flag.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';

class ControlBar extends StatefulWidget {
  final void Function(MessageFlag) flagMessage;
  final void Function() archive;
  final void Function() reply;
  final void Function() replyAll;
  final void Function() share;
  final void Function() settings;

  const ControlBar({
    super.key,
    required this.flagMessage,
    required this.archive,
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
  late void Function() _archive;
  late void Function() _reply;
  late void Function() _replyAll;
  late void Function() _share;
  late void Function() _settings;

  @override
  void initState() {
    super.initState();

    _flagMessage = widget.flagMessage;
    _archive = widget.archive;
    _reply = widget.reply;
    _replyAll = widget.replyAll;
    _share = widget.share;
    _settings = widget.settings;
  }

  Widget controlWidget(Control control) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 5),
      child: CustomButton(
        onTap: () => control.function(),
        child: Padding(
          padding: const EdgeInsets.all(5),
          child: SvgPicture.asset(
            'assets/icons/${control.icon}.svg',
            colorFilter:
                ColorFilter.mode(ProjectColors.main(false), BlendMode.srcIn),
            alignment: Alignment.centerRight,
            height: 20,
            width: 20,
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 10, bottom: 15, left: 50, right: 50),
      alignment: Alignment.center,
      child: Row(
        children: [
          controlWidget(Control('box-archive', _archive)),
          controlWidget(
            Control(
                'circle-exclamation', () => _flagMessage(MessageFlag.Flagged)),
          ),
          controlWidget(
            Control('trash-can', () => _flagMessage(MessageFlag.Deleted)),
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
