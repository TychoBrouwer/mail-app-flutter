import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';

class MessageHeader extends StatefulWidget {
  final String from;
  final String to;
  final String subject;
  final DateTime date;

  const MessageHeader({
    super.key,
    required this.from,
    required this.to,
    required this.subject,
    required this.date,
  });

  @override
  MessageHeaderState createState() => MessageHeaderState();
}

class MessageHeaderState extends State<MessageHeader> {
  late String _from;
  late String _to;
  late String _subject;
  late String _dateText;

  @override
  void initState() {
    super.initState();

    _from = widget.from;
    _to = widget.to;
    _subject = widget.subject;

    DateTime date = widget.date;

    _dateText = DateTime.now().difference(date).inDays == 0
        ? DateFormat('HH:mm').format(date)
        : DateTime.now().difference(date).inDays == -1
            ? 'Yesterday'
            : DateFormat('dd/MM/yy').format(date);
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 70,
          height: 70,
          margin: const EdgeInsets.only(right: 15),
          decoration: BoxDecoration(
            color: ProjectColors.main(true),
            borderRadius: ProjectSizes.borderRadiusLarge,
          ),
        ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.only(bottom: 7),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Row(
                  children: [
                    Expanded(
                      child: Text(
                        _from,
                        overflow: TextOverflow.fade,
                        softWrap: false,
                        style: TextStyle(
                          fontSize: ProjectSizes.fontSize,
                          fontWeight: FontWeight.bold,
                          color: ProjectColors.main(false),
                        ),
                      ),
                    ),
                    Text(
                      _dateText,
                      style: TextStyle(
                        fontSize: ProjectSizes.fontSize,
                        color: ProjectColors.secondary(false),
                      ),
                    ),
                  ],
                ),
                RichText(
                  overflow: TextOverflow.fade,
                  softWrap: false,
                  text: TextSpan(
                    style: TextStyle(
                      fontSize: ProjectSizes.fontSize,
                      color: ProjectColors.secondary(false),
                    ),
                    children: [
                      const TextSpan(text: 'To: '),
                      TextSpan(
                        text: _to,
                        style: TextStyle(
                          color: ProjectColors.secondary(false),
                        ),
                      ),
                    ],
                  ),
                ),
                RichText(
                  overflow: TextOverflow.fade,
                  softWrap: false,
                  text: TextSpan(
                    style: TextStyle(
                      fontSize: ProjectSizes.fontSize,
                      color: ProjectColors.secondary(false),
                    ),
                    children: [
                      const TextSpan(text: 'Subject: '),
                      TextSpan(
                        text: _subject,
                        style: TextStyle(
                          color: ProjectColors.secondary(false),
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
