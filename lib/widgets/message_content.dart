import 'package:flutter/material.dart';
import '../mail-client/enough_mail.dart';
import 'package:mail_app/widgets/message_header.dart';
import 'package:mail_app/utils/to_rgba.dart';
import 'package:webview_windows/webview_windows.dart';

class MessageContent extends StatefulWidget {
  final MimeMessage message;

  const MessageContent({
    super.key,
    required this.message,
  });

  @override
  _MessageContent createState() => _MessageContent();
}

class _MessageContent extends State<MessageContent> {
  late MimeMessage _message;
  late String _from;
  late String _to;
  late String _subject;
  late Widget _emailWidget;
  final _controller = WebviewController();

  @override
  void initState() {
    super.initState();

    _message = widget.message;

    if (_message.from![0].personalName == null) {
      _from = _message.from![0].email;
    } else {
      _from = '${_message.from![0].personalName!} <${_message.from![0].email}>';
    }

    _to = _message.to![0].email;
    _subject = _message.decodeSubject() ?? '';

    if (_message.decodeTextHtmlPart() != null) {
      loadHtml();
      // _emailWidget = loadPlainText();
    } else {
      _emailWidget = loadPlainText();
    }
  }

  loadHtml() async {
    final html = _message.decodeTextHtmlPart()!;
    final styledHtml = styleHtml(html);

    await _controller.initialize();
    await _controller.loadStringContent(styledHtml);
    await _controller.setBackgroundColor(Colors.transparent);
    // await _controller.executeScript('''
    //   document.getElementsByTagName("*").forEach((element) => {
    //     current = element.style.color;
    //     console.log(current);
    //   })
    // ''');
    await _controller.setPopupWindowPolicy(WebviewPopupWindowPolicy.deny);
    // await _controller.openDevTools();

    // print(htmlBody);

    if (!mounted) return;
    setState(() {});
  }

  styleHtml(String input) {
    String output = input;
    output = output.replaceAllMapped(
        RegExp(
            r'rgba\(([0-9][0-9]?[0-9]?), ?([0-9][0-9]?[0-9]?), ?([0-9][0-9]?[0-9]?), ?([0-9]?.?[0-9]?[0-9]?)\)'),
        (Match match) {
      return 'rgba(${255 - int.parse(match[1]!)}, ${255 - int.parse(match[2]!)}, ${255 - int.parse(match[3]!)}, ${match[4]})';
    });

    output = output.replaceAllMapped(
        RegExp(
            r'rgb\(([0-9][0-9]?[0-9]?), ?([0-9][0-9]?[0-9]?), ?([0-9][0-9]?[0-9]?)\)'),
        (Match match) {
      return 'rgb(${255 - int.parse(match[1]!)}, ${255 - int.parse(match[2]!)}, ${255 - int.parse(match[3]!)})';
    });

    output = output.replaceAllMapped(
        RegExp(r'#([0-9A-F]{6})', caseSensitive: false), (Match match) {
      final color = HexColor.fromHex(match[1]!);
      final newColor = Color.fromRGBO(
          255 - color.red, 255 - color.green, 255 - color.blue, 1);

      return newColor.toHex();
    });

    return output;
  }

  Widget loadPlainText() {
    return Text(
      _message.decodeTextPlainPart() ?? '',
      style: const TextStyle(color: Colors.white60),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 20, left: 20, right: 20),
      child: SingleChildScrollView(
        scrollDirection: Axis.vertical,
        child: Column(
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
            SizedBox(
              height: 500,
              child: Webview(_controller),
            ),
            // _emailWidget,
          ],
        ),
      ),
    );
  }
}



            // Text(
            //   _message.decodeTextPlainPart() ?? '',
            //   style: const TextStyle(color: Colors.white60),
            // ),
            //       ],
            //     ),

