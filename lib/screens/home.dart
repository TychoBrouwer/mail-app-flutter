import 'package:flutter/material.dart';
// import 'package:enough_mail/enough_mail.dart';

import '../services/mail-client.dart';
import '../widgets/message-preview.dart';
import '../widgets/vertical-split.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late Future<bool> clientLoaded;
  late Future<bool> mailLoaded;

  final _mailClient = MailClient();

  @override
  void initState() {
    super.initState();

    clientLoaded = _loadClient();
  }

  int _shownMessageID = 0;

  Future<bool> _loadClient() async {
    final connect = _mailClient.connect();

    return connect;
  }

  // Widget _makeMailList(emails) {
  //   emails!.sort((a, b) => b
  //       .decodeDate()!
  //       .millisecondsSinceEpoch
  //       .compareTo(a.decodeDate()!.millisecondsSinceEpoch));

  //   return ListView.builder(
  //     itemBuilder: (ctx, idx) {
  //       // return _makeMailPreview(emails[idx], idx);
  //       return MailPreview(
  //           email: emails[idx],
  //           idx: idx,
  //           getActive: getActive,
  //           updateMessageID: updateMessageID);
  //     },
  //     itemCount: emails.length,
  //   );
  // }

  Future<Widget> _makeMailList() async {
    await clientLoaded;

    final mailboxes = _mailClient.getMailBoxes();

    mailLoaded = _mailClient.updateMailboxMessages(mailboxes[2]);

    await mailLoaded;

    final emails = _mailClient.getMessages(mailboxes[2]);

    emails.sort((a, b) => b
        .decodeDate()!
        .millisecondsSinceEpoch
        .compareTo(a.decodeDate()!.millisecondsSinceEpoch));

    return ListView.builder(
      itemBuilder: (ctx, idx) {
        // return _makeMailPreview(emails[idx], idx);
        return MailPreview(
            email: emails[idx],
            idx: idx,
            getActive: getActive,
            updateMessageID: updateMessageID);
      },
      itemCount: emails.length,
    );
  }

  updateMessageID(int idx) {
    setState(() {
      _shownMessageID = idx;
    });
  }

  getActive(idx) => _shownMessageID == idx;

  Widget _makeMessage(idx) {
    final email = _mailClient.getMessageFromIdx(idx);

    return Padding(
      padding: const EdgeInsets.only(bottom: 12, top: 8, left: 16, right: 16),
      child: Text(
        email.decodeTextPlainPart() ?? 'unable to get message contents',
        style: const TextStyle(color: Colors.white60),
      ),
    );
  }

  Widget _buildLoadingScreen() {
    return const Center(
      child: SizedBox(
        width: 50,
        height: 50,
        child: CircularProgressIndicator(),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // appBar: AppBar(
      //   title: Text(widget.title),
      // ),
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
              child: const Text('All Accounts'),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: FutureBuilder<Widget>(
                future: _makeMailList(),
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.done:
                      // _mailClient.getMailBoxes();
                      // return _makeMailList(_mailClient.getMessages());
                      // return snapshot.data!;
                      // print(snapshot.data);
                      return snapshot.data!;
                    default:
                      return _buildLoadingScreen();
                  }
                },
              ),
            ),
            right: Container(
              decoration: const BoxDecoration(
                border: Border(left: BorderSide(color: Colors.black)),
              ),
              height: double.infinity,
              // child: FutureBuilder<bool>(
              //   future: mailLoaded,
              //   builder: (context, snapshot) {
              //     switch (snapshot.connectionState) {
              //       case ConnectionState.done:
              //         return _makeMessage(_shownMessageID);
              //       default:
              //         return Container();
              //     }
              //   },
              // ),
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
//   "INBOX" exists: 0,
//   highestModeSequence: null,
//   flags: [MailboxFlag.hasNoChildren,
//   MailboxFlag.inbox
// ], 
//   "[Gmail]" exists: 0,
//   highestModeSequence: null,
//   flags: [
//     MailboxFlag.hasChildren,
//     MailboxFlag.noSelect
//   ],
//   "[Gmail]/All Mail" exists: 0,
//   highestModeSequence: null, 
//   flags: [
//     MailboxFlag.all, 
//     MailboxFlag.hasNoChildren
//   ],
//   "[Gmail]/Drafts" exists: 0,
//   highestModeSequence: null,
//   flags: [
//     MailboxFlag.drafts,
//     MailboxFlag.hasNoChildren
//   ], 
//   "[Gmail]/Important" exists: 0,
//   highestModeSequence: null, 
//   flags: [
//     MailboxFlag.hasNoChildren, 
//     MailboxFlag.flagged
//   ], 
//   "[Gmail]/Sent Mail" exists: 0,
//   highestModeSequence: null,
//   flags: [
//     MailboxFlag.hasNoChildren,
//     MailboxFlag.sent
//   ],
//   "[Gmail]/Spam" exists: 0, 
//   highestModeSequence: null, 
//   flags: [
//     MailboxFlag.hasNoChildren, 
//     MailboxFlag.juck
//   ],
//   "[Gmail]/Starred" exists: 0, 
//   highestModeSequence: null, 
//   flags: [
//     MailboxFlag.flagged, 
//     MailboxFlag.hasNoChildren
//   ], 
//   "[Gmail]/Trash" exists: 0, 
//   highestModeSequence: null, 
//   flags: [
//     MailboxFlag.hasNoChildren, 
//     MailboxFlag.trash
//   ]
// ]
