import 'dart:async';

import 'package:enough_mail/enough_mail.dart';
import 'package:mail_app/utils/wait-until.dart';

class CustomMailClient {
  final ImapClient _client = ImapClient(isLogEnabled: false);

  final Map<String, List<MimeMessage>> _messages = {};
  late List<Mailbox> _mailBoxes;
  late Mailbox _currentMailbox;

  bool _connected = false;

  List<MimeMessage> getMessages() {
    return _messages[_currentMailbox.encodedPath]!;
  }

  MimeMessage getMessageFromIdx(idx) {
    return _messages[_currentMailbox.encodedPath]?[idx] ?? MimeMessage();
  }

  Future<bool> connected() async {
    return await waitUntil(() => _connected);
  }

  Future<bool> connect(
      {String email = 'test1928346534@gmail.com',
      String password = 'xsccljyfbfrgvtjw'}) async {
    try {
      await _client.connectToServer('imap.gmail.com', 993, isSecure: true);
      await _client.login(email, password);
      _currentMailbox = await _client.selectInbox();
      await updateMailBoxes();

      _connected = true;
    } on ImapException catch (e) {
      print(e);
    }

    return _connected;
  }

  Future<bool> disconnect() async {
    try {
      await _client.logout();

      _connected = false;
    } on ImapException catch (e) {
      print(e);
    }

    return _connected;
  }

  Future<bool> selectMailbox(idx) async {
    try {
      _currentMailbox = await _client.selectMailbox(_mailBoxes[idx]);

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }

  Future<bool> updateMailboxMessages() async {
    try {
      await _client.selectMailbox(_currentMailbox);
      // fetch 10 most recent messages:
      final fetchResult = await _client.fetchRecentMessages(
          messageCount: 10, criteria: 'BODY[]');

      _messages[_currentMailbox.encodedPath] = fetchResult.messages;

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }

  Future<List<Mailbox>> updateMailBoxes() async {
    _mailBoxes = await _client.listMailboxes(recursive: true);

    return _mailBoxes;
  }

  List<Mailbox> getMailBoxes() {
    return _mailBoxes;
  }

  List<String> getMailBoxNames() {
    return _mailBoxes.map((box) => box.encodedPath).toList();
  }

  // Future<void> discoverExample() async {
  //   var email = 't.brouwer1@student.tue.nl';
  //   var config = await Discover.discover(email, isLogEnabled: false);
  //   if (config == null) {
  //   } else {
  //     for (var provider in config.emailProviders!) {}
  //   }
  // }

  // Future<bool> imapExample() async {
  //   try {
  //     await _client.selectInbox();
  //     // fetch 10 most recent messages:
  //     final fetchResult = await _client.fetchRecentMessages(
  //         messageCount: 100, criteria: 'BODY[]');

  //     _messages["INBOX"] = fetchResult.messages;

  //     return true;
  //   } on ImapException catch (e) {
  //     print(e);
  //   }

  //   return false;
  // }
}
