import 'package:flutter/material.dart';
import 'package:mail_app/types/project_colors.dart';
import '../mail-client/enough_mail.dart';

import 'package:intl/intl.dart';

class MailPreview extends StatefulWidget {
  final MimeMessage email;
  final int idx;
  final DateTime? date;
  final Function getActive;
  final Function updateMessageID;

  const MailPreview({
    super.key,
    required this.email,
    required this.idx,
    required this.date,
    required this.getActive,
    required this.updateMessageID,
  });

  @override
  _MailPreview createState() => _MailPreview();
}

class _MailPreview extends State<MailPreview> {
  late MimeMessage _email;
  late int _idx;
  late DateTime? _date;
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
    _date = widget.date;
    _getActive = widget.getActive;
    _updateMessageID = widget.updateMessageID;

    if (_date == null) {
      _dateText = '';
    } else {
      _dateText = DateTime.now().difference(_date!).inDays == 0
          ? DateFormat('HH:mm').format(_date!)
          : DateTime.now().difference(_date!).inDays == -1
              ? 'Yesterday'
              : DateFormat('dd/MM/yy').format(_date!);
    }

    _from = _email.from![0].personalName ?? _email.from![0].email;

    if (_email.decodeTextHtmlPart() != null) {
      _previewStr = RegExp(r'(?<=>)([\w\s]{5,})(?=<\/)')
          .firstMatch(_email.decodeTextHtmlPart() ?? '')![0]!;
    } else {
      _previewStr = _email.decodeTextPlainPart()!.split(RegExp(r"\n"))[0];
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
            color: _getActive(_idx) ? Colors.blue : Colors.transparent,
          ),
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 30),
            decoration: BoxDecoration(
              border: !_getActive(_idx)
                  ? Border(
                      bottom: BorderSide(
                          color: ProjectColors.secondary(_getActive(_idx))),
                    )
                  : const Border(),
            ),
            child: Padding(
              padding: const EdgeInsets.only(bottom: 10, top: 6),
              child: Column(
                children: [
                  Align(
                    alignment: Alignment.centerLeft,
                    child: Row(
                      children: [
                        Expanded(
                          child: Text(
                            _from,
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              fontSize: 14,
                              fontWeight: FontWeight.bold,
                              color: ProjectColors.main(_getActive(_idx)),
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
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 13,
                        color: ProjectColors.secondary(_getActive(_idx)),
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                  ),
                  Align(
                    alignment: Alignment.centerLeft,
                    child: Text(
                      _previewStr,
                      overflow: TextOverflow.ellipsis,
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
