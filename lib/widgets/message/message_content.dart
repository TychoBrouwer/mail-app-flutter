import 'package:flutter/material.dart';
import 'package:html/parser.dart';
import 'package:webview_windows/webview_windows.dart';

import 'package:mail_app/utils/to_rgba.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/mail-client/enough_mail.dart';
import 'package:mail_app/widgets/message/message_header.dart';

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

  final GlobalKey _widgetKey = GlobalKey();

  double _webviewHeight = 100;
  bool _showHtml = false;
  Widget _emailWidget = const SizedBox(
    height: 10,
  );
  bool _displayEmpty = false;

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _controller = widget.controller;

    if (_message.from!.isEmpty) {
      _displayEmpty = true;

      return;
    }

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
      color: ${ProjectColors.main(true).toRgba()} !important;
      height: min-content !important;
      position: absolute !important;
      background-color: transparent !important;
      border: none !important;
    ''';

    if (document.body!.attributes['style'] == null) {
      document.body!.attributes['style'] = defaultStyle;
    } else {
      document.body!.attributes['style'] =
          '${document.body!.attributes['style']} $defaultStyle';
    }
    document.body!.attributes['bgcolor'] = '';

    final styledHtml = styleHtml(document.outerHtml);
    await _controller.loadStringContent(styledHtml);

    await Future.delayed(const Duration(milliseconds: 100));

    final int height =
        await _controller.executeScript('document.body.offsetHeight;');
    _controller.executeScript('''
      const maxWidth = document.body.offsetWidth;
      const styles = 'body::-webkit-scrollbar{display:none}*{max-width:'+maxWidth+'px!important;min-width:0!important}';

      document.body.style.marginLeft = Math.max(Math.floor(${getWidgetWidth()} - maxWidth) / 2, 0) + 'px';

      const styling = document.createElement('style');
      styling.type = 'text/css';
      styling.appendChild(document.createTextNode(styles));

      document.head.appendChild(styling);''');

    setState(() {
      _webviewHeight = height.toDouble() + 80;
    });

    // _controller.openDevTools();

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

  double getWidgetWidth() {
    final Size? size = _widgetKey.currentContext?.size;

    return size?.width ?? 0;
  }

  Widget loadPlainText() {
    return Text(
      _message.decodeTextPlainPart() ?? '',
      style: const TextStyle(color: Colors.white60),
    );
  }

  @override
  Widget build(BuildContext context) {
    return (_displayEmpty)
        ? const SizedBox()
        : Padding(
            padding: const EdgeInsets.only(bottom: 20, left: 15, right: 6),
            child: LayoutBuilder(
              builder: (BuildContext context, BoxConstraints constraints) {
                return SizedBox(
                  height: constraints.maxHeight,
                  child: SingleChildScrollView(
                    scrollDirection: Axis.vertical,
                    child: Padding(
                      padding: const EdgeInsets.only(right: 14),
                      child: Column(
                        key: _widgetKey,
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
                          Opacity(
                              opacity: _showHtml ? 1.0 : 0,
                              child: _emailWidget),
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
