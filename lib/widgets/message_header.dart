import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:mail_app/types/project_colors.dart';

class MessageHeader extends StatefulWidget {
  final String from;
  final String to;
  final String subject;
  final DateTime? date;

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

    DateTime? date = widget.date;

    if (date == null) {
      _dateText = '';
    } else {
      String time = DateFormat('HH:mm').format(date);
      _dateText = DateTime.now().difference(date).inDays == 0
          ? 'Today at $time'
          : DateTime.now().difference(date).inDays == -1
              ? 'Yesterday at $time'
              : '${DateFormat('dd/MM/yy').format(date)} at $time';
    }
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 80,
          height: 80,
          margin: const EdgeInsets.only(right: 15),
          decoration: const BoxDecoration(color: Colors.black),
        ),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            children: [
              Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.max,
                children: [
                  Expanded(
                    child: Container(
                      margin: const EdgeInsets.only(right: 15),
                      child: Text(
                        _from,
                        overflow: TextOverflow.fade,
                        softWrap: false,
                        style: TextStyle(
                          fontSize: 14,
                          fontWeight: FontWeight.bold,
                          color: ProjectColors.main(false),
                        ),
                      ),
                    ),
                  ),
                  Text(
                    _dateText,
                    style: TextStyle(
                      fontSize: 13,
                      color: ProjectColors.main(false),
                    ),
                  ),
                ],
              ),
              RichText(
                overflow: TextOverflow.fade,
                softWrap: false,
                text: TextSpan(
                  style: TextStyle(
                    fontSize: 14,
                    color: ProjectColors.main(false),
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
                    fontSize: 14,
                    color: ProjectColors.main(false),
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
      ],
    );
  }
}
