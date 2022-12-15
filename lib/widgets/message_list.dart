import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mail_app/types/project_colors.dart';
import '../mail-client/enough_mail.dart';

import 'message_preview.dart';

class MessageList extends StatefulWidget {
  final List<MimeMessage> messages;
  final String mailboxTitle;
  final int activeID;
  final Function updateActiveID;
  final Function refreshAll;

  const MessageList({
    super.key,
    required this.updateActiveID,
    required this.mailboxTitle,
    required this.messages,
    required this.activeID,
    required this.refreshAll,
  });

  @override
  _MessageList createState() => _MessageList();
}

class _MessageList extends State<MessageList> {
  late List<MimeMessage> _messages;
  late String _mailboxTitle;
  late int _activeID;
  late Function _updateActiveID;
  late Function _refreshAll;

  @override
  void initState() {
    super.initState();

    _messages = widget.messages;
    _mailboxTitle = widget.mailboxTitle;
    _activeID = widget.activeID;
    _updateActiveID = widget.updateActiveID;
    _refreshAll = widget.refreshAll;
  }

  bool _getActive(int idx) => _activeID == idx;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.only(left: 15, right: 15, top: 20),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  RegExp(r'.*@.*\.').hasMatch(_mailboxTitle)
                      ? 'INBOX'
                      : _mailboxTitle.toUpperCase(),
                  textAlign: TextAlign.left,
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: ProjectColors.main(false),
                  ),
                ),
              ),
              GestureDetector(
                onTap: () => _refreshAll(),
                child: SvgPicture.asset(
                  'assets/icons/arrows-rotate.svg',
                  color: ProjectColors.main(false),
                  width: 20,
                  height: 20,
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.only(left: 6, right: 6, top: 35),
            child: ListView.builder(
              itemBuilder: (_, idx) {
                return MailPreview(
                    email: _messages[idx],
                    date: _messages[idx].decodeDate(),
                    idx: idx,
                    getActive: _getActive,
                    updateMessageID: _updateActiveID);
              },
              itemCount: _messages.length,
            ),
          ),
        )
      ],
    );
  }
}
