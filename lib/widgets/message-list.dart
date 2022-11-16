import 'package:flutter/material.dart';
import 'package:enough_mail/enough_mail.dart';

import '../widgets/message-preview.dart';

class MessageList extends StatefulWidget {
  final List<MimeMessage> messages;
  final int activeID;
  final Function updateActiveID;

  const MessageList({
    super.key,
    required this.updateActiveID,
    required this.messages,
    required this.activeID,
  });

  @override
  _MessageList createState() => _MessageList();
}

class _MessageList extends State<MessageList> {
  late List<MimeMessage> _messages;
  late int _activeID;
  late Function _updateActiveID;

  @override
  void initState() {
    super.initState();

    _messages = widget.messages;
    _activeID = widget.activeID;
    _updateActiveID = widget.updateActiveID;
  }

  bool _getActive(int idx) => _activeID == idx;

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemBuilder: (_, idx) {
        return MailPreview(
            email: _messages[idx],
            idx: idx,
            getActive: _getActive,
            updateMessageID: _updateActiveID);
      },
      itemCount: _messages.length,
    );
  }
}
