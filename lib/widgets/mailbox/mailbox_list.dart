import 'package:flutter/material.dart';

import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/widgets/custom_button.dart';

class MailboxList extends StatefulWidget {
  final Map<int, List<MailboxInfo>> mailboxTree;
  final void Function(int, String) updateMessageList;
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
  late void Function(int, String) _updateMessageList;
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
          CustomButton(
            onTap: () => {
              _updateMessageList(session, inboxInfo.path),
            },
            child: Container(
              padding: inboxInfo.indent
                  ? const EdgeInsets.only(
                      left: 30,
                      right: 5,
                      top: 3,
                      bottom: 3,
                    )
                  : const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
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
                      ? ProjectColors.main(true)
                      : ProjectColors.main(false),
                  fontSize: ProjectSizes.fontSize,
                  fontWeight: FontWeight.normal,
                ),
                overflow: TextOverflow.clip,
                softWrap: false,
              ),
            ),
          ),
        );
        mailboxTreeWidgets.add(const SizedBox(
          height: 3,
          width: double.infinity,
        ));
      }
    });

    return mailboxTreeWidgets;
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 10),
      child: ListView(
        key: UniqueKey(),
        children: [_header, ...mailboxTreeBuilder()],
      ),
    );
  }
}
