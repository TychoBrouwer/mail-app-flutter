import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';

class MailPreview extends StatefulWidget {
  final Message message;
  final int idx;
  final bool Function(int) getActive;
  final void Function(int) updateMessageID;

  const MailPreview({
    super.key,
    required this.message,
    required this.idx,
    required this.getActive,
    required this.updateMessageID,
  });

  @override
  MailPreviewState createState() => MailPreviewState();
}

class MailPreviewState extends State<MailPreview> {
  late Message _message;
  late int _idx;
  late bool Function(int) _getActive;
  late Function _updateMessageID;

  late String _from;
  late String _dateText;

  @override
  void initState() {
    super.initState();

    _message = widget.message;
    _idx = widget.idx;
    _getActive = widget.getActive;
    _updateMessageID = widget.updateMessageID;

    DateTime? date = DateTime.fromMillisecondsSinceEpoch(_message.received);

    _dateText = DateTime.now().difference(date).inDays == 0
        ? DateFormat('HH:mm').format(date)
        : DateTime.now().difference(date).inDays == -1
            ? 'Yesterday'
            : DateFormat('dd/MM/yy').format(date);

    _from = '${_message.from.first.mailbox}@${_message.from.first.host}';
  }

  String _textPreview() {
    var decoded = utf8.decode(base64Decode(_message.text));
    decoded = decoded.replaceAll(RegExp(r"\n"), " ");

    return decoded.split(RegExp(r"\n"))[0];
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () => _updateMessageID(_idx),
      child: Container(
          decoration: BoxDecoration(
            borderRadius: ProjectSizes.borderRadius,
            color: _getActive(_idx) ? ProjectColors.accent : Colors.transparent,
          ),
          child: Container(
            margin: const EdgeInsets.only(left: 10, right: 30),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Container(
                  margin: const EdgeInsets.only(top: 13, right: 10),
                  width: 10,
                  height: 10,
                  decoration: BoxDecoration(
                    borderRadius: ProjectSizes.borderRadiusSmall,
                    color: _message.flags.contains('Seen')
                        ? Colors.transparent
                        : !_getActive(_idx)
                            ? ProjectColors.accent
                            : ProjectColors.main(true),
                  ),
                ),
                Expanded(
                  child: Container(
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
                                        fontSize: ProjectSizes.fontSize,
                                        fontWeight: FontWeight.bold,
                                        color: ProjectColors.main(
                                            _getActive(_idx)),
                                      ),
                                    ),
                                  ),
                                ),
                                Text(
                                  _dateText,
                                  style: TextStyle(
                                    color: ProjectColors.secondary(
                                        _getActive(_idx)),
                                    fontSize: ProjectSizes.fontSize,
                                  ),
                                ),
                              ],
                            ),
                          ),
                          Align(
                            alignment: Alignment.centerLeft,
                            child: Text(
                              _message.subject,
                              overflow: TextOverflow.fade,
                              softWrap: false,
                              style: TextStyle(
                                fontSize: ProjectSizes.fontSize,
                                color: ProjectColors.main(_getActive(_idx)),
                                fontWeight: FontWeight.w500,
                              ),
                            ),
                          ),
                          Align(
                            alignment: Alignment.centerLeft,
                            child: Text(
                              _textPreview(),
                              overflow: TextOverflow.fade,
                              softWrap: false,
                              style: TextStyle(
                                fontSize: ProjectSizes.fontSize,
                                color:
                                    ProjectColors.secondary(_getActive(_idx)),
                                fontWeight: FontWeight.w500,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ],
            ),
          )),
    );
  }
}
