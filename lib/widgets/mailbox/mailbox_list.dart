import 'package:flutter/material.dart';

import '../../types/mailbox_info.dart';
import '../../types/project_colors.dart';
import '../../types/project_sizes.dart';
import '../custom_button.dart';

class MailboxList extends StatefulWidget {
  final Map<int, List<MailboxInfo>> mailboxTree;
  final void Function(int, String) updateMessageList;
  final String activeMailbox;
  final int activeSession;

  const MailboxList({
    super.key,
    required this.mailboxTree,
    required this.updateMessageList,
    required this.activeMailbox,
    required this.activeSession,
  });

  @override
  MailboxListState createState() => MailboxListState();
}

class MailboxListState extends State<MailboxList> {
  late Map<int, List<MailboxInfo>> _mailboxTree;
  late void Function(int, String) _updateMessageList;
  late String _activeMailbox;
  late int _activeSession;

  @override
  void initState() {
    super.initState();

    _mailboxTree = widget.mailboxTree;
    _updateMessageList = widget.updateMessageList;
    _activeMailbox = widget.activeMailbox;
    _activeSession = widget.activeSession;
  }

  List<Widget> mailboxTreeBuilder() {
    List<Widget> mailboxTreeWidgets = [];

    _mailboxTree.forEach((int session, List<MailboxInfo> account) {
      for (MailboxInfo inboxInfo in account) {
        mailboxTreeWidgets.add(
          Padding(
            padding: const EdgeInsets.only(bottom: 3),
            child: CustomButton(
              onTap: () {
                _updateMessageList(session, inboxInfo.path);
              },
              child: Container(
                padding: inboxInfo.indent
                    ? const EdgeInsets.only(
                        left: 30,
                        right: 20,
                        top: 3,
                        bottom: 5,
                      )
                    : const EdgeInsets.only(
                        left: 20,
                        right: 20,
                        top: 3,
                        bottom: 5,
                      ),
                decoration: BoxDecoration(
                  borderRadius: ProjectSizes.borderRadiusSmall,
                  color: _activeSession == session &&
                          _activeMailbox == inboxInfo.path
                      ? ProjectColors.accent(true)
                      : Colors.transparent,
                ),
                child: Text(
                  inboxInfo.display,
                  style: TextStyle(
                    color: _activeSession == session &&
                            _activeMailbox == inboxInfo.path
                        ? ProjectColors.background(true)
                        : ProjectColors.text(true),
                    fontSize: ProjectSizes.fontSize,
                    fontWeight: FontWeight.w500,
                  ),
                  overflow: TextOverflow.clip,
                  softWrap: false,
                ),
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
    return Container(
      padding: const EdgeInsets.only(left: 5, right: 5, top: 10),
      child: ListView(
        key: UniqueKey(),
        children: mailboxTreeBuilder(),
      ),
    );
  }
}
