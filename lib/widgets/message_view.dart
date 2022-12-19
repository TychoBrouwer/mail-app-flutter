// import 'package:html/parser.dart' as htmlparser;
// import 'package:html/dom.dart' as dom;

import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:mail_app/widgets/message_header.dart';

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

  late String _from;
  late String _to;
  late String _subject;
  late Widget _emailWidget;
  late bool plainText;

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _controlBar = widget.controlBar;

    if (_message.from![0].personalName == null) {
      _from = _message.from![0].email;
    } else {
      _from = '${_message.from![0].personalName!} <${_message.from![0].email}>';
    }

    _to = _message.to![0].email;
    _subject = _message.decodeSubject() ?? '';

    if (_message.decodeTextHtmlPart() != null) {
      _emailWidget = loadHtml();

      plainText = false;
    } else {
      _emailWidget = loadPlainText();

      plainText = true;
    }
  }

  loadHtml() {
    final htmlBody = _message.decodeTextHtmlPart()!;
    final body =
        RegExp(r'<body.*?>(.*?)<\/body>', multiLine: true, dotAll: true)
            .firstMatch(htmlBody);

    if (body != null) {
      print(body[1]);
      return Html(
        data: body[1],
        shrinkWrap: true,
      );
    } else {
      print(htmlBody);
      return Html(
        data: htmlBody,
        shrinkWrap: true,
      );
    }
  }

  Widget loadPlainText() {
    return Text(
      _message.decodeTextPlainPart() ?? '',
      style: const TextStyle(color: Colors.white60),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        _controlBar,
        Expanded(
          child: Padding(
            padding: const EdgeInsets.only(bottom: 20),
            child: SingleChildScrollView(
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Padding(
                      padding: const EdgeInsets.only(bottom: 15),
                      child: MessageHeader(
                        from: _from,
                        to: _to,
                        subject: _subject,
                        date: _message.decodeDate(),
                      ),
                    ),
                    Flexible(child: _emailWidget),
                  ],
                ),
                // Text(
                //   _message.decodeTextPlainPart() ?? '',
                //   style: const TextStyle(color: Colors.white60),
                // ),
                //       ],
                //     ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
