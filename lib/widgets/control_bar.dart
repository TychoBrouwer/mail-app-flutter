import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';

class ControlBar extends StatefulWidget {
  final Function archive;
  final Function markImportant;
  final Function markDeleted;
  final Function markUnread;
  final Function reply;
  final Function replyAll;
  final Function share;

  const ControlBar({
    super.key,
    required this.archive,
    required this.markImportant,
    required this.markDeleted,
    required this.markUnread,
    required this.reply,
    required this.replyAll,
    required this.share,
  });

  @override
  ControlBarState createState() => ControlBarState();
}

class ControlBarState extends State<ControlBar> {
  late Function _archive;
  late Function _markImportant;
  late Function _markDeleted;
  late Function _markUnread;
  late Function _reply;
  late Function _replyAll;
  late Function _share;

  @override
  void initState() {
    super.initState();

    _archive = widget.archive;
    _markImportant = widget.markImportant;
    _markDeleted = widget.markDeleted;
    _markUnread = widget.markUnread;
    _reply = widget.reply;
    _replyAll = widget.replyAll;
    _share = widget.share;
  }

  List<Widget> buildControls() {
    final List<Control> controls = [
      Control('box-archive', _archive),
      Control('circle-exclamation', _markImportant),
      Control('trash-can', _markDeleted),
      Control('envelope-dot', _markUnread),
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
                padding: const EdgeInsets.all(10),
                child: SvgPicture.asset(
                  'assets/icons/${control.icon}.svg',
                  color: ProjectColors.main(false),
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

  Control(this.icon, this.function);
}
