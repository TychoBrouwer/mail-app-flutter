import 'dart:math';

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
      _loadHtml();
    } else {
      _emailWidget = _loadPlainText();
    }
  }

  Future<void> _loadHtml() async {
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
      margin-left: auto !important;
      margin-right: auto !important; 
    ''';

    if (document.body!.attributes['style'] == null) {
      document.body!.attributes['style'] = defaultStyle;
    } else {
      document.body!.attributes['style'] =
          '${document.body!.attributes['style']} $defaultStyle';
    }
    document.body!.attributes['bgcolor'] = '';

    final styledHtml = _styleHtml(document.outerHtml);
    await _controller.loadStringContent(styledHtml);

    await for (LoadingState state in _controller.loadingState) {
      if (state == LoadingState.navigationCompleted) {
        break;
      }
    }

    await _updateHtmlSize();

    // _controller.openDevTools();

    _emailWidget = SizedBox(
      height: _webviewHeight,
      child: Webview(
        _controller,
      ),
    );

    await Future.delayed(const Duration(milliseconds: 100));

    setState(() {
      _showHtml = true;
    });
  }

  _styleHtml(String input) {
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

  _updateHtmlSize() async {
    final int height =
        await _controller.executeScript('document.body.offsetHeight;') ?? 0;
    final int width =
        await _controller.executeScript('document.body.offsetWidth;') ?? 0;

    final double widgetWidth = _widgetKey.currentContext?.size?.width ?? 0;

    final String newMargin =
        '${max((widgetWidth - width) / 2, 0).toString()}px';
    await _controller
        .executeScript('document.body.style.marginLeft = "$newMargin";');

    setState(() {
      _webviewHeight = height.toDouble() + 80;
    });

    return;
  }

  Widget _loadPlainText() {
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
                      child:
                          NotificationListener<SizeChangedLayoutNotification>(
                        onNotification: (notification) {
                          if (_showHtml) {
                            _updateHtmlSize();
                          }

                          return true;
                        },
                        child: SizeChangedLayoutNotifier(
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
                    ),
                  ),
                );
              },
            ),
          );
  }
}
