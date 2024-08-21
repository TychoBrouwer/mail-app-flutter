import 'package:flutter/material.dart';

import '../../types/message_flag.dart';
import '../../types/project_colors.dart';
import '../../types/mail_account.dart';
import '../custom_icon_button.dart';

class MessageControlBar extends StatefulWidget {
  final void Function(MessageFlag) flagMessage;
  final void Function(SpecialMailboxType) moveMessage;
  final void Function() reply;
  final void Function() replyAll;
  final void Function() share;
  final void Function() openSettings;

  const MessageControlBar({
    super.key,
    required this.flagMessage,
    required this.moveMessage,
    required this.reply,
    required this.replyAll,
    required this.share,
    required this.openSettings,
  });

  @override
  MessageControlBarState createState() => MessageControlBarState();
}

class MessageControlBarState extends State<MessageControlBar> {
  late void Function(MessageFlag) _flagMessage;
  late void Function(SpecialMailboxType) _moveMessage;
  late void Function() _reply;
  late void Function() _replyAll;
  late void Function() _share;
  late void Function() _openSettings;

  @override
  void initState() {
    super.initState();

    _flagMessage = widget.flagMessage;
    _moveMessage = widget.moveMessage;
    _reply = widget.reply;
    _replyAll = widget.replyAll;
    _share = widget.share;
    _openSettings = widget.openSettings;
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
          CustomIconButton(
              onTap: () => _moveMessage(SpecialMailboxType.archive),
              icon: "box-archive"),
          CustomIconButton(
              onTap: () => _flagMessage(MessageFlag.Flagged),
              icon: "circle-exclamation"),
          CustomIconButton(
              onTap: () => _moveMessage(SpecialMailboxType.trash),
              icon: "trash-can"),
          CustomIconButton(
              onTap: () => _flagMessage(MessageFlag.Seen),
              icon: "envelope-dot"),
          CustomIconButton(onTap: _reply, icon: "reply"),
          CustomIconButton(onTap: _replyAll, icon: "reply-all"),
          CustomIconButton(onTap: _share, icon: "share"),
          const Spacer(),
          CustomIconButton(onTap: _openSettings, icon: "settings"),
        ],
      ),
    );
  }
}
