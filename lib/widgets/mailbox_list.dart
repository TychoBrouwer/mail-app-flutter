import 'package:flutter/material.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/project_colors.dart';

class InboxList extends StatefulWidget {
  final Map<String, List<MailboxInfo>> mailboxTree;
  final Function updateMessageList;
  final Map<String, String> activeMailbox;
  final Function updateActiveMailbox;
  final Widget header;

  const InboxList({
    super.key,
    required this.mailboxTree,
    required this.updateMessageList,
    required this.activeMailbox,
    required this.updateActiveMailbox,
    required this.header,
  });

  @override
  InboxListState createState() => InboxListState();
}

class InboxListState extends State<InboxList> {
  late Map<String, List<MailboxInfo>> _mailboxTree;
  late Function _updateMessageList;
  late Map<String, String> _activeMailbox;
  late Function _updateActiveMailbox;
  late Widget _header;

  @override
  void initState() {
    super.initState();

    _mailboxTree = widget.mailboxTree;
    _updateMessageList = widget.updateMessageList;
    _activeMailbox = widget.activeMailbox;
    _updateActiveMailbox = widget.updateActiveMailbox;
    _header = widget.header;
  }

  List<Widget> mailboxTreeBuilder() {
    List<Widget> mailboxTreeWidgets = [];

    _mailboxTree.forEach((String email, List<MailboxInfo> account) {
      for (MailboxInfo inboxInfo in account) {
        // print({inboxInfo.display, inboxInfo.path, inboxInfo.indent});
        mailboxTreeWidgets.add(
          GestureDetector(
            onTap: () => {
              _updateActiveMailbox(email, inboxInfo.path),
              _updateMessageList(email, inboxInfo.path, inboxInfo.display),
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
                color: _activeMailbox['email'] == email &&
                        _activeMailbox['path'] == inboxInfo.path
                    ? ProjectColors.secondary(true)
                    : Colors.transparent,
              ),
              child: Text(
                inboxInfo.display,
                style: TextStyle(
                  color: ProjectColors.main(false),
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
