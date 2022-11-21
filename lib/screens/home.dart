// import 'package:enough_mail/codecs.dart';
import 'package:flutter/material.dart';
// import 'package:enough_mail/enough_mail.dart';
import '../mail-client/enough_mail.dart';

import '../services/mail-service.dart';
import '../services/inbox-service.dart';
import '../widgets/vertical-split.dart';
import '../widgets/message-list.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  // final CustomMailClient _mailClient = CustomMailClient();
  final InboxService _inboxService = InboxService();
  final String email = 'test1928346534@gmail.com';
  final String password = 'xsccljyfbfrgvtjw';

  late List<MimeMessage> _messages = [];
  late int _activeID = 0;

  @override
  void initState() {
    super.initState();

    // _mailClient.connect(email, password);
    _inboxService.newClient(
        email, password, 'imap.gmail.com', 993, 'smtp.gmail.com', 993);
    // _updateMailList(0);
    updateInbox();
  }

  _updateActiveID(int idx) {
    setState(() {
      _activeID = idx;
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

  updateInbox() async {
    await _inboxService.clientsConnected();
    _inboxService.updateInbox();
  }

  updateMessageList(String email, String mailboxPath) async {
    _inboxService.updateLocalMailbox(email, mailboxPath);

    _setMessages();

    await _inboxService.updateMailList(email, mailboxPath);

    _setMessages();
  }

  Widget _makeMessage(int idx) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12, top: 8, left: 16, right: 16),
      child: SingleChildScrollView(
        child: Text(
          _messages.length > idx
              ? _messages[idx].decodeTextPlainPart() ?? ''
              : '',
          style: const TextStyle(color: Colors.white60),
        ),
      ),
    );
  }

  Widget _makeAccountsTree() {
    final accountsTree = _inboxService.accountsTree();
    List<Widget> accountsTreeWidgets = [];

    accountsTree.forEach((email, account) {
      for (var inboxInfo in account) {
        accountsTreeWidgets.add(GestureDetector(
          onTap: () => {
            updateMessageList(email, inboxInfo.path),
          },
          child: Padding(
            padding: inboxInfo.indent
                ? const EdgeInsets.only(left: 10)
                : EdgeInsets.zero,
            child: Text(
              inboxInfo.display,
              style: const TextStyle(color: Colors.white60),
              overflow: TextOverflow.clip,
              softWrap: false,
            ),
          ),
        ));
      }
    });

    return Padding(
      padding: const EdgeInsets.only(bottom: 10, top: 10, left: 10, right: 10),
      child: ListView(
        children: accountsTreeWidgets,
      ),
    );
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
              decoration: const BoxDecoration(
                border: Border(right: BorderSide(color: Colors.black)),
              ),
              height: double.infinity,
              child: FutureBuilder<List<bool>>(
                future: _inboxService.clientsConnected(),
                // future: _mailClient.connected(),
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.done:
                      return _makeAccountsTree();
                    default:
                      return Container();
                  }
                },
              ),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: MessageList(
                  messages: _messages,
                  activeID: _activeID,
                  updateActiveID: _updateActiveID,
                  key: UniqueKey()),
            ),
            right: Container(
              decoration: const BoxDecoration(
                border: Border(left: BorderSide(color: Colors.black)),
              ),
              height: double.infinity,
              child: FutureBuilder<List<bool>>(
                future: _inboxService.clientsConnected(),
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.done:
                      return _makeMessage(_activeID);
                    default:
                      return Container();
                  }
                },
              ),
            ),
            ratio2: 0.3,
            minRatio2: 0.1,
            maxRatio2: 0.45,
            ratio1: 0.15,
            minRatio1: 0.1,
            maxRatio1: 0.25,
          ),
        ),
      ),
    );
  }
}

void printMessage(message) {
  if (!message.isTextPlainMessage()) {
  } else {
    final plainText = message.decodeTextPlainPart();

    if (plainText != null) {
      final lines = plainText.split('\r\n');

      for (final line in lines) {
        if (line.startsWith('>')) {
          // break when quoted text starts
          break;
        }
      }
    }
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
