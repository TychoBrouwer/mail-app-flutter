import 'package:flutter/material.dart';
// import 'package:html/parser.dart' as html_parser;
import 'package:html/dom.dart' as html_dom;
import 'package:flutter_widget_from_html_core/flutter_widget_from_html_core.dart'
    as html_widget;

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/utils/hex_color.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/message/message_header.dart';

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
    // final document = html_parser.parse(decoded);

    // // var defaultStyle = '''
    // //   height: min-content !important;
    // //   position: absolute !important;
    // //   background-color: ${ProjectColors.main(true).toHex()} !important;
    // //   border: none !important;
    // //   margin-left: auto !important;
    // //   margin-right: auto !important;
    // //   width: 100% !important;
    // // ''';

    // // if (document.body!.attributes['style'] == null) {
    // //   document.body!.attributes['style'] = defaultStyle;
    // // } else {
    // //   document.body!.attributes['style'] =
    // //       '${document.body!.attributes['style']} $defaultStyle';
    // // }

    // // var styleTag = html_parser
    // //     .parseFragment('<style>* {background-color: transparent}</style>');

    // // document.body!.append(styleTag);

    // // final styledHtml = _styleHtml(document.body!.outerHtml);
    // String styledHtml = document.body!.outerHtml;

    // Remove text color styling
    final colorRegex = RegExp(r'(?<!-)color:.*?;', caseSensitive: false);
    html = html.replaceAllMapped(colorRegex, (Match match) => '');

    // final colorBgRegex =
    //     RegExp(r'background(-color)?:.*?;', caseSensitive: false);
    // html = html.replaceAllMapped(colorBgRegex, (Match match) => '');

    // Remove bgcolor styling
    final bgColorRegex = RegExp(r'bgcolor=".*?"', caseSensitive: false);
    html = html.replaceAllMapped(bgColorRegex, (Match match) => '');

    // Add href attribute to links without it only containing the link as text
    final hrefRegex =
        RegExp(r'<a((?!.*href).*?)>(http.*?)<\/a>', caseSensitive: false);
    html = html.replaceAllMapped(hrefRegex,
        (Match match) => '<a ${match[1]} href="${match[2]}">${match[2]}</a>');

    print(html);

    _emailWidget = html_widget.HtmlWidget(
      html,
      key: UniqueKey(),
      textStyle: TextStyle(
        color: ProjectColors.main(false),
        fontSize: ProjectSizes.fontSize,
      ),
      renderMode: const html_widget.ListViewMode(shrinkWrap: true),
      customStylesBuilder: (element) {
        final type = element.localName;
        final attributes = element.attributes;

        Map<String, String>? style = {};

        if (!checkParrentForHref(element)) {
          style['background'] =
              '${ProjectColors.main(true).toHex()} !important';
          style['background-color'] =
              '${ProjectColors.main(true).toHex()} !important';
        }

        if (type != 'a' && type != 'link') {
          style['color'] = '${ProjectColors.main(false).toHex()} !important';
        } else if ((type == 'a' || type == 'link') &&
            attributes.keys.contains('href')) {
          style['color'] = '${ProjectColors.accent(true).toHex()} !important';
        }

        return style;
      },
      onTapUrl: (url) => openUrl(url),
    );

    setState(() {
      _showHtml = true;
    });
  }

  bool checkParrentForHref(html_dom.Element element) {
    if (element.localName == 'a' || element.localName == 'link') {
      return true;
    }

    if (element.parent == null) {
      return false;
    }

    return checkParrentForHref(element.parent!);
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

  // _styleHtml(String input) {
  //   String output = input;

  //   final rgbRegex =
  //       RegExp(r'rgba?\((\d{1,3}), ?(\d{1,3}), ?(\d{1,3}),? ?(\d.?\d{1,2})?\)');

  //   output = output.replaceAllMapped(rgbRegex, (Match match) {
  //     final r = 255 - int.parse(match[1]!);
  //     final g = 255 - int.parse(match[2]!);
  //     final b = 255 - int.parse(match[3]!);
  //     final a = match.groupCount == 4 ? double.parse(match[4]!) : 1.0;

  //     final color = Color.fromRGBO(r, g, b, a * 255);

  //     return color.toRgba();
  //   });

  //   final hexRegex = RegExp(r'#([0-9a-f]{6})', caseSensitive: false);

  //   output = output.replaceAllMapped(hexRegex, (Match match) {
  //     final color = HexColor.fromHex(match[1]!);

  //     final newColor = Color.fromRGBO(
  //       255 - color.red,
  //       255 - color.green,
  //       255 - color.blue,
  //       1,
  //     );

  //     return newColor.toRgba();
  //   });

  //   final hexRegexShort = RegExp(r'#([0-9a-f]{3})', caseSensitive: false);

  //   output = output.replaceAllMapped(hexRegexShort, (Match match) {
  //     final fullHex =
  //         match[1]!.split('').map((String char) => char + char).join('');
  //     final color = HexColor.fromHex(fullHex);

  //     final newColor = Color.fromRGBO(
  //       255 - color.red,
  //       255 - color.green,
  //       255 - color.blue,
  //       1,
  //     );

  //     return newColor.toRgba();
  //   });

  //   // output =
  //   //     output.replaceAllMapped(RegExp(r'(?<!-)color:.*?;'), (Match match) {
  //   //   return 'color: ${ProjectColors.main(false).toRgba()} !important;';
  //   // });

  //   return output;
  // }

  @override
  Widget build(BuildContext context) {
    return (_message == null)
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
