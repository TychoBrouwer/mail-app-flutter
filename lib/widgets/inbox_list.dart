import 'package:flutter/material.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/project_colors.dart';

class InboxList extends StatefulWidget {
  final Map<String, List<MailboxInfo>> accountsTree;
  final Function updateMessageList;
  final Map<String, String> activeMailbox;
  final Function updateActiveMailbox;
  final Widget header;

  const InboxList({
    super.key,
    required this.accountsTree,
    required this.updateMessageList,
    required this.activeMailbox,
    required this.updateActiveMailbox,
    required this.header,
  });

  @override
  _InboxList createState() => _InboxList();
}

class _InboxList extends State<InboxList> {
  late Map<String, List<MailboxInfo>> _accountsTree;
  late Function _updateMessageList;
  late Map<String, String> _activeMailbox;
  late Function _updateActiveMailbox;
  late Widget _header;

  @override
  void initState() {
    super.initState();

    _accountsTree = widget.accountsTree;
    _updateMessageList = widget.updateMessageList;
    _activeMailbox = widget.activeMailbox;
    _updateActiveMailbox = widget.updateActiveMailbox;
    _header = widget.header;
  }

  List<Widget> accountsTreeBuilder() {
    List<Widget> accountsTreeWidgets = [];

    _accountsTree.forEach((email, account) {
      for (var inboxInfo in account) {
        accountsTreeWidgets.add(
          GestureDetector(
            onTap: () => {
              _updateActiveMailbox(email, inboxInfo.path),
              _updateMessageList(email, inboxInfo.path, inboxInfo.display),
            },
            child: Container(
              margin: const EdgeInsets.symmetric(horizontal: 8),
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
                    ? ProjectColors.secondary(false)
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

    return accountsTreeWidgets;
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 10, top: 10, left: 10, right: 10),
      child: ListView(
        children: [_header, ...accountsTreeBuilder()],
      ),
    );
  }
}
