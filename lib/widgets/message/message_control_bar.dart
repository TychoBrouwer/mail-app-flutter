import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/types/message_update.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';

class ControlBar extends StatefulWidget {
  final Function markMessage;
  final Function reply;
  final Function replyAll;
  final Function share;

  const ControlBar({
    super.key,
    required this.markMessage,
    required this.reply,
    required this.replyAll,
    required this.share,
  });

  @override
  ControlBarState createState() => ControlBarState();
}

class ControlBarState extends State<ControlBar> {
  late Function _markMessage;
  late Function _reply;
  late Function _replyAll;
  late Function _share;

  @override
  void initState() {
    super.initState();

    _markMessage = widget.markMessage;
    _reply = widget.reply;
    _replyAll = widget.replyAll;
    _share = widget.share;
  }

  List<Widget> buildControls() {
    final List<Control> controls = [
      Control('box-archive', _markMessage, MessageUpdate.archive),
      Control('circle-exclamation', _markMessage, MessageUpdate.flag),
      Control('trash-can', _markMessage, MessageUpdate.delete),
      Control('envelope-dot', _markMessage, MessageUpdate.seen),
      Control('reply', _reply),
      Control('reply-all', _replyAll),
      Control('share', _share),
    ];

    return controls
        .map(
          (control) => Padding(
            padding: const EdgeInsets.symmetric(horizontal: 5),
            child: CustomButton(
              onTap: () => control.argument == MessageUpdate.none
                  ? control.function()
                  : control.function(control.argument),
              child: Padding(
                padding: const EdgeInsets.all(5),
                child: SvgPicture.asset(
                  'assets/icons/${control.icon}.svg',
                  colorFilter: ColorFilter.mode(
                      ProjectColors.main(false), BlendMode.srcIn),
                  alignment: Alignment.centerRight,
                  height: 20,
                  width: 20,
                ),
              ),
            ),
          ),
        )
        .toList();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 10, bottom: 15, left: 50),
      alignment: Alignment.center,
      child: Row(
        children: buildControls(),
      ),
    );
  }
}

class Control {
  final String icon;
  final Function function;
  final MessageUpdate argument;

  Control(this.icon, this.function, [this.argument = MessageUpdate.none]);
}
