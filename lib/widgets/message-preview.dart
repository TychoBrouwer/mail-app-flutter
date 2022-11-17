import 'package:flutter/material.dart';
// import 'package:enough_mail/enough_mail.dart';
import '../mail-client/enough_mail.dart';

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
  _MailPreview createState() => _MailPreview();
}

class _MailPreview extends State<MailPreview> {
  late MimeMessage _email;
  late int _idx;
  late Function _getActive;
  late Function _updateMessageID;

  late String _from;
  late String _previewStr;

  @override
  void initState() {
    super.initState();

    _email = widget.email;
    _idx = widget.idx;
    _getActive = widget.getActive;
    _updateMessageID = widget.updateMessageID;

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
      child: DecoratedBox(
        decoration: BoxDecoration(
          border: const Border(
            bottom: BorderSide(color: Colors.black),
          ),
          color: _getActive(_idx) ? Colors.blue : Colors.transparent,
        ),
        child: Padding(
          padding:
              const EdgeInsets.only(bottom: 12, top: 8, left: 16, right: 16),
          child: Column(
            children: [
              Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  _from,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(fontSize: 18, color: Colors.white60),
                ),
              ),
              Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  'subject: ${_email.decodeSubject()}',
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(color: Colors.white60),
                ),
              ),
              Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  _previewStr,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(color: Colors.white60),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
