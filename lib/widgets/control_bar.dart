import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';

class ControlBar extends StatefulWidget {
  final void Function() markMessage;
  final void Function() readMessage;
  final void Function() reply;
  final void Function() replyAll;
  final void Function() share;

  const ControlBar({
    super.key,
    required this.markMessage,
    required this.readMessage,
    required this.reply,
    required this.replyAll,
    required this.share,
  });

  @override
  ControlBarState createState() => ControlBarState();
}

class ControlBarState extends State<ControlBar> {
  late void Function() _markMessage;
  late void Function() _readMessage;
  late void Function() _reply;
  late void Function() _replyAll;
  late void Function() _share;

  @override
  void initState() {
    super.initState();

    _markMessage = widget.markMessage;
    _readMessage = widget.readMessage;
    _reply = widget.reply;
    _replyAll = widget.replyAll;
    _share = widget.share;
  }

  List<Widget> buildControls() {
    final List<Control> controls = [
      Control('box-archive', _markMessage),
      Control('circle-exclamation', _markMessage),
      Control('trash-can', _markMessage),
      Control('envelope-dot', _readMessage),
      Control('reply', _reply),
      Control('reply-all', _replyAll),
      Control('share', _share),
    ];

    return controls
        .map(
          (control) => Padding(
            padding: const EdgeInsets.symmetric(horizontal: 5),
            child: CustomButton(
              onTap: () => control.function(),
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
  final void Function() function;

  Control(this.icon, this.function);
}
