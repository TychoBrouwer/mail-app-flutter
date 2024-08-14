import 'package:flutter/material.dart';
import 'package:html/dom.dart' as html_dom;
import 'package:flutter_widget_from_html_core/flutter_widget_from_html_core.dart'
    as html_widget;
import 'package:csslib/visitor.dart' as css_parser;

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/utils/hex_color.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/message/message_header.dart';

class _HtmlWidgetFactory extends html_widget.WidgetFactory {
  @override
  void parseStyle(html_widget.BuildTree tree, css_parser.Declaration style) {
    if (!checkParrentForHref(tree.element) &&
        (style.property == 'background-color' ||
            style.property == 'background')) {
      return;
    }

    if (style.property == 'color') {
      return;
    }

    return super.parseStyle(tree, style);
  }
}

bool checkParrentForHref(html_dom.Element element) {
  if (element.attributes.keys.contains('href')) {
    return true;
  }

  if (element.parent == null) {
    return false;
  }

  return checkParrentForHref(element.parent!);
}

class MessageContent extends StatefulWidget {
  final Message? message;

  const MessageContent({
    super.key,
    required this.message,
  });

  @override
  MessageContentState createState() => MessageContentState();
}

class MessageContentState extends State<MessageContent> {
  late Message? _message;
  late String _from;
  late String _to;

  final GlobalKey _widgetKey = GlobalKey();

  bool _showHtml = false;
  Widget _emailWidget = const SizedBox(
    height: 10,
  );

  @override
  void initState() {
    super.initState();

    _message = widget.message;

    if (_message != null) {
      _from = '${_message!.from.first.mailbox}@${_message!.from.first.host}';
      _to = '${_message!.to.first.mailbox}@${_message!.to.first.host}';

      if (_message!.html.isNotEmpty) {
        _loadHtml();
      } else {
        _emailWidget = _loadText();
      }
    }
  }

  Future<void> _loadHtml() async {
    setState(() {
      _showHtml = false;
    });

    String html = _message!.decodedHtml();

    // Add href attribute to links without it only containing the link as text
    final hrefRegex =
        RegExp(r'<a((?!.*href).*?)>(http.*?)<\/a>', caseSensitive: false);
    html = html.replaceAllMapped(hrefRegex,
        (Match match) => '<a ${match[1]} href="${match[2]}">${match[2]}</a>');

    _emailWidget = html_widget.HtmlWidget(
      html,
      key: UniqueKey(),
      textStyle: TextStyle(
        color: ProjectColors.text(true),
        fontSize: ProjectSizes.fontSize,
      ),
      renderMode: const html_widget.ListViewMode(shrinkWrap: true),
      customStylesBuilder: (element) {
        final type = element.localName;
        final attributes = element.attributes;

        Map<String, String>? style = {};

        if (!checkParrentForHref(element)) {
          style['background'] = ProjectColors.background(true).toHex();
          style['background-color'] = ProjectColors.background(true).toHex();
        }

        if (type != 'a' && type != 'link') {
          style['color'] = ProjectColors.text(true).toHex();
        } else if ((type == 'a' || type == 'link') &&
            attributes.keys.contains('href')) {
          style['color'] = ProjectColors.accent(true).toHex();
        }

        if (type == 'title') {
          style['display'] = 'none';
        }

        return style;
      },
      factoryBuilder: () => _HtmlWidgetFactory(),
      onTapUrl: (url) => openUrl(url),
    );

    setState(() {
      _showHtml = true;
    });
  }

  bool openUrl(String url) {
    print('tapped $url');

    return true;
  }

  Widget _loadText() {
    final decoded = _message!.decodedText();

    return Text(
      decoded,
      style: const TextStyle(color: Colors.white60),
    );
  }

  @override
  Widget build(BuildContext context) {
    return (_message == null)
        ? const SizedBox()
        : Padding(
            padding: const EdgeInsets.only(bottom: 20, left: 10, right: 5),
            child: LayoutBuilder(
              builder: (BuildContext context, BoxConstraints constraints) {
                return SizedBox(
                  height: constraints.maxHeight,
                  child: SingleChildScrollView(
                    scrollDirection: Axis.vertical,
                    child: Padding(
                      padding: const EdgeInsets.only(right: 10),
                      child: Column(
                        key: _widgetKey,
                        children: [
                          Padding(
                            padding: const EdgeInsets.only(bottom: 15),
                            child: MessageHeader(
                              from: _from,
                              to: _to,
                              subject: _message!.subject,
                              date: DateTime.fromMillisecondsSinceEpoch(
                                  _message!.received),
                            ),
                          ),
                          Opacity(
                            opacity: _showHtml ? 1.0 : 0,
                            child: _emailWidget,
                          ),
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
