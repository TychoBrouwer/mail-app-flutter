import 'package:flutter/material.dart';
import 'package:mail_app/types/project_colors.dart';
import '../mail-client/enough_mail.dart';

import 'package:intl/intl.dart';

class MailPreview extends StatefulWidget {
  final MimeMessage email;
  final int idx;
  final Function getActive;
  final Function updateMessageID;

  const MailPreview({
    super.key,
    required this.email,
    required this.idx,
    required this.getActive,
    required this.updateMessageID,
  });

  @override
  MailPreviewState createState() => MailPreviewState();
}

class MailPreviewState extends State<MailPreview> {
  late MimeMessage _email;
  late int _idx;
  late Function _getActive;
  late Function _updateMessageID;

  late String _from;
  late String _previewStr;
  late String _dateText;

  @override
  void initState() {
    super.initState();

    _email = widget.email;
    _idx = widget.idx;
    _getActive = widget.getActive;
    _updateMessageID = widget.updateMessageID;

    DateTime? date = _email.decodeDate();

    if (date == null) {
      _dateText = '';
    } else {
      _dateText = DateTime.now().difference(date).inDays == 0
          ? DateFormat('HH:mm').format(date)
          : DateTime.now().difference(date).inDays == -1
              ? 'Yesterday'
              : DateFormat('dd/MM/yy').format(date);
    }

    _from = _email.from![0].personalName ?? _email.from![0].email;

    _previewStr = _textPreview();
  }

  String _textPreview() {
    try {
      if (_email.decodeTextHtmlPart() != null) {
        return _htmlPreview() ?? _plainTextPreview();
      } else {
        return _plainTextPreview();
      }
    } catch (e) {
      return _plainTextPreview();
    }
  }

  String? _htmlPreview() {
    final html = _email.decodeTextHtmlPart()!;
    final decoded = html
        .replaceAll(RegExp(r"\n|\r"), "")
        .replaceAll(RegExp(r"( +|&nbsp;)"), " ")
        .replaceAll(RegExp(r"&amp;"), "&");
    final previewRegex =
        RegExp(r'(?<=>)([^\/<>}\n]{5,})(?=<)').firstMatch(decoded);

    if (previewRegex == null) {
      return null;
    } else {
      return previewRegex[0]!.trim();
    }
  }

  String _plainTextPreview() {
    if (_email.decodeTextPlainPart() != null) {
      return _email.decodeTextPlainPart()!.split(RegExp(r"\n"))[0];
    } else {
      return '';
    }
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () => _updateMessageID(_idx),
      child: Container(
          decoration: BoxDecoration(
            borderRadius: const BorderRadius.all(
              Radius.circular(10),
            ),
            color: _getActive(_idx) ? ProjectColors.accent : Colors.transparent,
          ),
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 30),
            decoration: BoxDecoration(
              border: Border(
                bottom: BorderSide(
                  color: !_getActive(_idx)
                      ? ProjectColors.secondary(_getActive(_idx))
                      : Colors.transparent,
                ),
              ),
            ),
            child: Padding(
              padding: const EdgeInsets.only(bottom: 10, top: 8),
              child: Column(
                children: [
                  Align(
                    alignment: Alignment.centerLeft,
                    child: Row(
                      children: [
                        Expanded(
                          child: Padding(
                            padding: const EdgeInsets.only(right: 10),
                            child: Text(
                              _from,
                              overflow: TextOverflow.fade,
                              softWrap: false,
                              style: TextStyle(
                                fontSize: 14,
                                fontWeight: FontWeight.bold,
                                color: ProjectColors.main(_getActive(_idx)),
                              ),
                            ),
                          ),
                        ),
                        Text(
                          _dateText,
                          style: TextStyle(
                            color: ProjectColors.secondary(_getActive(_idx)),
                            fontSize: 12,
                          ),
                        ),
                      ],
                    ),
                  ),
                  Align(
                    alignment: Alignment.centerLeft,
                    child: Text(
                      _email.decodeSubject() ?? '',
                      overflow: TextOverflow.fade,
                      softWrap: false,
                      style: TextStyle(
                        fontSize: 13,
                        color: ProjectColors.main(_getActive(_idx)),
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                  ),
                  Align(
                    alignment: Alignment.centerLeft,
                    child: Text(
                      _previewStr,
                      overflow: TextOverflow.fade,
                      softWrap: false,
                      style: TextStyle(
                        fontSize: 13,
                        color: ProjectColors.secondary(_getActive(_idx)),
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          )),
    );
  }
}
