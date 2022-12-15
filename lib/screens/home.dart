import 'package:flutter/material.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/control_bar.dart';
import 'package:mail_app/widgets/inbox_list.dart';
import 'package:mail_app/widgets/mailbox_header.dart';
import 'package:mail_app/widgets/message_view.dart';

import '../mail-client/enough_mail.dart';
import '../services/inbox_service.dart';
import '../widgets/vertical_split.dart';
import '../widgets/message_list.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final InboxService _inboxService = InboxService();
  final String email = 'test1928346534@gmail.com';
  final String password = 'xsccljyfbfrgvtjw';

  List<MimeMessage> _messages = [];
  Map<String, List<MailboxInfo>> _accountsTree = {};
  int _activeID = 0;
  String _mailboxTitle = '';
  Map<String, String> _activeMailbox = {
    'email': 'email',
    'path': 'path',
  };

  @override
  void initState() {
    super.initState();

    _inboxService.newClient(
        email, password, 'imap.gmail.com', 993, 'smtp.gmail.com', 993);
    _updateInbox();
  }

  _updateActiveID(int idx) {
    setState(() {
      _activeID = idx;
    });
  }

  _updateActiveMailbox(String email, String path) {
    setState(() {
      _activeMailbox = {
        'email': email,
        'path': path,
      };
    });
  }

  _setMessages() {
    final List<MimeMessage> messages = _inboxService.getMessages();

    messages.sort((a, b) => b
        .decodeDate()!
        .millisecondsSinceEpoch
        .compareTo(a.decodeDate()!.millisecondsSinceEpoch));

    setState(() {
      _messages = messages;
    });
  }

  _setAccountTree() {
    setState(() {
      _accountsTree = _inboxService.accountsTree();
    });
  }

  _updateInbox() async {
    await _inboxService.clientsConnected();
    _inboxService.updateInbox();
    _setAccountTree();
  }

  _updateMessageList(
      String email, String mailboxPath, String mailboxTitle) async {
    _activeID = 0;
    _mailboxTitle = mailboxTitle;
    _inboxService.updateLocalMailbox(email, mailboxPath);

    _setMessages();

    await _inboxService.updateMailList(email, mailboxPath);

    _setMessages();
  }

  _refreshAll() {
    print('refreshing');
  }

  _composeMessage() {
    print('composing a message');
  }

  _archive() {
    print('archive a message');
  }

  _markImportant() {
    print('mark as important');
  }

  _markDeleted() {
    print('mark as deleted');
  }

  _markUnread() {
    print('mark as unread');
  }

  _reply() {
    print('reply to message');
  }

  _replyAll() {
    print('reply to all message');
  }

  _share() {
    print('share message');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          color: Colors.black87,
        ),
        child: Center(
          child: VerticalSplitView(
            left: Container(
              decoration: BoxDecoration(
                border: Border(
                    right: BorderSide(color: ProjectColors.secondary(false))),
              ),
              height: double.infinity,
              child: InboxList(
                  accountsTree: _accountsTree,
                  updateMessageList: _updateMessageList,
                  activeMailbox: _activeMailbox,
                  updateActiveMailbox: _updateActiveMailbox,
                  header: MailboxHeader(
                    composeMessage: _composeMessage,
                  ),
                  key: UniqueKey()),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: MessageList(
                  messages: _messages,
                  mailboxTitle: _mailboxTitle,
                  activeID: _activeID,
                  updateActiveID: _updateActiveID,
                  refreshAll: _refreshAll,
                  key: UniqueKey()),
            ),
            right: Container(
              decoration: BoxDecoration(
                border: Border(
                    left: BorderSide(color: ProjectColors.secondary(false))),
              ),
              height: double.infinity,
              child: MessageView(
                controlBar: ControlBar(
                  archive: _archive,
                  markImportant: _markImportant,
                  markDeleted: _markDeleted,
                  markUnread: _markUnread,
                  reply: _reply,
                  replyAll: _replyAll,
                  share: _share,
                ),
                message: _messages.length > _activeID
                    ? _messages[_activeID]
                    : MimeMessage(),
                key: UniqueKey(),
              ),
            ),
            ratio2: 0.25,
            minRatio2: 0.1,
            maxRatio2: 0.45,
            ratio1: 0.2,
            minRatio1: 0.1,
            maxRatio1: 0.25,
          ),
        ),
      ),
    );
  }
}


// [
//   "INBOX" exists: 3, highestModeSequence: 1589, flags: [MailboxFlag.hasNoChildren, MailboxFlag.inbox],
//   "[Gmail]" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasChildren, MailboxFlag.noSelect],
//   "[Gmail]/All Mail" exists: 0, highestModeSequence: null, flags: [MailboxFlag.all, MailboxFlag.hasNoChildren],
//   "[Gmail]/Drafts" exists: 0, highestModeSequence: null, flags: [MailboxFlag.drafts, MailboxFlag.hasNoChildren],
//   "[Gmail]/Important" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.flagged],
//   "[Gmail]/Sent Mail" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.sent],
//   "[Gmail]/Spam" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.junk],
//   "[Gmail]/Starred" exists: 0, highestModeSequence: null, flags: [MailboxFlag.flagged, MailboxFlag.hasNoChildren],
//   "[Gmail]/Trash" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.trash]
// ]
