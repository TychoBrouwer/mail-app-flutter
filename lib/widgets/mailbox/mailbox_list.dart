import 'package:flutter/material.dart';

import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/project_colors.dart';

class MailboxList extends StatefulWidget {
  final Map<int, List<MailboxInfo>> mailboxTree;
  final Function updateMessageList;
  final String activeMailbox;
  final int activeSession;
  final Widget header;

  const MailboxList({
    super.key,
    required this.mailboxTree,
    required this.updateMessageList,
    required this.activeMailbox,
    required this.activeSession,
    required this.header,
  });

  @override
  MailboxListState createState() => MailboxListState();
}

class MailboxListState extends State<MailboxList> {
  late Map<int, List<MailboxInfo>> _mailboxTree;
  late Function _updateMessageList;
  late String _activeMailbox;
  late int _activeSession;
  late Widget _header;

  @override
  void initState() {
    super.initState();

    _mailboxTree = widget.mailboxTree;
    _updateMessageList = widget.updateMessageList;
    _activeMailbox = widget.activeMailbox;
    _activeSession = widget.activeSession;
    _header = widget.header;
  }

  List<Widget> mailboxTreeBuilder() {
    List<Widget> mailboxTreeWidgets = [];

    _mailboxTree.forEach((int session, List<MailboxInfo> account) {
      for (MailboxInfo inboxInfo in account) {
        mailboxTreeWidgets.add(
          GestureDetector(
            onTap: () => {
              // int sessionId, String mailboxPath, String mailboxTitle
              _updateMessageList(session, inboxInfo.path, inboxInfo.display),
            },
            child: Container(
              padding: inboxInfo.indent
                  ? const EdgeInsets.only(
                      left: 30,
                      top: 5,
                      bottom: 5,
                    )
                  : const EdgeInsets.symmetric(horizontal: 8, vertical: 5),
              decoration: BoxDecoration(
                borderRadius: const BorderRadius.all(
                  Radius.circular(5),
                ),
                color: _activeSession == session &&
                        _activeMailbox == inboxInfo.path
                    ? ProjectColors.accent
                    : Colors.transparent,
              ),
              child: Text(
                inboxInfo.display,
                style: TextStyle(
                  color: _activeSession == session &&
                          _activeMailbox == inboxInfo.path
                      ? ProjectColors.main(true)
                      : ProjectColors.main(false),
                  fontSize: 14,
                ),
                overflow: TextOverflow.clip,
                softWrap: false,
              ),
            ),
          ),
        );
      }
    });

    return mailboxTreeWidgets;
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 10),
      child: ListView(
        children: [_header, ...mailboxTreeBuilder()],
      ),
    );
  }
}
