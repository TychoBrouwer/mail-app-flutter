import 'package:flutter/material.dart';
import 'package:flutter_html/flutter_html.dart';
import 'package:mail_app/widgets/message_header.dart';
import 'package:html/parser.dart' as htmlParser;
import 'package:html/dom.dart' as dom;

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
  // late String _htmlBody;
  // late dom.Document _htmlDoc;

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

    // if (_message.decodeTextHtmlPart() != null) {
    //   final body = RegExp(r'<body.*?</body>', multiLine: true, dotAll: true)
    //       .firstMatch(_message.decodeTextHtmlPart()!);
    //   if (body != null) {
    //     _htmlBody = body[0]!;
    //     dom.Document _htmlDoc = htmlParser.parse(body[0]!);

    //     print(body[0]!);
    //   } else {
    //     print('failed');
    //     _htmlBody = _message.decodeTextHtmlPart()!;
    //   }

    //   // _htmlBody = RegExp(r'<body.*?</body>')
    //   //     .firstMatch(_message.decodeTextHtmlPart()!)![0]!;
    //   // print(RegExp(r'<body.*?</body>')
    //   //     .firstMatch(_message.decodeTextHtmlPart()!));
    // } else {
    //   print('failed!');
    //   _htmlBody = _message.decodeTextPlainPart()!;
    // }
    // _htmlBody = _message.decodeTextPlainPart()!;
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
                  mainAxisSize: MainAxisSize.max,
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
                    Text(
                      _message.decodeTextPlainPart() ?? '',
                      style: const TextStyle(color: Colors.white60),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
