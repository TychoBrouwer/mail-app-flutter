import 'package:flutter/material.dart';

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

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _controlBar = widget.controlBar;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _controlBar,
        Expanded(
          child: Padding(
            padding:
                const EdgeInsets.only(bottom: 12, top: 8, left: 16, right: 16),
            child: Text(
              _message.decodeTextPlainPart() ?? '',
              style: const TextStyle(color: Colors.white60),
            ),
          ),
        ),
      ],
    );
  }
}
