import 'package:flutter/material.dart';
import 'package:mail_app/widgets/message_content.dart';

import '../mail-client/enough_mail.dart';

class MessageView extends StatefulWidget {
  final MimeMessage message;
  final Widget controlBar;

  const MessageView({
    super.key,
    required this.message,
    required this.controlBar,
  });

  @override
  _MessageView createState() => _MessageView();
}

class _MessageView extends State<MessageView> {
  late MimeMessage _message;
  late Widget _controlBar;

  late bool plainText;

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _controlBar = widget.controlBar;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        _controlBar,
        MessageContent(message: _message),
      ],
    );
  }
}
