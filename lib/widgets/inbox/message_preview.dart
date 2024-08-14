import 'package:flutter/material.dart';
import 'package:intl/intl.dart' show DateFormat;

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/message_flag.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/widgets/custom_button.dart';

class MessagePreview extends StatefulWidget {
  final Message message;
  final int idx;
  final bool Function(int) getActive;
  final void Function(int) updateMessageID;

  const MessagePreview({
    super.key,
    required this.message,
    required this.idx,
    required this.getActive,
    required this.updateMessageID,
  });

  @override
  MessagePreviewState createState() => MessagePreviewState();
}

class MessagePreviewState extends State<MessagePreview> {
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
    var decoded = _message.decodedText();

    return decoded.replaceAll(RegExp(r"\n"), " ");
  }

  @override
  Widget build(BuildContext context) {
    return CustomButton(
      onTap: () => _updateMessageID(_idx),
      borderRadius: ProjectSizes.borderRadius,
      child: Container(
        decoration: BoxDecoration(
          borderRadius: ProjectSizes.borderRadius,
          color: _getActive(_idx)
              ? ProjectColors.accent(true)
              : Colors.transparent,
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
                  color: _message.flags.contains(MessageFlag.Seen)
                      ? Colors.transparent
                      : !_getActive(_idx)
                          ? ProjectColors.accent(true)
                          : ProjectColors.background(true),
                ),
              ),
              Expanded(
                child: Container(
                  decoration: BoxDecoration(
                    border: Border(
                      bottom: BorderSide(
                        color: !_getActive(_idx)
                            ? ProjectColors.border(false)
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
                                      color: !_getActive(_idx)
                                          ? ProjectColors.text(true)
                                          : ProjectColors.background(true),
                                    ),
                                  ),
                                ),
                              ),
                              Text(
                                _dateText,
                                style: TextStyle(
                                  color: !_getActive(_idx)
                                      ? ProjectColors.text(false)
                                      : ProjectColors.background(true),
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
                              color: !_getActive(_idx)
                                  ? ProjectColors.text(false)
                                  : ProjectColors.background(true),
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
                              color: !_getActive(_idx)
                                  ? ProjectColors.text(false)
                                  : ProjectColors.background(true),
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
        ),
      ),
    );
  }
}
