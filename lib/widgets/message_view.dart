import 'package:flutter/material.dart';
import 'package:mail_app/widgets/message_content.dart';
import 'package:webview_windows/webview_windows.dart';

import '../mail-client/enough_mail.dart';

class MessageView extends StatefulWidget {
  final MimeMessage message;
  final Widget controlBar;
  final WebviewController controller;

  const MessageView({
    super.key,
    required this.message,
    required this.controlBar,
    required this.controller,
  });

  @override
  MessageViewState createState() => MessageViewState();
}

class MessageViewState extends State<MessageView> {
  late MimeMessage _message;
  late WebviewController _controller;
  late Widget _controlBar;

  late bool plainText;

  final messageContentKey = GlobalKey<MessageContentState>();

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _controlBar = widget.controlBar;
    _controller = widget.controller;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        _controlBar,
        Expanded(
          child: MessageContent(message: _message, controller: _controller),
        ),
      ],
    );
  }
}
