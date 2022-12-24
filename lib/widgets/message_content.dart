import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:mail_app/utils/wait_until.dart';
import '../mail-client/enough_mail.dart';
import 'package:mail_app/widgets/message_header.dart';
import 'package:mail_app/utils/to_rgba.dart';
import 'package:webview_windows/webview_windows.dart';
import 'package:html/parser.dart';
import 'package:mail_app/types/project_colors.dart';

class MessageContent extends StatefulWidget {
  final MimeMessage message;
  final WebviewController controller;

  const MessageContent({
    super.key,
    required this.message,
    required this.controller,
  });

  @override
  MessageContentState createState() => MessageContentState();
}

class MessageContentState extends State<MessageContent> {
  late MimeMessage _message;
  late String _from;
  late String _to;
  late String _subject;
  late WebviewController _controller;
  // final _controller = WebviewController();

  double _webviewHeight = 100;
  bool _showHtml = false;
  Widget _emailWidget = const SizedBox();

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _controller = widget.controller;

    if (_message.from![0].personalName == null) {
      _from = _message.from![0].email;
    } else {
      _from = '${_message.from![0].personalName!} <${_message.from![0].email}>';
    }

    _to = _message.to![0].email;
    _subject = _message.decodeSubject() ?? '';

    if (_message.decodeTextHtmlPart() != null) {
      loadHtml();
    } else {
      _emailWidget = loadPlainText();
    }
  }

  Future<void> loadHtml() async {
    setState(() {
      _showHtml = false;
    });
    final document = parse(_message.decodeTextHtmlPart());

    final defaultStyle = '''
      color: ${ProjectColors.main(true).toRgba()};
      font-family: Arial, Helvetica, sans-serif;
      font-size: 13;
      height: min-content;
      background-color: transparent;
    ''';

    if (document.body!.attributes['style'] == null) {
      document.body!.attributes['style'] = defaultStyle;
    } else {
      document.body!.attributes['style'] =
          '${document.body!.attributes['style']} $defaultStyle';
    }
    document.body!.attributes['bgcolor'] = '';
    document.body?.nodes.add(parseFragment(
        '<style>body::-webkit-scrollbar { width: 0;height: 0;}</style>'));
    // document.body!.attributes['onload'] =
    //     '(function() {document.body.setAttribute("height", document.body.offsetHeight);}).call(this)';
    // print(viewKey.currentContext?.size?.width);

    final styledHtml = styleHtml(document.outerHtml);
    await _controller.loadStringContent(styledHtml);
    await _controller.stop();

    // fixHeight();

    await Future.delayed(const Duration(milliseconds: 100));
    await fixHeight();
    _emailWidget = SizedBox(
      height: _webviewHeight,
      child: Webview(
        _controller,
      ),
    );

    await Future.delayed(const Duration(milliseconds: 40));
    setState(() {
      _showHtml = true;
    });
  }

  Future<void> fixHeight() async {
    int height = await _controller.executeScript('document.body.offsetHeight;');
    setState(() {
      _webviewHeight = height.toDouble() + 80;
    });
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
      padding: const EdgeInsets.only(bottom: 20, left: 6, right: 6),
      child: LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
          return SizedBox(
            height: constraints.maxHeight,
            child: SingleChildScrollView(
              scrollDirection: Axis.vertical,
              physics: const PageScrollPhysics(),
              // dragStartBehavior: DragStartBehavior.down,
              child: Padding(
                padding: const EdgeInsets.only(left: 16, right: 16),
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
                    Opacity(opacity: _showHtml ? 1.0 : 0, child: _emailWidget),
                    // FutureBuilder(
                    //   future: waitUntil(isHtmlVisible),
                    //   builder: (BuildContext context, snapshot) {
                    //     if (snapshot.connectionState == ConnectionState.done) {
                    //       return _emailWidget;
                    //     } else {
                    //       return const SizedBox();
                    //     }
                    //   },
                    // ),
                    // _emailWidget,
                  ],
                ),
              ),
            ),
          );
        },
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
