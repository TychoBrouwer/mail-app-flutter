import 'package:flutter/material.dart';
import 'package:enough_mail/enough_mail.dart';
import 'package:mail_app/widgets/message-preview.dart';

import '../widgets/vertical-split.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late Future<bool> mailLoaded;

  final _mailClient = MailClient();

  @override
  void initState() {
    super.initState();

    mailLoaded = _mailClient.imapExample();
  }

  int _shownMessageID = 0;

  Widget _makeMailList(List<MimeMessage>? emails) {
    emails!.sort((a, b) => b
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
    final email = _mailClient.getMessagesIdx(idx);

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
              child: FutureBuilder<bool>(
                future: mailLoaded,
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.done:
                      return _makeMailList(_mailClient.getMessages());
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
              child: FutureBuilder<bool>(
                future: mailLoaded,
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.done:
                      return _makeMessage(_shownMessageID);
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

class MailClient {
  late List<MimeMessage> _messages;

  List<MimeMessage> getMessages() {
    return _messages;
  }

  MimeMessage getMessagesIdx(idx) {
    return _messages[idx];
  }

  Future<void> discoverExample() async {
    var email = 't.brouwer1@student.tue.nl';
    var config = await Discover.discover(email, isLogEnabled: false);
    if (config == null) {
    } else {
      for (var provider in config.emailProviders!) {}
    }
  }

  void printMessage(MimeMessage message) {
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

  Future<bool> imapExample() async {
    final client = ImapClient(isLogEnabled: false);
    try {
      await client.connectToServer('imap.gmail.com', 993, isSecure: true);
      await client.login('test1928346534@gmail.com', 'xsccljyfbfrgvtjw');
      // final mailboxes = await client.listMailboxes();
      // print('mailboxes: $mailboxes');
      await client.selectInbox();
      // fetch 10 most recent messages:
      final fetchResult = await client.fetchRecentMessages(
          messageCount: 10, criteria: 'BODY[]');

      await client.logout();

      _messages = fetchResult.messages;

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }
}
